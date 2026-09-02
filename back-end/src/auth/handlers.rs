use super::repository;
use crate::{
    auth::jwt::generate_token, error::AppError, models::login_request::LoginRequest,
    models::login_response::LoginResponse, models::user_response::UserResponse, state::AppState,
};
use axum::{Json, extract::State, response::IntoResponse};
use bcrypt::verify;

pub async fn login(
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

    let token = generate_token(*user.id(), &state.jwt_secret)
        .map_err(|_| AppError::TokenGenerationError)?;

    Ok(Json(LoginResponse {
        token,
        user: UserResponse::from(&user),
    }))
}
