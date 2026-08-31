use serde::Deserialize;

/// PUT - todos os campos são obrigatórios.
#[derive(Deserialize)]
pub struct ReplaceUserRequest {
    pub username: String,
}
