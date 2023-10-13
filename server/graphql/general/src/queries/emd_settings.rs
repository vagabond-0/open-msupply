use async_graphql::*;
use graphql_core::{standard_graphql_error::validate_auth, ContextExt};
use repository::{KeyValueStoreRepository, KeyValueType};
use service::auth::{Resource, ResourceAccessRequest};

#[derive(SimpleObject)]
pub struct EmdSettingsNode {
    pub ip: String,
    pub interval_seconds: i32,
}

pub(crate) fn emd_settings(ctx: &Context<'_>) -> Result<EmdSettingsNode> {
    validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
            store_id: None,
        },
    )?;

    let service_provider = ctx.service_provider();
    let service_context = service_provider.basic_context()?;

    let repo = KeyValueStoreRepository::new(&service_context.connection);

    Ok(EmdSettingsNode {
        ip: repo.get_string(KeyValueType::EmdIP)?.unwrap_or_default(),
        interval_seconds: repo
            .get_i32(KeyValueType::EmdIntervalSeconds)?
            .unwrap_or_default(),
    })
}
