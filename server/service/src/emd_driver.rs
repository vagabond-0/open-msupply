use std::sync::Arc;

use crate::{service_provider::ServiceProvider, sync::ActiveStoresOnSite};

use chrono::Utc;
use repository::{
    KeyValueStoreRepository, KeyValueType, SensorRow, SensorRowRepository, TemperatureLogRow,
    TemperatureLogRowRepository,
};
use reqwest::Client;
use tokio::time::Duration;
use url::Url;
use util::uuid;

pub struct EmdDriver;

impl EmdDriver {
    pub async fn run(service_provider: Arc<ServiceProvider>) {
        let default_interval = Duration::from_secs(15);

        loop {
            // Need to check is_initialsed from database on every iteration, since it could have been updated

            let Some((url, interval)) = get_emd_settings(&service_provider) else {
                tokio::time::sleep(default_interval).await;
                continue;
            };

            tokio::time::sleep(interval).await;

            let result = Client::new()
                .get(url.join("/usb-data").unwrap())
                .send()
                .await;

            let response = match result {
                Ok(response) => response,
                Err(error) => {
                    log::error!("Error while syncing emd data {:?}", error);
                    continue;
                }
            };

            if !response.status().is_success() {
                log::error!("Emd response is not 200 ok: {}", response.status());
                continue;
            }

            let text = match response.text().await {
                Ok(text) => text,
                Err(error) => {
                    log::error!("Cannot parse reponse text {:?}", error);
                    continue;
                }
            };

            let mut lines = text.split("\n");
            let is_door_open_or_power_off = lines.next().unwrap_or("false") == "true";
            let temperature = match lines
                .next()
                .unwrap_or("will not parse to float")
                .parse::<f64>()
            {
                Ok(temperature) => temperature,
                Err(e) => {
                    log::error!("Cannot parse temperature {:?}", e);
                    continue;
                }
            };

            log::info!("Data {} {}", is_door_open_or_power_off, temperature);
            let ctx = service_provider.basic_context().unwrap();

            let now = Utc::now().naive_utc();
            let active_stores = match ActiveStoresOnSite::get(&ctx.connection) {
                Ok(stores) => stores,
                Err(e) => {
                    log::error!("Cannot get active stores on site {:?}", e);
                    continue;
                }
            };

            let senosor_id = "STM32".to_string();
            let store_id = active_stores.store_ids().pop();
            if let Err(e) = SensorRowRepository::new(&ctx.connection).upsert_one(&SensorRow {
                id: senosor_id.clone(),
                name: "Example Sensor".to_string(),
                serial: senosor_id.clone(),
                location_id: None,
                store_id: store_id.clone(),
                battery_level: Some(if is_door_open_or_power_off { 0 } else { 100 }),
                log_interval: Some(interval.as_secs() as i32),
                is_active: true,
                last_connection_datetime: Some(now),
            }) {
                log::error!("Cannot upsert sensor {:?}", e);
                continue;
            };

            if let Err(e) =
                TemperatureLogRowRepository::new(&ctx.connection).upsert_one(&TemperatureLogRow {
                    id: uuid::uuid(),
                    temperature,
                    sensor_id: senosor_id,
                    location_id: None,
                    store_id: store_id.clone(),
                    datetime: now,
                    temperature_breach_id: None,
                })
            {
                log::error!("Cannot upsert temperature log {:?}", e);
                continue;
            };
        }
    }
}

fn get_emd_settings(service_provider: &ServiceProvider) -> Option<(Url, Duration)> {
    let ctx = service_provider.basic_context().unwrap();

    let repo = KeyValueStoreRepository::new(&ctx.connection);

    let Some(ip) =  repo.get_string(KeyValueType::EmdIP).unwrap() else {
        return None;
    };

    let interval_seconds = repo
        .get_i32(KeyValueType::EmdIntervalSeconds)
        .unwrap()
        .unwrap_or_default();

    if interval_seconds == 0 {
        return None;
    };

    Some((
        Url::parse(&format!("http://{}", ip)).unwrap(),
        Duration::from_secs(interval_seconds as u64),
    ))
}
