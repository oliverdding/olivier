use anyhow::{Context, Result};
use migration::{Migrator, MigratorTrait};
use tokio_graceful_shutdown::SubsystemHandle;
use tracing::{error, info, warn};
use utoipa::openapi::info;

use crate::{config::AppConfig, router};

use super::AppState;

pub struct AppServer {
    pub state: AppState,
    listener: tokio::net::TcpListener,
}

impl AppServer {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let listener = tokio::net::TcpListener::bind(config.service.get_socket_addr()?).await?;

        let addr = listener.local_addr()?;
        info!("listening on {}", addr);

        let state = AppState::new(config).await?;
        Ok(Self { state, listener })
    }

    pub async fn run(self, subsys: SubsystemHandle) -> Result<()> {
        info!("migration started");
        Migrator::up(&*self.state.database, None).await?;
        info!("migration done");

        info!("service started");
        tokio::select! {
            _ = subsys.on_shutdown_requested() => {
                warn!("the service is passively shut down");
            },
            res = self.serve() => {
                if let Err(err) = res {
                    error!("the service if actively terminated with error: {}", err);
                }else {
                    info!("the service is actively terminated");
                }
            }
        };
        info!("service stopped");

        Ok(())
    }

    async fn serve(self) -> Result<()> {
        let router = router::init(self.state);
        axum::serve(self.listener, router)
            .await
            .context("cannot start axum service at given address:port")?;
        Ok(())
    }
}
