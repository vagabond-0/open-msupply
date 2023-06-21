use chrono::Utc;
use repository::{
    Document, DocumentFilter, DocumentRegistry, DocumentRegistryFilter, DocumentRegistryRepository,
    DocumentRegistryType, DocumentRepository, DocumentStatus, EqualFilter, Pagination,
    RepositoryError, StringFilter, TransactionError,
};

use crate::{
    document::{document_service::DocumentInsertError, is_latest_doc, raw_document::RawDocument},
    programs::patient::{main_patient_doc_name, patient_doc_name},
    service_provider::{ServiceContext, ServiceProvider},
};

use super::{
    program_enrolment_updated::update_program_enrolment_row, program_schema::SchemaProgramEnrolment,
};

#[derive(PartialEq, Debug)]
pub enum UpsertProgramEnrolmentError {
    NotAllowedToMutateDocument,
    InvalidPatientId,
    InvalidParentId,
    /// Each patient can only be enrolled in a program once
    ProgramExists,
    InvalidDataSchema(Vec<String>),
    DocumentTypeDoesNotExit,
    DataSchemaDoesNotExist,
    InternalError(String),
    DatabaseError(RepositoryError),
}

pub struct UpsertProgramEnrolment {
    pub patient_id: String,
    pub r#type: String,
    pub data: serde_json::Value,
    pub schema_id: String,
    /// If the patient is new the parent is not set
    pub parent: Option<String>,
}

pub fn upsert_program_enrolment(
    ctx: &ServiceContext,
    service_provider: &ServiceProvider,
    user_id: &str,
    input: UpsertProgramEnrolment,
    allowed_ctx: Vec<String>,
) -> Result<Document, UpsertProgramEnrolmentError> {
    let program_document = ctx
        .connection
        .transaction_sync(|_| {
            let patient_id = input.patient_id.clone();
            let (schema_program, registry) = validate(ctx, service_provider, &input)?;
            let doc = generate(user_id, input, registry)?;

            let document = service_provider
                .document_service
                .update_document(ctx, doc, &allowed_ctx)
                .map_err(|err| match err {
                    DocumentInsertError::NotAllowedToMutateDocument => {
                        UpsertProgramEnrolmentError::NotAllowedToMutateDocument
                    }
                    DocumentInsertError::InvalidDataSchema(err) => {
                        UpsertProgramEnrolmentError::InvalidDataSchema(err)
                    }
                    DocumentInsertError::DatabaseError(err) => {
                        UpsertProgramEnrolmentError::DatabaseError(err)
                    }
                    DocumentInsertError::InternalError(err) => {
                        UpsertProgramEnrolmentError::InternalError(err)
                    }
                    DocumentInsertError::DataSchemaDoesNotExist => {
                        UpsertProgramEnrolmentError::DataSchemaDoesNotExist
                    }
                    DocumentInsertError::InvalidParent(_) => {
                        UpsertProgramEnrolmentError::InvalidParentId
                    }
                })?;

            if is_latest_doc(ctx, service_provider, &document.name, document.datetime)
                .map_err(UpsertProgramEnrolmentError::DatabaseError)?
            {
                update_program_enrolment_row(
                    &ctx.connection,
                    &patient_id,
                    &document,
                    schema_program,
                )?;
            };
            Ok(document)
        })
        .map_err(|err: TransactionError<UpsertProgramEnrolmentError>| err.to_inner_error())?;
    Ok(program_document)
}

impl From<RepositoryError> for UpsertProgramEnrolmentError {
    fn from(err: RepositoryError) -> Self {
        UpsertProgramEnrolmentError::DatabaseError(err)
    }
}

