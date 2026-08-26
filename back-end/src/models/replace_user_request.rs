use serde::Deserialize;

/// PUT - todos os campos são obrigatórios.
#[derive(Deserialize)]
pub struct ReplaceUserRequest {
    pub display_name: String,
    pub username: String,
}
