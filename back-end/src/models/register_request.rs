use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub display_name: String,
    pub username: String,
    pub password: String,
}
