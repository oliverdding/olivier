use axum::response::{IntoResponse, Response};
use chrono::NaiveDateTime as DateTime;
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

#[derive(Debug, Serialize, ToSchema)]
pub struct TodoResponse {
    pub id: i64,
    pub body: String,
    pub complated: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<entity::todos::Model> for TodoResponse {
    fn from(value: entity::todos::Model) -> Self {
        Self {
            id: value.id,
            body: value.body,
            complated: value.complated,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl IntoResponse for TodoResponse {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TodosResponse {
    pub todos: Vec<TodoResponse>,
}

impl IntoResponse for TodosResponse {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
