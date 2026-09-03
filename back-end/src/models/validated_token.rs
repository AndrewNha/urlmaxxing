use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ValidatedToken {
    pub user_id: Uuid,
    pub token_version: i32,
}
