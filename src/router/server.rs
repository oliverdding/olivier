use axum::routing::get;

use crate::{handler::server, server::AppState};

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/health", get(server::health))
        .route("/state", get(server::state))
}
