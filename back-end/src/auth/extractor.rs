use crate::{
    auth::{jwt::validate_token, repository},
    error::AppError,
    models::auth_user::AuthUser,
    state::AppState,
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

        let validated_token =
            validate_token(token, &state.jwt_secret).map_err(|_| AppError::Unauthorized)?;

        let current_token_version =
            repository::find_token_version(&state.pool, validated_token.user_id)
                .await? // propaga erro do banco
                .ok_or(AppError::Unauthorized)?; //

        if validated_token.token_version != current_token_version {
            return Err(AppError::Unauthorized);
        }

        let auth_user = AuthUser::new(validated_token.user_id);

        Ok(auth_user)
    }
}
