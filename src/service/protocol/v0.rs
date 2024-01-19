use axum::response::IntoResponse;
use chrono::NaiveDateTime as DateTime;
use entity::sea_orm_active_enums::Category;
use serde::{Deserialize, Serialize};

use entity::item::Model as Item;
use entity::user::Model as User;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Response {
    ItemStory(ItemStoryResponseData),
    ItemComment(ItemCommentResponseData),
    ItemAsk(ItemAskResponseData),
    User(UserResponse),
    List(Vec<i64>),
}

impl From<Item> for Response {
    fn from(value: Item) -> Self {
        match value.category {
            Category::Story => Response::ItemStory(ItemStoryResponseData {
                id: value.id,
                category: value.category,
                by: value.by,
                time: value.time,
                kids: value.kids,
                url: value.url,
                score: value.score,
                title: value.title,
                descendants: value.desendants,
            }),
            Category::Ask => Response::ItemAsk(ItemAskResponseData {
                id: value.id,
                category: value.category,
                by: value.by,
                time: value.time,
                text: value.text,
                kids: value.kids,
                score: value.score,
                title: value.title,
                descendants: value.desendants,
            }),
            Category::Comment => Response::ItemComment(ItemCommentResponseData {
                id: value.id,
                category: value.category,
                by: value.by,
                time: value.time,
                parent: value.parent,
                kids: value.kids,
                text: value.text,
            }),
        }
    }
}

impl From<User> for Response {
    fn from(value: User) -> Self {
        Response::User(UserResponse {
            id: value.id,
            name: value.name,
            created: value.created,
            about: value.about,
            submitted: value.submitted,
        })
    }
}

impl From<Vec<i64>> for Response {
    fn from(value: Vec<i64>) -> Self {
        Response::List(value)
    }
}

#[derive(Debug, Serialize)]
pub struct ItemStoryResponseData {
    pub id: i64,
    pub category: Category,
    pub by: i64,
    pub time: DateTime,
    pub kids: Vec<i64>,
    pub url: String,
    pub score: i32,
    pub title: String,
    pub descendants: i32,
}

#[derive(Debug, Serialize)]
pub struct ItemCommentResponseData {
    pub id: i64,
    pub category: Category,
    pub by: i64,
    pub time: DateTime,
    pub parent: i64,
    pub kids: Vec<i64>,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct ItemAskResponseData {
    pub id: i64,
    pub category: Category,
    pub by: i64,
    pub time: DateTime,
    pub text: String,
    pub kids: Vec<i64>,
    pub score: i32,
    pub title: String,
    pub descendants: i32,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub created: DateTime,
    pub about: String,
    pub submitted: Vec<i64>,
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

#[derive(Deserialize)]
pub struct GetUserRequest {
    pub id: usize,
}

#[derive(Deserialize)]
pub struct PostUserRequest {
    pub name: String,
    pub about: String,
}
