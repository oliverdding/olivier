use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetUserRequest {
    pub id: usize,
}

#[derive(Deserialize)]
pub struct PostUserRequest {
    pub name: String,
    pub about: String,
}
