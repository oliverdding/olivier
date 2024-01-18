use axum::{
    debug_handler,
    extract::{Json, Query, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

use crate::service::protocol::v0::{ErrorResponse, GetUserRequest, PostUserRequest, Response};
use entity::user::Entity as UserEntity;
use entity::user::Model as UserModel;

#[debug_handler]
pub async fn get_user(
    State(db): State<DatabaseConnection>,
    Query(payload): Query<GetUserRequest>,
) -> Result<Response, (StatusCode, ErrorResponse)> {
    let user = UserEntity::find_by_id(payload.id as i64)
        .one(&db)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: -1,
                    reason: err.to_string(),
                },
            )
        })?;

    match user {
        Some(user) => Ok(Response::from(user)),
        None => Err((
            StatusCode::NOT_FOUND,
            ErrorResponse {
                code: -1,
                reason: format!("cannot find user with id {}", payload.id),
            },
        )),
    }
}

#[debug_handler]
pub async fn post_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<PostUserRequest>,
) -> Result<Response, (StatusCode, ErrorResponse)> {
    // FIXME: is this a correct usage of sea-orm?
    let mut user = entity::user::ActiveModel {
        name: ActiveValue::set(payload.name),
        about: ActiveValue::Set(payload.about),
        ..Default::default()
    };

    user.submitted = ActiveValue::Set(vec![]);

    let res: UserModel = user.insert(&db).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse {
                code: -1,
                reason: err.to_string(),
            },
        )
    })?;

    Ok(Response::from(res))
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
