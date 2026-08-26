mod handlers;
mod repository;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::get_users).post(handlers::create_user))
        .route(
            "/{id}",
            get(handlers::get_user)
                .put(handlers::replace_user)
                .patch(handlers::update_user)
                .delete(handlers::delete_user),
        )
}
