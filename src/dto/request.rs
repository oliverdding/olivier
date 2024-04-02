use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NewTodoRequest {
    #[garde(length(min = 1, max = 1024))]
    pub body: String,
    #[garde(skip)]
    pub complated: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTodoRequest {
    #[garde(length(min = 1, max = 1024))]
    pub body: Option<String>,
    #[garde(skip)]
    pub complated: Option<bool>,
}

impl From<UpdateTodoRequest> for NewTodoRequest {
    fn from(value: UpdateTodoRequest) -> Self {
        Self {
            body: value.body.unwrap_or_default(),
            complated: value.complated,
        }
    }
}
