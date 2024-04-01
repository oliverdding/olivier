use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServiceStatusResponse {
    pub database: bool,
}

impl IntoResponse for ServiceStatusResponse {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
