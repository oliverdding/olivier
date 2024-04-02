use anyhow::{Context, Result};
use migration::{Migrator, MigratorTrait};
use tokio_graceful_shutdown::SubsystemHandle;
use tracing::{error, info, warn};

use crate::{config::AppConfig, router};

use super::AppState;

pub struct AppServer {
    router: axum::routing::Router,
    listener: tokio::net::TcpListener,
}

impl AppServer {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let listener = tokio::net::TcpListener::bind(config.service.get_socket_addr()?).await?;

        let state = AppState::new(config).await?;

        info!("migration started");
        Migrator::up(&*state.database, None).await?;
        info!("migration done");

        let router = router::init(state);

        Ok(Self { router, listener })
    }

    pub async fn run(self, subsys: SubsystemHandle) -> Result<()> {
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
        let addr = self.listener.local_addr()?;
        info!("listening on {}", addr);

        axum::serve(self.listener, self.router)
            .await
            .context(format!("cannot start axum service at {}", addr))?;
        Ok(())
    }
}
