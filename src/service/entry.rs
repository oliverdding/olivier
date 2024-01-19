use std::time::Duration;

use anyhow::Result;
use axum::{extract::MatchedPath, http::Request, response::Response, routing::get, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tokio_graceful_shutdown::SubsystemHandle;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{debug, error, info, info_span, warn, Span};

use crate::config;
use crate::service::routes::v0 as route;

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
            .with_state(db)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<_>| {
                        let matched_path = request
                            .extensions()
                            .get::<MatchedPath>()
                            .map(MatchedPath::as_str);

                        info_span!(
                            "http_request",
                            method = ?request.method(),
                            matched_path,
                        )
                    })
                    .on_request(|_request: &Request<_>, _span: &Span| debug!("request received"))
                    .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                        _span.record("status_code", &tracing::field::display(_response.status()));
                    })
                    .on_failure(
                        |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                            warn!("request failed with error: {}", _error);
                        },
                    ),
            )
            .route(&format!("{}{}", &self.prefix, "health"), get(route::root));

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
