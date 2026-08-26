use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow, Debug)]
pub struct Bookmark {
    id: Uuid,
    user_id: Uuid,
    title: String,
    url: String,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
}

impl Bookmark {
    pub fn new(user_id: Uuid, title: String, url: String, tags: Vec<String>) -> Self {
        Self {
            user_id,
            id: Uuid::new_v4(),
            title,
            url,
            tags,
            created_at: Utc::now(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn tags(&self) -> &[String] {
        &self.tags
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}
