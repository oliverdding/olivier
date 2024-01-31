use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use tracing::info;

use crate::error::ServiceError;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
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
                        message: source.to_string(),
                    },
                )
                    .into_response(),
                sea_orm::DbErr::RecordNotFound(str) => {
                    (StatusCode::NOT_FOUND, ErrorResponse { message: str }).into_response()
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        message: "Database error".to_owned(),
                    },
                )
                    .into_response(),
            },
            ServiceError::UserNotFound(id) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    message: format!("User {id} not found"),
                },
            )
                .into_response(),
            ServiceError::PostNotFound(id) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    message: format!("Post {id} not found"),
                },
            )
                .into_response(),
            ServiceError::ItemEmpty => {
                info!("Item is empty");
                (
                    StatusCode::NO_CONTENT,
                    ErrorResponse {
                        message: "Cannot find item in database".to_owned(),
                    },
                )
                    .into_response()
            }
            ServiceError::JsonExtractorRejection(rejection) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse {
                    message: rejection.body_text(),
                },
            )
                .into_response(),
            ServiceError::QueryExtractorRejection(rejection) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    message: rejection.body_text(),
                },
            )
                .into_response(),
            ServiceError::PathExtractorRejection(rejection) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    message: rejection.body_text(),
                },
            )
                .into_response(),
        }
    }
}
