mod handlers;
mod repository;

use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_user))
        .route(
            "/{id}",
            get(handlers::get_user)
                .put(handlers::replace_user)
                .delete(handlers::delete_user),
        )
        .route("/{id}/password", patch(handlers::update_password))
}
