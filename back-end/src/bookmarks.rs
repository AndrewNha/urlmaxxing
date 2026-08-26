mod handlers;
mod repository;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::get_bookmarks).post(handlers::create_bookmark),
        )
        .route(
            "/{id}",
            get(handlers::get_bookmark)
                .put(handlers::replace_bookmark)
                .patch(handlers::update_bookmark)
                .delete(handlers::delete_bookmark),
        )
}
