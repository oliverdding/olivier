use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OIError {
    #[error("configuration error")]
    ConfigurationError(#[from] ConfigError),

    #[error("database error")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("{0}")]
    Supplementary(String),
}
