use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    pub current_password: String,
}
