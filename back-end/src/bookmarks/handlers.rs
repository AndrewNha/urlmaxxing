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
        bookmark::Bookmark, bookmark_request::CreateBookmarkRequest,
        update_bookmark_request::UpdateBookmarkRequest,
    },
    state::AppState,
};

use super::repository;

pub async fn get_bookmarks(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let bookmarks = repository::find_bookmarks(&state.pool).await?;
    Ok(Json(bookmarks))
}

pub async fn get_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let bookmark = repository::find_bookmark(&state.pool, &id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(bookmark))
}

pub async fn create_bookmark(
    State(state): State<AppState>,
    Json(req): Json<CreateBookmarkRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: substituir por um user_id real vindo de autenticação (JWT) quando estiver disponível
    let user_id = Uuid::nil();
    let bookmark = Bookmark::new(user_id, req.title, req.url, req.tags);

    repository::insert_bookmark(&state.pool, &bookmark).await?;

    Ok((StatusCode::CREATED, Json(bookmark)))
}

pub async fn update_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateBookmarkRequest>,
) -> Result<impl IntoResponse, AppError> {
    let bookmark = repository::save_bookmark(&state.pool, &id, &req)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(bookmark))
}

pub async fn replace_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateBookmarkRequest>,
) -> Result<impl IntoResponse, AppError> {
    let bookmark = repository::replace_bookmark(&state.pool, &id, &req)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(bookmark))
}

pub async fn delete_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let bookmark = repository::remove_bookmark(&state.pool, &id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(bookmark))
}
