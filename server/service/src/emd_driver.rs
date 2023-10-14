use std::{fs, io::Write, path::Path, sync::Arc};

use crate::service_provider::ServiceProvider;

use anyhow::Context;
use chrono::{Local, LocalResult, TimeZone, Utc};
use repository::{
    EqualFilter, KeyValueStoreRepository, KeyValueType, Pagination, RepositoryError, SensorFilter,
    SensorRepository, SensorRow, SensorRowRepository, Sort, TemperatureLogFilter,
    TemperatureLogRepository, TemperatureLogRow, TemperatureLogRowRepository,
    TemperatureLogSortField,
};
use reqwest::Client;
use temperature_sensor::{berlinger::read_sensor_from_file, Sensor, TemperatureLog};
use thiserror::Error;
use tokio::time::Duration;
use url::Url;
use util::uuid;

#[derive(Error, Debug)]
enum ProcessEmdError {
    #[error("Database error {0}")]
    DatabaseError(#[from] RepositoryError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
pub struct EmdDriver;

type EmdSettings = (Url, Duration, String);

async fn process_emd(
    service_provider: &ServiceProvider,
    (url, interval, store_id): EmdSettings,
    temp_dir: Option<String>,
) -> Result<(), ProcessEmdError> {
    let ctx = service_provider.basic_context()?;
    // Get data from emd
    let text = Client::new()
        .get(url.join("/usb-data").unwrap())
        .send()
        .await
        .context("Error while syncing emd data")?
        .text()
        .await
        .context("Cannot parse reponse text")?;

    // Parse Emd Data
    let mut lines = text.split("\n");
    let is_door_open = lines.next().context("First line does not exist")?.trim() == "true";
    let is_powered = lines.next().context("Second line does not exist")?.trim() == "true";

    let temperature = lines
        .next()
        .context("Third line does not exist")
        .map(|t| {
            log::info!("'{}'", t.trim());
            t.trim()
        })?
        .parse::<f64>()
        .context("Cannot parse temperature")?;

    log::info!("Data {} {} {}", is_door_open, is_powered, temperature);

    let now = Utc::now().naive_utc();
    // Check if we have more data:

    let (temperatures, serial) = if let (Some(first_fridgetag_line), Some(second_firdetage_line)) =
        (lines.next(), lines.next())
    {
        let temp_file_path = Path::new(&temp_dir.unwrap_or_default()).join("temp.txt");
        let temp_file_path = temp_file_path
            .to_str()
            .context("Problem converting path to string")?;

        // File will be closed when dropped (when this scope{..} finishes)
        {
            let mut file =
                fs::File::create(temp_file_path).context("Cannot open file for writing")?;
            file.write_all(first_fridgetag_line.as_bytes())
                .context("Cannot write line")?;
            file.write_all("\n".as_bytes())
                .context("Cannot write line")?;
            file.write_all(second_firdetage_line.as_bytes())
                .context("Cannot write line")?;
            file.write_all("\n".as_bytes())
                .context("Cannot write line")?;
            for line in lines {
                file.write_all(line.as_bytes())
                    .context("Cannot write line")?;
                file.write_all("\n".as_bytes())
                    .context("Cannot write line")?;
            }
        }

        let Sensor { logs, serial, .. } =
            read_sensor_from_file(temp_file_path).context("Could not parse berlinger data")?;

        fs::remove_file(temp_file_path).context("Could not remove temporary file")?;

        // Convert to utc naive date time
        let logs = logs
            .context("No temperature logs")?
            .into_iter()
            .map(
                |TemperatureLog {
                     timestamp,
                     temperature,
                 }| {
                    let local = match Local.from_local_datetime(&timestamp) {
                        LocalResult::None => {
                            return Err(anyhow::anyhow!("Cannot convert to local timestamp"))
                        }
                        LocalResult::Single(r) => r,
                        LocalResult::Ambiguous(r, _) => r,
                    };

                    Ok(TemperatureLog {
                        timestamp: local.naive_utc(),
                        temperature,
                    })
                },
            )
            .collect::<Result<_, _>>()?;

        (logs, serial)
    } else {
        (
            vec![TemperatureLog {
                temperature,
                timestamp: now,
            }],
            "STM".to_string(),
        )
    };

    // Encoded to store door open and is powered
    let sensor_temperature = match (is_door_open, is_powered) {
        (false, false) => 0.0,
        (true, false) => 10.0,
        (false, true) => 100.0,
        (true, true) => 110.0,
    };

    // Save Emd Data

    service_provider
        .basic_context()
        .context("Cannot get service context")?;

    let sensor_filter = SensorFilter::new().serial(EqualFilter::equal_to(&serial));

    let base_sensor = SensorRepository::new(&ctx.connection)
        .query_by_filter(sensor_filter.clone())?
        .pop()
        .map(|s| s.sensor_row)
        .unwrap_or(SensorRow {
            id: uuid::uuid(),
            name: serial.clone(),
            serial,
            is_active: true,
            battery_level: Some(100),
            ..Default::default()
        });

    SensorRowRepository::new(&ctx.connection).upsert_one(&SensorRow {
        store_id: Some(store_id.clone()),
        temperature: sensor_temperature,
        log_interval: Some(interval.as_secs() as i32),
        last_connection_datetime: Some(now),
        ..base_sensor.clone()
    })?;

    let latest_temperature_log = TemperatureLogRepository::new(&ctx.connection)
        .query(
            Pagination::one(),
            Some(TemperatureLogFilter::new().sensor(sensor_filter)),
            Some(Sort {
                key: TemperatureLogSortField::Datetime,
                desc: Some(true),
            }),
        )?
        .pop();

    log::info!("Latest log in db {:?}", latest_temperature_log);
    log::info!("Number of logs read {}", temperatures.len());

    let mut count = 0;
    for temperature in
        temperatures
            .into_iter()
            .filter(|log_to_integrate| match latest_temperature_log.as_ref() {
                Some(already_integrated_log) => {
                    already_integrated_log.temperature_log_row.datetime < log_to_integrate.timestamp
                }
                None => true,
            })
    {
        TemperatureLogRowRepository::new(&ctx.connection).upsert_one(&TemperatureLogRow {
            id: uuid::uuid(),
            temperature: temperature.temperature,
            sensor_id: base_sensor.id.clone(),
            location_id: base_sensor.location_id.clone(),
            store_id: Some(store_id.clone()),
            datetime: temperature.timestamp,
            temperature_breach_id: None,
        })?;

        count = count + 1;
    }

    log::info!("Number of logs integrated {}", count);

    Ok(())
}

impl EmdDriver {
    pub async fn run(service_provider: Arc<ServiceProvider>, temp_dir: Option<String>) {
        let default_interval = Duration::from_secs(15);

        loop {
            // Need to check is_initialsed from database on every iteration,
            // since it could have been updated

            let Some(emd_settings) = get_emd_settings(&service_provider) else {
                tokio::time::sleep(default_interval).await;
                continue;
            };

            let (_, interval, _) = &emd_settings;
            tokio::time::sleep(interval.to_owned()).await;

            match process_emd(&service_provider, emd_settings, temp_dir.clone()).await {
                Ok(()) => log::info!("Processed sensor data"),
                Err(error) => log::error!("Problem processing sensor data {}", error),
            }
        }
    }
}

fn get_emd_settings(service_provider: &ServiceProvider) -> Option<EmdSettings> {
    let ctx = service_provider.basic_context().unwrap();

    let repo = KeyValueStoreRepository::new(&ctx.connection);

    let Some(ip) =  repo.get_string(KeyValueType::EmdIP).unwrap() else {
        return None;
    };

    let Some(store_id) =  repo.get_string(KeyValueType::EmdStoreId).unwrap() else {
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
        store_id,
    ))
}
