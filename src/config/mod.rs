mod database;
mod log;
mod service;

pub use database::*;
pub use log::*;
pub use service::*;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::{env, path::PathBuf};

// default value <- global configuration file <- user configuration file <- environment variables
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub log: Log,
    pub service: Service,
    pub database: Database,
}

pub fn new() -> Result<AppConfig, ConfigError> {
    let home_path: PathBuf = match env::var("OLIVIER_HOME") {
        Ok(v) => PathBuf::from(v),
        Err(_) => env::current_dir()
            .map_err(|err| ConfigError::Message(format!("cannot get current dir: {}", err)))?,
    };

    let config_builder = Config::builder()
        .add_source(File::from(home_path.join("config")).required(true))
        .add_source(File::from(home_path.join("custom")).required(false))
        .add_source(
            Environment::with_prefix("OLIVIER")
                .separator("_")
                .ignore_empty(true),
        );

    config_builder.build()?.try_deserialize()
}