fn generate(
    user_id: &str,
    input: UpsertProgramEnrolment,
    registry: DocumentRegistry,
) -> Result<RawDocument, RepositoryError> {
    Ok(RawDocument {
        name: patient_doc_name(&input.patient_id, &input.r#type),
        parents: input.parent.map(|p| vec![p]).unwrap_or(vec![]),
        author: user_id.to_string(),
        datetime: Utc::now(),
        r#type: input.r#type.clone(),
        data: input.data,
        form_schema_id: Some(input.schema_id),
        status: DocumentStatus::Active,
        owner_name_id: Some(input.patient_id),
        context: registry.document_context,
    })
}

fn validate_program_schema(
    input: &UpsertProgramEnrolment,
) -> Result<SchemaProgramEnrolment, serde_json::Error> {
    // Check that we can parse the data into a default Program object, i.e. that it's following the
    // default program JSON schema.
    // If the program data uses a derived program schema, the derived schema is validated in the
    // document service.
    serde_json::from_value(input.data.clone())
}

fn validate_patient_exists(
    ctx: &ServiceContext,
    patient_id: &str,
) -> Result<bool, RepositoryError> {
    let doc_name = main_patient_doc_name(patient_id);
    let document = DocumentRepository::new(&ctx.connection)
        .query(
            Pagination::one(),
            Some(DocumentFilter::new().name(StringFilter::equal_to(&doc_name))),
            None,
        )?
        .pop();
    Ok(document.is_some())
}

fn validate_program_not_exists(
    ctx: &ServiceContext,
    service_provider: &ServiceProvider,
    patient_id: &str,
    document_type: &str,
) -> Result<bool, RepositoryError> {
    let patient_name = patient_doc_name(patient_id, document_type);
    let existing_document = service_provider
        .document_service
        .document(ctx, &patient_name, None)?;
    Ok(existing_document.is_none())
}

fn validate_document_type(
    ctx: &ServiceContext,
    document_type: &str,
) -> Result<Option<DocumentRegistry>, RepositoryError> {
    let mut entry = DocumentRegistryRepository::new(&ctx.connection).query_by_filter(
        DocumentRegistryFilter::new()
            .r#type(DocumentRegistryType::ProgramEnrolment.equal_to())
            .document_type(EqualFilter::equal_to(document_type)),
    )?;
    Ok(entry.pop())
}

fn validate(
    ctx: &ServiceContext,
    service_provider: &ServiceProvider,
    input: &UpsertProgramEnrolment,
) -> Result<(SchemaProgramEnrolment, DocumentRegistry), UpsertProgramEnrolmentError> {
    if !validate_patient_exists(ctx, &input.patient_id)? {
        return Err(UpsertProgramEnrolmentError::InvalidPatientId);
    }

    let document_registry = match validate_document_type(ctx, &input.r#type)? {
        Some(document_registry) => document_registry,
        None => return Err(UpsertProgramEnrolmentError::DocumentTypeDoesNotExit),
    };

    let program = validate_program_schema(input).map_err(|err| {
        UpsertProgramEnrolmentError::InvalidDataSchema(vec![format!(
            "Invalid program data: {}",
            err
        )])
    })?;

    if input.parent.is_none() {
        if !validate_program_not_exists(ctx, service_provider, &input.patient_id, &input.r#type)? {
            return Err(UpsertProgramEnrolmentError::ProgramExists);
        }
    }

    Ok((program, document_registry))
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Timelike, Utc};
    use repository::{
        mock::{mock_form_schema_empty, MockDataInserts},
        test_db::setup_all,
        DocumentFilter, DocumentRegistryRow, DocumentRegistryRowRepository, DocumentRegistryType,
        DocumentRepository, FormSchemaRowRepository, Pagination, ProgramEnrolmentRepository,
        StringFilter,
    };
    use serde_json::json;
    use util::inline_init;

    use crate::{
        programs::{
            patient::{patient_doc_name, test::mock_patient_1, UpdatePatient, PATIENT_TYPE},
            program_enrolment::{program_schema::SchemaProgramEnrolment, UpsertProgramEnrolment},
        },
        service_provider::ServiceProvider,
    };

    use super::UpsertProgramEnrolmentError;

    #[actix_rt::test]
    async fn test_program_upsert() {
        let (_, _, connection_manager, _) = setup_all(
            "test_program_upsert",
            MockDataInserts::none()
                .names()
                .stores()
                .form_schemas()
                .name_store_joins(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "");
        let ctx = service_provider.basic_context().unwrap();

        // dummy schema
        let schema = mock_form_schema_empty();
        FormSchemaRowRepository::new(&ctx.connection)
            .upsert_one(&schema)
            .unwrap();
        let program_type = "ProgramType".to_string();
        let registry_repo = DocumentRegistryRowRepository::new(&ctx.connection);
        registry_repo
            .upsert_one(&DocumentRegistryRow {
                id: "patient_id".to_string(),
                r#type: DocumentRegistryType::Patient,
                document_type: PATIENT_TYPE.to_string(),
                document_context: "Patient".to_string(),
                name: None,
                parent_id: None,
                form_schema_id: Some(schema.id.clone()),
                config: None,
            })
            .unwrap();
        registry_repo
            .upsert_one(&DocumentRegistryRow {
                id: "program_enrolment_id".to_string(),
                r#type: DocumentRegistryType::ProgramEnrolment,
                document_type: program_type.to_string(),
                document_context: program_type.to_string(),
                name: None,
                parent_id: None,
                form_schema_id: Some(schema.id.clone()),
                config: None,
            })
            .unwrap();

        let service = &service_provider.program_enrolment_service;

        // NotAllowedToMutateDocument
        let err = service
            .upsert_program_enrolment(
                &ctx,
                &service_provider,
                "user",
                UpsertProgramEnrolment {
                    data: json!({"enrolment_datetime": true}),
                    schema_id: schema.id.clone(),
                    parent: None,
                    patient_id: "some_id".to_string(),
                    r#type: program_type.clone(),
                },
                vec!["WrongType".to_string()],
            )
            .err()
            .unwrap();
        matches!(err, UpsertProgramEnrolmentError::NotAllowedToMutateDocument);

        // InvalidPatientId
        let err = service
            .upsert_program_enrolment(
                &ctx,
                &service_provider,
                "user",
                UpsertProgramEnrolment {
                    data: json!({"enrolment_datetime": true}),
                    schema_id: schema.id.clone(),
                    parent: None,
                    patient_id: "some_id".to_string(),
                    r#type: program_type.clone(),
                },
                vec![program_type.clone()],
            )
            .err()
            .unwrap();
        matches!(err, UpsertProgramEnrolmentError::InvalidPatientId);

        let patient = mock_patient_1();
        service_provider
            .patient_service
            .upsert_patient(
                &ctx,
                &service_provider,
                "store_a",
                &patient.id,
                UpdatePatient {
                    data: serde_json::to_value(&patient).unwrap(),
                    schema_id: schema.id.clone(),
                    parent: None,
                },
            )
            .unwrap();
        // InvalidDataSchema
        let err = service
            .upsert_program_enrolment(
                &ctx,
                &service_provider,
                "user",
                UpsertProgramEnrolment {
                    data: json!({"enrolment_datetime": true}),
                    schema_id: schema.id.clone(),
                    parent: None,
                    patient_id: "some_id".to_string(),
                    r#type: program_type.clone(),
                },
                vec![program_type.clone()],
            )
            .err()
            .unwrap();
        matches!(err, UpsertProgramEnrolmentError::InvalidDataSchema(_));

        // success insert

        let program = inline_init(|v: &mut SchemaProgramEnrolment| {
            v.enrolment_datetime = Utc::now().with_nanosecond(0).unwrap().to_rfc3339();
            v.program_enrolment_id = Some("patient id 1".to_string());
        });

        service
            .upsert_program_enrolment(
                &ctx,
                &service_provider,
                "user",
                UpsertProgramEnrolment {
                    data: serde_json::to_value(program.clone()).unwrap(),
                    schema_id: schema.id.clone(),
                    parent: None,
                    patient_id: patient.id.clone(),
                    r#type: program_type.clone(),
                },
                vec![program_type.clone()],
            )
            .unwrap();

        assert_eq!(
            service
                .upsert_program_enrolment(
                    &ctx,
                    &service_provider,
                    "user",
                    UpsertProgramEnrolment {
                        patient_id: patient.id.clone(),
                        r#type: program_type.clone(),
                        data: serde_json::to_value(program.clone()).unwrap(),
                        schema_id: schema.id.clone(),
                        parent: None
                    },
                    vec![program_type.clone()]
                )
                .err()
                .unwrap(),
            UpsertProgramEnrolmentError::ProgramExists,
        );

        assert_eq!(
            service
                .upsert_program_enrolment(
                    &ctx,
                    &service_provider,
                    "user",
                    UpsertProgramEnrolment {
                        patient_id: patient.id.clone(),
                        r#type: program_type.clone(),
                        data: serde_json::to_value(program.clone()).unwrap(),
                        schema_id: schema.id.clone(),
                        parent: Some("invalid".to_string()),
                    },
                    vec![program_type.clone()]
                )
                .err()
                .unwrap(),
            UpsertProgramEnrolmentError::InvalidParentId
        );

        // success update
        let v0 = DocumentRepository::new(&ctx.connection)
            .query(
                Pagination::one(),
                Some(
                    DocumentFilter::new().name(StringFilter::equal_to(&patient_doc_name(
                        &patient.id,
                        &program_type,
                    ))),
                ),
                None,
            )
            .unwrap()
            .pop()
            .unwrap();
        service
            .upsert_program_enrolment(
                &ctx,
                &service_provider,
                "user",
                UpsertProgramEnrolment {
                    patient_id: patient.id.clone(),
                    r#type: program_type.clone(),
                    data: serde_json::to_value(program.clone()).unwrap(),
                    schema_id: schema.id.clone(),
                    parent: Some(v0.id),
                },
                vec![program_type.clone()],
            )
            .unwrap();
        // Test program has been written to the programs table
        let found_program = ProgramEnrolmentRepository::new(&ctx.connection)
            .find_one_by_type_and_patient(&program_type, &patient.id)
            .unwrap()
            .unwrap();
        assert_eq!(program_type, found_program.context);
        assert_eq!(
            program.enrolment_datetime,
            DateTime::<Utc>::from_utc(found_program.enrolment_datetime, Utc).to_rfc3339()
        );
        assert_eq!(
            program.program_enrolment_id,
            found_program.program_enrolment_id
        );
    }
}
