use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateBookmarkRequest {
    // não é obrigatório atualizar todos os campos
    pub title: Option<String>,
    pub url: Option<String>,
    pub tags: Option<Vec<String>>,
}
