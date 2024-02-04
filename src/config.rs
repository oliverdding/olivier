use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::{env, net::IpAddr, path::PathBuf};

// default value <- global configuration file <- user configuration file <- environmen variables
#[derive(Debug, Clone, Deserialize)]
pub struct GlobalConfig {
    pub log: Log,
    pub service: Service,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Log {
    pub directives: String,
    pub file: LogFile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogFile {
    pub enabled: bool,
    pub path: String,
    pub directives: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub uri: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    pub host: IpAddr,
    pub port: u16,
    pub prefix: String,
    pub database: Database,
}

impl GlobalConfig {
    pub fn new() -> Result<GlobalConfig, ConfigError> {
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
}
