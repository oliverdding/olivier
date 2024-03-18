use std::str::FromStr;

use crate::error::Result;
use crate::protocol::{PostItemRequest, Validate};
use crate::{error::ServiceError, protocol::Response};
use axum::extract::Path;
use axum::Json;
use axum::{debug_handler, extract::State, response::IntoResponse};
use axum_extra::extract::WithRejection;
use entity::item::ActiveModel as ItemActiveModel;
use entity::item::Column as ItemColumn;
use entity::item::Entity as ItemEntity;
use entity::item::Model as ItemModel;
use entity::sea_orm_active_enums::Category;
use http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect,
};

#[debug_handler]
pub async fn get_item(
    State(db): State<DatabaseConnection>,
    WithRejection(Path(id), _): WithRejection<Path<i64>, ServiceError>,
) -> Result<impl IntoResponse> {
    let item = ItemEntity::find_by_id(id)
        .one(&db)
        .await
        .map_err(ServiceError::Database)?;

    match item {
        Some(item) => Ok(Response::from(item)),
        None => Err(ServiceError::ItemNotFound(id)),
    }
}

#[debug_handler]
pub async fn post_item(
    State(db): State<DatabaseConnection>,
    WithRejection(Json(payload), _): WithRejection<Json<PostItemRequest>, ServiceError>,
) -> Result<impl IntoResponse> {
    payload.validate().await?;

    let category = Category::from_str(&payload.category).map_err(|err| ServiceError::Validation(err.to_string()))?;

    // FIXME: is this a correct usage of sea-orm?
    let mut item = match category {
        entity::sea_orm_active_enums::Category::Ask => entity::item::ActiveModel {
            deleted: ActiveValue::set(false),
            dead: ActiveValue::set(false),
            parent: ActiveValue::set(0),
            url: ActiveValue::set("".to_string()),
            score: ActiveValue::set(1),
            category: ActiveValue::set(category),
            by: ActiveValue::set(payload.by),
            text: ActiveValue::set(payload.text.unwrap()),
            title: ActiveValue::set(payload.title.unwrap()),
            ..Default::default()
        },
        entity::sea_orm_active_enums::Category::Comment => entity::item::ActiveModel {
            deleted: ActiveValue::set(false),
            dead: ActiveValue::set(false),
            url: ActiveValue::set("".to_string()),
            score: ActiveValue::set(1),
            category: ActiveValue::set(category),
            by: ActiveValue::set(payload.by),
            text: ActiveValue::set(payload.text.unwrap()),
            parent: ActiveValue::set(payload.parent.unwrap()),
            ..Default::default()
        },
        entity::sea_orm_active_enums::Category::Story => entity::item::ActiveModel {
            deleted: ActiveValue::set(false),
            dead: ActiveValue::set(false),
            parent: ActiveValue::set(0),
            score: ActiveValue::set(1),
            category: ActiveValue::set(category),
            by: ActiveValue::set(payload.by),
            url: ActiveValue::set(payload.url.unwrap()),
            title: ActiveValue::set(payload.title.unwrap()),
            ..Default::default()
        },
    };

    item.kids = ActiveValue::Set(vec![]);

    let res: ItemModel = item.insert(&db).await.map_err(ServiceError::Database)?;

    Ok((StatusCode::CREATED, Response::from(res)))
}

#[debug_handler]
pub async fn get_max_item(State(db): State<DatabaseConnection>) -> Result<impl IntoResponse> {
    let item = ItemEntity::find()
        .order_by_desc(ItemColumn::Id)
        .limit(1)
        .one(&db)
        .await
        .map_err(ServiceError::Database)?;

    match item {
        Some(item) => Ok(Response::from(item)),
        None => Err(ServiceError::ItemEmpty),
    }
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
