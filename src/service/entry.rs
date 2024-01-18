use anyhow::Result;
use axum::{routing::get, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tokio_graceful_shutdown::SubsystemHandle;
use tracing::{debug, info};

use crate::config;
use crate::service::routes::v0 as route;

impl config::Service {
    pub async fn run(self, subsys: SubsystemHandle) -> Result<()> {
        info!("service started");
        tokio::select! {
            _ = subsys.on_shutdown_requested() => {
                info!("the service is passively shut down");
            },
            res = self.serve() => {
                if let Err(err) = res {
                    info!("the service if actively terminated with error: {}", err);
                }else {
                    info!("the service is actively terminated");
                }
            }
        };
        info!("service stopped");

        Ok(())
    }

    async fn serve(self) -> Result<()> {
        debug!("would connect {}", self.database.uri);

        let db: sea_orm::prelude::DatabaseConnection =
            Database::connect(&self.database.uri).await?;
        Migrator::up(&db, None).await?;

        let app = Router::new()
            .route(
                &format!("{}{}", &self.prefix, "api/health"),
                get(route::root),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/item"),
                get(route::root).post(route::root),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/user"),
                get(route::get_user).post(route::post_user),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/maxitem"),
                get(route::root),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/topstories"),
                get(route::root),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/newstories"),
                get(route::root),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/topaskes"),
                get(route::root),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/newaskes"),
                get(route::root),
            )
            .with_state(db);

        let listener = tokio::net::TcpListener::bind(format!(
            "{host}:{port}",
            host = self.host,
            port = self.port
        ))
        .await
        .unwrap();

        info!("listening on {}", listener.local_addr().unwrap());

        axum::serve(listener, app).await.unwrap();

        Ok(())
    }
}
