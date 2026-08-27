use serde::Serialize;

use crate::models::user_response::UserResponse;

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}
