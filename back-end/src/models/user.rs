use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct User {
    id: Uuid,
    display_name: String,
    username: String,
    #[serde(skip_serializing)]
    password_hash: String,
}

impl User {
    pub fn new(display_name: String, username: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            display_name,
            username,
            password_hash,
        }
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
}
