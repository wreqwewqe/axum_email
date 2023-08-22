use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Bind {
    pub email: String,
    pub address: String,
    pub delay: u64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateContent {
    pub content: String,
}
