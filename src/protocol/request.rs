use std::str::FromStr;

use entity::sea_orm_active_enums::Category;
use serde::Deserialize;

use crate::error::{Result, ServiceError};

pub trait Validate {
    async fn validate(&self) -> Result<()>;
}

#[derive(Deserialize)]
pub struct PostUserRequest {
    pub name: String,
    pub about: String,
}

impl Validate for PostUserRequest {
    async fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(ServiceError::Validation(
                "name could not be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct PostItemRequest {
    pub category: String,
    pub by: i64,
    pub text: Option<String>,
    pub parent: Option<i64>,
    pub url: Option<String>,
    pub title: Option<String>,
}

impl Validate for PostItemRequest {
    async fn validate(&self) -> Result<()> {
        let category = Category::from_str(&self.category).map_err(|e| ServiceError::Validation(e.to_string()))?;

        match category {
            Category::Ask => {
                if !self.text.as_ref().is_some_and(|s| !s.is_empty()) {
                    return Err(ServiceError::Validation(
                        "ask's text must exist and not empty".to_string(),
                    ));
                }
                if !self.title.as_ref().is_some_and(|url| !url.is_empty()) {
                    return Err(ServiceError::Validation(
                        "ask's title must exist and not empty".to_string(),
                    ));
                }
            }
            Category::Comment => {
                if !self.text.as_ref().is_some_and(|s| !s.is_empty()) {
                    return Err(ServiceError::Validation(
                        "comment's text must exist and not empty".to_string(),
                    ));
                }
                if !self.parent.is_some_and(|_s| true) {
                    // TODO: check if parent exists
                    return Err(ServiceError::Validation(
                        "comment's parent must exist and not empty".to_string(),
                    ));
                }
            }
            Category::Story => {
                if !self.url.as_ref().is_some_and(|url| !url.is_empty()) {
                    return Err(ServiceError::Validation(
                        "story's url must exist and not empty".to_string(),
                    ));
                }
                if !self.title.as_ref().is_some_and(|url| !url.is_empty()) {
                    return Err(ServiceError::Validation(
                        "story's title must exist and not empty".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}
