use repository::{
    Document, DocumentRegistryFilter, DocumentRegistryRepository, DocumentRepository,
    EncounterFilter, EncounterRepository, EqualFilter, ProgramFilter, ProgramRepository,
    RepositoryError, StorageConnection,
};

use crate::programs::{
    encounter::{
        encounter_updated::update_encounter_row, validate_misc::validate_encounter_schema,
    },
    patient::{patient_schema::SchemaPatient, patient_updated::update_patient_row},
    program_enrolment::program_enrolment_updated::update_program_enrolment_row,
    program_enrolment::program_schema::SchemaProgramEnrolment,
    update_program_document::update_program_events,
};

pub(crate) fn sync_upsert_document(
    con: &StorageConnection,
    document: &Document,
) -> Result<(), RepositoryError> {
    // TODO comment why only insert here
    DocumentRepository::new(con).sync_insert(document)?;

    let Some(registry) = DocumentRegistryRepository::new(con)
        .query_by_filter(
            DocumentRegistryFilter::new().document_type(EqualFilter::equal_to(&document.r#type)),
        )?
        .pop() else {
        log::warn!("Received unknown document type: {}", document.r#type);
        return Ok(());
    };

    match registry.category {
        repository::DocumentRegistryCategory::Patient => update_patient(con, document)?,
        repository::DocumentRegistryCategory::ProgramEnrolment => {
            update_program_enrolment(con, document)?
        }
        repository::DocumentRegistryCategory::Encounter => update_encounter(con, document)?,
        repository::DocumentRegistryCategory::Custom => {}
    };
    Ok(())
}

fn update_patient(con: &StorageConnection, document: &Document) -> Result<(), RepositoryError> {
    let patient: SchemaPatient = serde_json::from_value(document.data.clone()).map_err(|err| {
        RepositoryError::as_db_error(&format!("Invalid patient data: {}", err), "")
    })?;

    update_patient_row(con, &document.datetime, patient, true)
        .map_err(|err| RepositoryError::as_db_error(&format!("{:?}", err), ""))?;
    Ok(())
}

fn update_program_enrolment(
    con: &StorageConnection,
    document: &Document,
) -> Result<(), RepositoryError> {
    let Some(patient_id) = &document.owner_name_id else {
        return Err(RepositoryError::as_db_error("Document owner id expected", ""));
    };
    let program_enrolment: SchemaProgramEnrolment = serde_json::from_value(document.data.clone())
        .map_err(|err| {
        RepositoryError::as_db_error(&format!("Invalid program enrolment data: {}", err), "")
    })?;
    let program_row = ProgramRepository::new(con)
        .query_one(ProgramFilter::new().context_id(EqualFilter::equal_to(&document.context_id)))?
        .ok_or(RepositoryError::as_db_error("Program row not found", ""))?;
    update_program_enrolment_row(con, patient_id, document, program_enrolment, program_row)
        .map_err(|err| RepositoryError::as_db_error(&format!("{:?}", err), ""))?;
    Ok(())
}

fn update_encounter(con: &StorageConnection, document: &Document) -> Result<(), RepositoryError> {
    let Some(patient_id) = &document.owner_name_id else {
        return Err(RepositoryError::as_db_error("Document owner id expected", ""));
    };

    let encounter: crate::programs::encounter::validate_misc::ValidatedSchemaEncounter =
        validate_encounter_schema(&document.data).map_err(|err| {
            RepositoryError::as_db_error(&format!("Invalid encounter data: {}", err), "")
        })?;
    let encounter_start_time = encounter.start_datetime;
    let existing_encounter = EncounterRepository::new(con)
        .query_by_filter(
            EncounterFilter::new().document_name(EqualFilter::equal_to(&document.name)),
        )?
        .pop();

    let clinician_id = encounter
        .encounter
        .clinician
        .as_ref()
        .and_then(|c| c.id.clone());
    let program_row = ProgramRepository::new(con)
        .query_one(ProgramFilter::new().context_id(EqualFilter::equal_to(&document.context_id)))?
        .ok_or(RepositoryError::as_db_error("Program row not found", ""))?;
    update_encounter_row(
        con,
        &patient_id,
        document,
        encounter,
        clinician_id,
        program_row,
    )
    .map_err(|err| RepositoryError::as_db_error(&format!("{:?}", err), ""))?;

    update_program_events(
        con,
        &patient_id,
        encounter_start_time,
        existing_encounter.map(|(existing, _)| existing.start_datetime),
        &document,
        None,
    )
    .map_err(|err| RepositoryError::as_db_error(&format!("{:?}", err), ""))?;
    Ok(())
}
