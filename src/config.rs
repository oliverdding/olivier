use anyhow::Result;
use config::{Config, Environment, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{env, net::IpAddr, path::PathBuf};

use crate::error::OIError;

static QUALIFIER: &str = "com.github.oliverdding";
static ORGANIZATION: &str = "Oliver Ding";
static APPLICATION: &str = "oi";

// default value <- global configuration file <- user configuration file <- environmen variables
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub log: Log,
    pub service: Service,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub level: String,
    pub file: LogFile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogFile {
    pub enabled: bool,
    pub path: String,
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub host: IpAddr,
    pub port: u16,
    pub prefix: String,
    pub database: Database,
}

impl GlobalConfig {
    pub async fn new() -> Result<GlobalConfig> {
        let project_dir = match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            Some(v) => v,
            None => {
                return Err(OIError::Supplementary(
                    "error getting configurations path following XDG base directory".to_owned(),
                )
                .into());
            }
        };

        let home_path: PathBuf = match env::var("OI_HOME") {
            Ok(v) => PathBuf::from(v),
            Err(_) => project_dir.config_dir().to_path_buf(),
        };

        let config_builder = Config::builder()
            .set_default("log.level", "info")
            .map_err(OIError::ConfigurationError)?
            .set_default("log.file.enabled", false)
            .map_err(OIError::ConfigurationError)?
            .set_default(
                "log.file.path",
                project_dir
                    .cache_dir()
                    .to_str()
                    .ok_or(OIError::Supplementary(
                        "error getting cache path".to_owned(),
                    ))?,
            )
            .map_err(OIError::ConfigurationError)?
            .set_default("log.file.level", "info")
            .map_err(OIError::ConfigurationError)?
            .set_default("service.host", "0.0.0.0")
            .map_err(OIError::ConfigurationError)?
            .set_default("service.port", 3000)
            .map_err(OIError::ConfigurationError)?
            .set_default(
                "service.database.uri",
                "postgres://postgres:postgres@127.0.0.1:5432/postgres",
            )
            .map_err(OIError::ConfigurationError)?
            .set_default("service.prefix", "/")
            .map_err(OIError::ConfigurationError)?
            .add_source(File::with_name("/etc/olivier/config").required(false))
            .add_source(File::from(home_path.join("config")).required(false))
            .add_source(
                File::from(
                    env::current_dir()
                        .map_err(|err| OIError::Supplementary(err.to_string()))?
                        .join("config"),
                )
                .required(false),
            )
            .add_source(
                Environment::with_prefix("OI")
                    .separator("_")
                    .ignore_empty(true),
            );

        Ok(config_builder
            .build()
            .map_err(OIError::ConfigurationError)?
            .try_deserialize()
            .map_err(OIError::ConfigurationError)?)
    }
}
