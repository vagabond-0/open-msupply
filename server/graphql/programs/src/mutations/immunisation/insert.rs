use async_graphql::*;

use graphql_core::{
    simple_generic_errors::{DatabaseError, RecordAlreadyExist},
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::program_node::ProgramNode;
use service::{
    auth::{Resource, ResourceAccessRequest},
    program::insert_immunisation::{
        InsertImmunisationProgram, InsertImmunisationProgramError as ServiceError,
    },
};

#[derive(InputObject)]
pub struct InsertImmunisationProgramInput {
    pub id: String,
    pub name: String,
}

impl From<InsertImmunisationProgramInput> for InsertImmunisationProgram {
    fn from(input: InsertImmunisationProgramInput) -> Self {
        Self {
            id: input.id,
            name: input.name,
        }
    }
}

#[derive(SimpleObject)]
pub struct InsertImmunisationProgramError {
    pub error: InsertImmunisationProgramErrorInterface,
}

#[derive(Interface)]
#[graphql(field(name = "description", type = "String"))]
pub enum InsertImmunisationProgramErrorInterface {
    ProgramAlreadyExists(RecordAlreadyExist),
    DatabaseError(DatabaseError),
}

fn map_error(error: ServiceError) -> Result<InsertImmunisationProgramErrorInterface> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        // Standard Graphql Errors
        ServiceError::ImmunisationProgramAlreadyExists => BadUserInput(formatted_error),
        ServiceError::CreatedRecordNotFound => InternalError(formatted_error),
        ServiceError::DatabaseError(_) => InternalError(formatted_error),
    };

    Err(graphql_error.extend())
}

#[derive(Union)]
pub enum InsertImmunisationProgramResponse {
    Response(ProgramNode),
    Error(InsertImmunisationProgramError),
}

pub fn insert_immunisation_program(
    ctx: &Context<'_>,
    store_id: String,
    input: InsertImmunisationProgramInput,
) -> Result<InsertImmunisationProgramResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::MutateProgram,
            store_id: Some(store_id.clone()),
        },
    )?;
    let service_provider = ctx.service_provider();
    let service_context = service_provider.context(store_id.to_string(), user.user_id)?;

    match service_provider
        .program_service
        .insert_immunisation_program(&service_context, input.into())
    {
        Ok(row) => Ok(InsertImmunisationProgramResponse::Response(ProgramNode {
            program_row: row,
        })),
        Err(error) => Ok(InsertImmunisationProgramResponse::Error(
            InsertImmunisationProgramError {
                error: map_error(error)?,
            },
        )),
    }
}
