use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HealthResponse {
    // teste resenha
    pub status: String,
}
