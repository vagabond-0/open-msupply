use anyhow::Error;
use repository::{KeyValueStoreRepository, KeyValueType, RepositoryError};
use std::{fs, path::Path};

use crate::{service_provider::ServiceContext, settings::Level};

pub trait LogServiceTrait: Send + Sync {
    fn get_log_file_names(&self) -> Result<Vec<String>, Error> {
        let log_dir = Path::new("log");
        let mut log_file_names = Vec::new();

        for entry in fs::read_dir(log_dir)? {
            let path = entry?.path();
            log_file_names.push(path.file_name().unwrap().to_string_lossy().to_string());
        }

        Ok(log_file_names)
    }

    fn get_log_content(&self, file_name: Option<String>) -> Result<(String, Vec<String>), Error> {
        let log_dir = Path::new("log");
        let default_log_file = "remote_server.log".to_string();

        let file_name = match file_name {
            Some(file_name) => file_name,
            None => default_log_file,
        };

        let log_file_path = log_dir.join(&file_name);
        let log_file_content = fs::read_to_string(log_file_path)?;

        let log_file_content = log_file_content
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Ok((file_name, log_file_content))
    }

    fn log_level(&self, ctx: &ServiceContext) -> Result<Option<Level>, RepositoryError> {
        let key_value_store = KeyValueStoreRepository::new(&ctx.connection);

        let log_level = key_value_store.get_string(KeyValueType::LogLevel)?;

        let level = match log_level {
            Some(log_level) => match log_level.as_str() {
                "off" => Some(Level::Off),
                "error" => Some(Level::Error),
                "warn" => Some(Level::Warn),
                "info" => Some(Level::Info),
                "debug" => Some(Level::Debug),
                "trace" => Some(Level::Trace),
                _ => None,
            },
            None => None,
        };

        Ok(level)
    }

    fn upsert_log_level(
        &self,
        ctx: &ServiceContext,
        log_level: Level,
    ) -> Result<(), RepositoryError> {
        let key_value_store = KeyValueStoreRepository::new(&ctx.connection);

        let log_level = match log_level {
            Level::Off => "off",
            Level::Error => "error",
            Level::Warn => "warn",
            Level::Info => "info",
            Level::Debug => "debug",
            Level::Trace => "trace",
        };

        key_value_store.set_string(KeyValueType::LogLevel, Some(log_level.to_string()))?;

        Ok(())
    }
}

pub struct LogService {}

impl LogServiceTrait for LogService {}
