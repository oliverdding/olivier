use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use utoipa::ToSchema;

use crate::dto::ErrorResponse;

pub type AppResult<T, E = ServiceError> = core::result::Result<T, E>;

// Contains all error that generate by the service, which is useful to the user
#[derive(Error, Debug, ToSchema)]
#[error("...")]
pub enum ServiceError {
    #[error("{0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("{0}")]
    InvalidInputError(#[from] garde::Report),

    #[error("cannot find todo with id {0}")]
    TodoNotFoundError(i64),

    #[error("{0}")]
    JsonExtractorRejection(#[from] JsonRejection),

    #[error("{0}")]
    QueryExtractorRejection(#[from] QueryRejection),

    #[error("{0}")]
    PathExtractorRejection(#[from] PathRejection),
}

impl ServiceError {
    fn get_status_code(&self) -> StatusCode {
        match self {
            // 4xx
            ServiceError::QueryExtractorRejection(_) => StatusCode::BAD_REQUEST,
            ServiceError::PathExtractorRejection(_) => StatusCode::BAD_REQUEST,
            ServiceError::JsonExtractorRejection(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ServiceError::InvalidInputError(_) => StatusCode::BAD_REQUEST,
            ServiceError::TodoNotFoundError(_) => StatusCode::NOT_FOUND,
            // 5xx
            ServiceError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn get_internal_code(&self) -> i32 {
        match self {
            // 4xx
            ServiceError::InvalidInputError(_) => 40000,
            ServiceError::QueryExtractorRejection(err) => match err {
                QueryRejection::FailedToDeserializeQueryString(_) => 40100,
                _ => todo!(),
            },
            ServiceError::PathExtractorRejection(err) => match err {
                PathRejection::FailedToDeserializePathParams(_) => 40200,
                PathRejection::MissingPathParams(_) => 40201,
                _ => todo!(),
            },
            ServiceError::JsonExtractorRejection(err) => match err {
                JsonRejection::JsonDataError(_) => 40300,
                JsonRejection::JsonSyntaxError(_) => 40301,
                JsonRejection::MissingJsonContentType(_) => 40302,
                JsonRejection::BytesRejection(_) => 40303,
                _ => todo!(),
            },
            ServiceError::TodoNotFoundError(_) => 40400,

            // 5xx
            ServiceError::Database(err) => match err {
                sea_orm::DbErr::ConnectionAcquire(_) => 50100,
                sea_orm::DbErr::TryIntoErr {
                    from: _,
                    into: _,
                    source: _,
                } => 50004,
                sea_orm::DbErr::Conn(_) => 50101,
                sea_orm::DbErr::Exec(_) => 50102,
                sea_orm::DbErr::Query(_) => 50103,
                sea_orm::DbErr::ConvertFromU64(_) => 50104,
                sea_orm::DbErr::UnpackInsertId => 50105,
                sea_orm::DbErr::UpdateGetPrimaryKey => 50106,
                sea_orm::DbErr::RecordNotFound(_) => 50107,
                sea_orm::DbErr::AttrNotSet(_) => 50108,
                sea_orm::DbErr::Custom(_) => 50109,
                sea_orm::DbErr::Type(_) => 50110,
                sea_orm::DbErr::Json(_) => 50111,
                sea_orm::DbErr::Migration(_) => 50112,
                sea_orm::DbErr::RecordNotInserted => 50113,
                sea_orm::DbErr::RecordNotUpdated => 50114,
            },
        }
    }

    fn get_prompt_message(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        (
            self.get_status_code(),
            ErrorResponse {
                code: self.get_internal_code(),
                message: self.get_prompt_message(),
            },
        )
            .into_response()
    }
}
