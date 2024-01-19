use axum::{
    debug_handler,
    extract::{Json, Query, State},
};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

use crate::{
    error::ServiceError,
    service::protocol::v0::{GetUserRequest, PostUserRequest, Response},
};
use entity::user::Entity as UserEntity;
use entity::user::Model as UserModel;

#[debug_handler]
pub async fn get_user(
    State(db): State<DatabaseConnection>,
    Query(payload): Query<GetUserRequest>,
) -> Result<Response, ServiceError> {
    let user = UserEntity::find_by_id(payload.id as i64)
        .one(&db)
        .await
        .map_err(ServiceError::Database)?;

    match user {
        Some(user) => Ok(Response::from(user)),
        None => Err(ServiceError::UserNotFound(payload.id)),
    }
}

#[debug_handler]
pub async fn post_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<PostUserRequest>,
) -> Result<Response, ServiceError> {
    // FIXME: is this a correct usage of sea-orm?
    let mut user = entity::user::ActiveModel {
        name: ActiveValue::set(payload.name),
        about: ActiveValue::Set(payload.about),
        ..Default::default()
    };

    user.submitted = ActiveValue::Set(vec![]);

    let res: UserModel = user.insert(&db).await.map_err(ServiceError::Database)?;

    Ok(Response::from(res))
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
