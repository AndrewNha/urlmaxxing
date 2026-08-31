use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{
        auth_user::AuthUser, bookmark::Bookmark, bookmark_request::CreateBookmarkRequest,
        update_bookmark_request::UpdateBookmarkRequest,
    },
    state::AppState,
};

use super::repository;

pub async fn get_bookmarks(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let bookmarks = repository::find_bookmarks(&state.pool, auth_user.user_id()).await?;
    Ok(Json(bookmarks))
}

pub async fn get_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let bookmark = repository::find_bookmark(&state.pool, &id, &auth_user.user_id())
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(bookmark))
}

pub async fn create_bookmark(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateBookmarkRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = *auth_user.user_id();
    let bookmark = Bookmark::new(user_id, req.title, req.url, req.tags);

    repository::insert_bookmark(&state.pool, &bookmark).await?;

    Ok((StatusCode::CREATED, Json(bookmark)))
}

pub async fn update_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    auth_user: AuthUser,
    Json(req): Json<UpdateBookmarkRequest>,
) -> Result<impl IntoResponse, AppError> {
    let bookmark = repository::save_bookmark(&state.pool, &id, &req, auth_user.user_id())
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(bookmark))
}

pub async fn replace_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    auth_user: AuthUser,
    Json(req): Json<CreateBookmarkRequest>,
) -> Result<impl IntoResponse, AppError> {
    let bookmark = repository::replace_bookmark(&state.pool, &id, &req, auth_user.user_id())
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(bookmark))
}

pub async fn delete_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let bookmark = repository::remove_bookmark(&state.pool, &id, auth_user.user_id())
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(bookmark))
}
