use crate::error::Result;
use crate::{
    error::ServiceError,
    protocol::Response,
};
use axum::{
    debug_handler,
    extract::State,
    response::IntoResponse,
};
use entity::item::Column as ItemColumn;
use entity::item::Entity as ItemEntity;
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryOrder, QuerySelect,
};

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
