use crate::{
    auth::{cookie::AUTH_COOKIE_NAME, jwt::validate_token, repository},
    error::AppError,
    models::auth_user::AuthUser,
    state::AppState,
};
use axum::{extract::FromRequestParts, http::request::Parts};
use tower_cookies::Cookies;

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Unauthorized)?;

        let cookie = cookies
            .get(AUTH_COOKIE_NAME)
            .ok_or(AppError::Unauthorized)?;

        let token = cookie.value();

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
