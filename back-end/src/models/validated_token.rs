use uuid::Uuid;

pub struct ValidatedToken {
    pub user_id: Uuid,
    pub token_version: i32,
}
