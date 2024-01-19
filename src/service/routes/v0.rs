use crate::{
    error::ServiceError,
    service::protocol::v0::{PostUserRequest, Response},
};
use axum::{
    debug_handler,
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use entity::user::ActiveModel as UserActiveModel;
use entity::user::Entity as UserEntity;
use entity::user::Model as UserModel;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, Set};

#[debug_handler]
pub async fn get_user(
    State(db): State<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<i64>, ServiceError>,
) -> Result<impl IntoResponse, ServiceError> {
    let user = UserEntity::find_by_id(id)
        .one(&db)
        .await
        .map_err(ServiceError::Database)?;

    match user {
        Some(user) => Ok(Response::from(user)),
        None => Err(ServiceError::UserNotFound(id)),
    }
}

#[debug_handler]
pub async fn post_user(
    State(db): State<DatabaseConnection>,
    WithRejection(Json(payload), _): WithRejection<Json<PostUserRequest>, ServiceError>,
) -> Result<impl IntoResponse, ServiceError> {
    // FIXME: is this a correct usage of sea-orm?
    let mut user = entity::user::ActiveModel {
        name: ActiveValue::set(payload.name),
        about: ActiveValue::Set(payload.about),
        ..Default::default()
    };

    user.submitted = ActiveValue::Set(vec![]);

    let res: UserModel = user.insert(&db).await.map_err(ServiceError::Database)?;

    Ok((StatusCode::CREATED, Response::from(res)))
}

#[debug_handler]
pub async fn delete_user(
    State(db): State<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<i64>, ServiceError>,
) -> Result<impl IntoResponse, ServiceError> {
    let res = UserEntity::delete_by_id(id)
        .exec(&db)
        .await
        .map_err(ServiceError::Database)?;

    if res.rows_affected == 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::OK)
    }
}

#[debug_handler]
pub async fn put_user(
    State(db): State<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<i64>, ServiceError>,
    WithRejection(Json(payload), _): WithRejection<Json<PostUserRequest>, ServiceError>,
) -> Result<impl IntoResponse, ServiceError> {
    let user = UserEntity::find_by_id(id)
        .one(&db)
        .await
        .map_err(ServiceError::Database)?;

    let mut user: UserActiveModel = match user {
        Some(model) => model.into(),
        None => {
            let mut user = entity::user::ActiveModel {
                id: ActiveValue::set(id),
                name: ActiveValue::set(payload.name),
                about: ActiveValue::Set(payload.about),
                ..Default::default()
            };

            user.submitted = ActiveValue::Set(vec![]);

            let res: UserModel = user.insert(&db).await.map_err(ServiceError::Database)?;

            return Ok((StatusCode::CREATED, Response::from(res)));
        }
    };

    user.name = Set(payload.name);
    user.about = Set(payload.about);

    let res: UserModel = user.update(&db).await.map_err(ServiceError::Database)?;

    Ok((StatusCode::OK, Response::from(res)))
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
