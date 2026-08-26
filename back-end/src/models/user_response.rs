use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Serialize, FromRow)]
pub struct UserResponse {
    id: Uuid,
    display_name: String,
    username: String,
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            id: *user.id(),
            display_name: user.display_name().to_string(),
            username: user.username().to_string(),
        }
    }
}
