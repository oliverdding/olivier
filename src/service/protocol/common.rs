use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::error::ServiceError;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub reason: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ServiceError::Database(err) => match err {
                sea_orm::DbErr::ConnectionAcquire(source) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        reason: source.to_string(),
                    },
                )
                    .into_response(),
                sea_orm::DbErr::RecordNotFound(str) => {
                    (StatusCode::NOT_FOUND, ErrorResponse { reason: str }).into_response()
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        reason: "Database error".to_owned(),
                    },
                )
                    .into_response(),
            },
            ServiceError::UserNotFound(id) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    reason: format!("User {id} not found"),
                },
            )
                .into_response(),
        }
    }
}
