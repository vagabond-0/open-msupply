use std::time::Duration;

use async_graphql::*;

use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use repository::{KeyValueStoreRepository, KeyValueType};
use reqwest::{Client, Url};
use service::auth::{Resource, ResourceAccessRequest};

use crate::queries::emd_settings::EmdSettingsNode;

#[derive(InputObject)]
pub struct EmdSettingsInput {
    ip: String,
    interval_seconds: i32,
}

pub async fn update_emd_settings(
    ctx: &Context<'_>,
    store_id: &str,
    input: EmdSettingsInput,
) -> Result<EmdSettingsNode> {
    validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::ServerAdmin,
            store_id: None,
        },
    )?;

    let service_provider = ctx.service_provider();
    let service_context = service_provider.basic_context()?;

    let response = Client::new()
        .get(Url::parse(&format!("http://{}", input.ip))?)
        .timeout(Duration::from_secs(5))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(StandardGraphqlError::from_str(&format!(
            "Response is 200 Ok: {}",
            response.status()
        )));
    }

    let text = response.text().await?;

    if !text.contains("Update mSupply cold chain info") {
        return Err(StandardGraphqlError::from_str(
            "Give ip does not respond with correct text",
        ));
    }

    let repo = KeyValueStoreRepository::new(&service_context.connection);

    repo.set_string(KeyValueType::EmdIP, Some(input.ip))?;
    repo.set_string(KeyValueType::EmdStoreId, Some(store_id.to_string()))?;
    repo.set_i32(
        KeyValueType::EmdIntervalSeconds,
        Some(input.interval_seconds),
    )?;

    Ok(EmdSettingsNode {
        ip: repo.get_string(KeyValueType::EmdIP)?.unwrap_or_default(),
        interval_seconds: repo
            .get_i32(KeyValueType::EmdIntervalSeconds)?
            .unwrap_or_default(),
    })
}
