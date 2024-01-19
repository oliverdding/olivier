use std::io;

use config::ConfigError;
use thiserror::Error;

// Contains all error that generate by the application itself, which is useful to maintainer
#[derive(Error, Debug)]
pub enum OIError {
    #[error("configuration error")]
    Config(#[from] ConfigError),

    #[error("unable to load config because {0}")]
    LoadConfig(String),

    #[error("unable to run service")]
    Service(#[from] io::Error),

    #[error("database error")]
    Database(#[from] sea_orm::DbErr),
}

// Contains all error that generate by the service, which is useful to the user
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("database error")]
    Database(#[from] sea_orm::DbErr),

    #[error("cannot find user with id {0}")]
    UserNotFound(usize),
}
