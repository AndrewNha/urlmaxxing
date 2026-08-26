use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateBookmarkRequest {
    pub title: String,
    pub url: String,
    pub tags: Vec<String>,
}
