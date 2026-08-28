use crate::{
    auth::jwt::validate_token, error::AppError, models::auth_user::AuthUser, state::AppState,
};
use axum::{extract::FromRequestParts, http::request::Parts};

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("Authorization")
            .ok_or(AppError::Unauthorized)?;
        let header = header.to_str().map_err(|_| AppError::Unauthorized)?;

        let token = header
            .strip_prefix("Bearer ") // remove o prefixo "Bearer " do token
            .ok_or(AppError::Unauthorized)?;

        let user_id =
            validate_token(token, &state.jwt_secret).map_err(|_| AppError::Unauthorized)?;

        let auth_user = AuthUser::new(user_id);

        Ok(auth_user)
    }
}
