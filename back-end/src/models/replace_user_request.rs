use serde::Deserialize;

use crate::{error::AppError, validation::validate_username};

/// PUT - todos os campos são obrigatórios.
#[derive(Deserialize)]
pub struct ReplaceUserRequest {
    pub username: String,
}

impl ReplaceUserRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        validate_username(&self.username)?;
        Ok(())
    }
}
