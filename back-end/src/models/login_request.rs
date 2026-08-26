use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}
