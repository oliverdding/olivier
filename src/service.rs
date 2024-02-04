use anyhow::{Context, Result};
use axum::routing::post;
use axum::{routing::get, Router};
use http::header;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tokio_graceful_shutdown::SubsystemHandle;
use tower_http::decompression::DecompressionLayer;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer, trace,
};
use tracing::{debug, error, info, warn};

use crate::config;
use crate::routes;

impl config::Service {
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
        debug!("would connect {}", self.database.uri);

        let db: sea_orm::prelude::DatabaseConnection =
            Database::connect(&self.database.uri).await?;
        Migrator::up(&db, None).await?;

        let app = Router::new()
            .route(
                &format!("{}{}", &self.prefix, "api/v0/item"),
                post(routes::root),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/item"),
                get(routes::root),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/user"),
                post(routes::post_user),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/user/:id"),
                get(routes::get_user)
                    .put(routes::put_user)
                    .delete(routes::delete_user),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/maxitem"),
                get(routes::get_max_item),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/maxuser"),
                get(routes::get_max_user),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/topstories"),
                get(routes::root),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/newstories"),
                get(routes::root),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/topaskes"),
                get(routes::root),
            )
            .route(
                &format!("{}{}", &self.prefix, "api/v0/newaskes"),
                get(routes::root),
            )
            .with_state(db)
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                    .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO))
                    .on_failure(trace::DefaultOnFailure::new().level(tracing::Level::WARN)),
            )
            .route(
                &format!("{}{}", &self.prefix, "health"),
                get(routes::health).head(routes::health),
            )
            .layer(DecompressionLayer::new())
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
                "x-request-id",
            )))
            // TODO be more restrictive
            .layer(CorsLayer::permissive());

        let listener = tokio::net::TcpListener::bind(format!(
            "{host}:{port}",
            host = self.host,
            port = self.port
        ))
        .await
        .context("cannot bind to the address:port")?;

        info!("listening on {}", listener.local_addr()?);

        axum::serve(listener, app)
            .await
            .context("cannot start axum service at given address:port")?;

        Ok(())
    }
}
