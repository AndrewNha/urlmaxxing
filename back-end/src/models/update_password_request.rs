use serde::Deserialize;

use crate::{error::AppError, validation::validate_password};

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

impl UpdatePasswordRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        validate_password(&self.current_password)?;
        Ok(())
    }
}
