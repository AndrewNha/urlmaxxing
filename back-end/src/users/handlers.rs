use std::ptr::read;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use bcrypt::{hash, verify};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{
        auth_user::AuthUser, register_request::RegisterRequest,
        replace_user_request::ReplaceUserRequest, update_password_request::UpdatePasswordRequest,
        user::User, user_response::UserResponse,
    },
    state::AppState,
};

use super::repository;

fn hash_password(password: &str, cost: u32) -> Result<String, bcrypt::BcryptError> {
    hash(password, cost)
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    if id != *auth_user.user_id() {
        return Err(AppError::Unauthorized);
    }

    let user = repository::find_user(&state.pool, &id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(user))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(mut req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.username = req.username.trim().to_lowercase();
    req.validate()?;

    let password_hash = hash_password(&req.password, bcrypt::DEFAULT_COST)?;
    let user = User::new(req.username, password_hash);

    repository::insert_user(&state.pool, &user).await?;

    Ok((StatusCode::CREATED, Json(UserResponse::from(&user))))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    if user_id != *auth_user.user_id() {
        return Err(AppError::Unauthorized);
    }

    let user = repository::remove_user(&state.pool, &user_id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(user))
}

pub async fn replace_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    auth_user: AuthUser,
    Json(mut req): Json<ReplaceUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    if user_id != *auth_user.user_id() {
        return Err(AppError::Unauthorized);
    }
    req.username = req.username.trim().to_lowercase();
    req.validate()?;

    let user = repository::replace_user(&state.pool, &user_id, &req)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(user))
}

pub async fn update_password(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    auth_user: AuthUser,
    Json(req): Json<UpdatePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    if user_id != *auth_user.user_id() {
        return Err(AppError::Unauthorized);
    }
    req.validate()?;
    if req.current_password == req.new_password {
        return Err(AppError::ValidationError(
            "Current password and new password must be different".to_string(),
        ));
    }

    let current_user = repository::find_user_with_password(&state.pool, auth_user.user_id())
        .await?
        .ok_or(AppError::NotFound)?;

    let password_matches = verify(&req.current_password, current_user.password_hash())?;
    if !password_matches {
        return Err(AppError::InvalidCredentials);
    }

    let new_password_hash = bcrypt::hash(&req.new_password, bcrypt::DEFAULT_COST)?;

    let updated_user = repository::update_password(&state.pool, &user_id, &new_password_hash)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(updated_user))
}
