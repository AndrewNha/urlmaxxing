use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use bcrypt::hash;
use uuid::Uuid;

use crate::{
    models::{
        register_request::RegisterRequest, replace_user_request::ReplaceUserRequest,
        update_profile_request::UpdateProfileRequest, user::User, user_response::UserResponse,
    },
    state::AppState,
};

use super::repository;

fn hash_password(password: &str, cost: u32) -> Result<String, bcrypt::BcryptError> {
    hash(password, cost)
}

pub async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    match repository::find_users(&state.pool).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => {
            eprintln!("Error fetching users: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching users").into_response()
        }
    }
}

pub async fn get_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repository::find_user(&state.pool, &id).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            eprintln!("Error fetching user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching user").into_response()
        }
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    let password_hash = match hash_password(&req.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password").into_response();
        }
    };

    let user = User::new(req.display_name, req.username, password_hash);

    match repository::insert_user(&state.pool, &user).await {
        Ok(_) => (StatusCode::CREATED, Json(UserResponse::from(&user))).into_response(),
        Err(e) => {
            eprintln!("Error creating user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error creating user").into_response()
        }
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdateProfileRequest>,
) -> impl IntoResponse {
    match repository::save_user(&state.pool, &user_id, &req).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            eprintln!("Error updating user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error updating user").into_response()
        }
    }
}

pub async fn replace_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<ReplaceUserRequest>,
) -> impl IntoResponse {
    match repository::replace_user(&state.pool, &user_id, &req).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            eprintln!("Error replacing user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error replacing user").into_response()
        }
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    match repository::remove_user(&state.pool, &user_id).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(e) => {
            eprintln!("Error deleting user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting user").into_response()
        }
    }
}
