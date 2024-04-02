use crate::config::AppConfig;
use anyhow::{Ok, Result};
use sea_orm::Database;
use tracing::info;

use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<crate::config::AppConfig>,
    pub database: Arc<sea_orm::DatabaseConnection>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self> {
        info!("connecting to database");
        let database = Arc::new(Database::connect(&config.database.uri).await?);

        Ok(Self {
            config: Arc::new(config),
            database,
        })
    }
}
