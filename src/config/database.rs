use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub uri: String,
}
