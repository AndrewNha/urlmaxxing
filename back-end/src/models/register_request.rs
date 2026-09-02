use serde::Deserialize;

use crate::{
    error::AppError,
    validation::{validate_password, validate_username},
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

impl RegisterRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        validate_username(&self.username)?;
        validate_password(&self.password)?;
        Ok(())
    }
}
