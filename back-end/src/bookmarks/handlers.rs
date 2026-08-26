use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    models::{
        bookmark::Bookmark, bookmark_request::CreateBookmarkRequest,
        update_bookmark_request::UpdateBookmarkRequest,
    },
    state::AppState,
};

use super::repository;

pub async fn get_bookmarks(State(state): State<AppState>) -> impl IntoResponse {
    match repository::find_bookmarks(&state.pool).await {
        Ok(bookmarks) => (StatusCode::OK, Json(bookmarks)).into_response(),
        Err(e) => {
            eprintln!("Error fetching bookmarks: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error fetching bookmarks",
            )
                .into_response()
        }
    }
}

pub async fn get_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repository::find_bookmark(&state.pool, &id).await {
        Ok(Some(bookmark)) => (StatusCode::OK, Json(bookmark)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Bookmark not found").into_response(),
        Err(e) => {
            eprintln!("Error fetching bookmark: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching bookmark").into_response()
        }
    }
}

pub async fn create_bookmark(
    State(state): State<AppState>,
    Json(req): Json<CreateBookmarkRequest>,
) -> impl IntoResponse {
    // TODO: substituir por um user_id real vindo de autenticação (JWT) quando estiver disponível
    let user_id = Uuid::nil();
    let bookmark = Bookmark::new(user_id, req.title, req.url, req.tags);

    match repository::insert_bookmark(&state.pool, &bookmark).await {
        Ok(_) => (StatusCode::CREATED, Json(bookmark)).into_response(),
        Err(e) => {
            eprintln!("Error creating bookmark: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error creating bookmark").into_response()
        }
    }
}

pub async fn update_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateBookmarkRequest>,
) -> impl IntoResponse {
    match repository::save_bookmark(&state.pool, &id, &req).await {
        Ok(Some(bookmark)) => (StatusCode::OK, Json(bookmark)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Bookmark not found").into_response(),
        Err(e) => {
            eprintln!("Error updating bookmark: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error updating bookmark").into_response()
        }
    }
}

pub async fn replace_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateBookmarkRequest>,
) -> impl IntoResponse {
    match repository::replace_bookmark(&state.pool, &id, &req).await {
        Ok(Some(bookmark)) => (StatusCode::OK, Json(bookmark)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Bookmark not found").into_response(),
        Err(e) => {
            eprintln!("Error replacing bookmark: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error replacing bookmark",
            )
                .into_response()
        }
    }
}

pub async fn delete_bookmark(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repository::remove_bookmark(&state.pool, &id).await {
        Ok(Some(bookmark)) => (StatusCode::OK, Json(bookmark)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Bookmark not found").into_response(),
        Err(e) => {
            eprintln!("Error deleting bookmark: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting bookmark").into_response()
        }
    }
}
