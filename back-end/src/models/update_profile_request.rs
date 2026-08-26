use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    // não é obrigatório atualizar todos os campos
    pub display_name: Option<String>,
    pub username: Option<String>,
}
