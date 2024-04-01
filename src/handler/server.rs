use axum::{extract::State, http::StatusCode, response::IntoResponse};
use tracing::error;

use crate::{dto::ServiceStatusResponse, server::AppState};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 204, description = "check service is up")
    )
)]
pub async fn health() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

#[utoipa::path(
    get,
    path = "/state",
    responses(
        (status = 200, description = "state of inner connection services", body = [ServiceStatusResponse])
    )
)]
pub async fn state(State(state): State<AppState>) -> impl IntoResponse {
    let database_status = state.database.ping().await;
    if let Err(e) = database_status.as_ref() {
        error!("Database connection failed with error: {e}.");
    }

    ServiceStatusResponse {
        database: database_status.is_ok(),
    }
}
