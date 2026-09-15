use super::repository;
use crate::{
    auth::{
        cookie::{build_auth_cookie, build_removal_cookie},
        jwt::generate_token,
    },
    error::AppError,
    models::{
        auth_user::AuthUser, login_request::LoginRequest, login_response::LoginResponse,
        user_response::UserResponse,
    },
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::verify;
use tower_cookies::Cookies;

pub async fn login(
    cookies: Cookies,
    State(state): State<AppState>,
    Json(mut req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.username = req.username.trim().to_lowercase();

    let user = repository::find_user_by_username(&state.pool, &req.username)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    let password_matches = verify(&req.password, user.password_hash())?;

    if !password_matches {
        return Err(AppError::InvalidCredentials);
    }

    let token_version = repository::find_token_version(&state.pool, *user.id())
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    let token = generate_token(*user.id(), token_version, &state.jwt_secret)
        .map_err(|_| AppError::TokenGenerationError)?;

    cookies.add(build_auth_cookie(token, state.cookie_secure));

    Ok(Json(LoginResponse {
        user: UserResponse::from(&user),
    }))
}

pub async fn logout(cookies: Cookies, State(state): State<AppState>) -> StatusCode {
    let cookie = build_removal_cookie(state.cookie_secure);

    cookies.remove(cookie);
    StatusCode::NO_CONTENT
}

pub async fn me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<UserResponse>, AppError> {
    let user = repository::find_user(&state.pool, *auth_user.user_id())
        .await?
        .ok_or(AppError::Unauthorized)?;

    Ok(Json(user))
}
