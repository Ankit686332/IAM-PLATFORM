use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::role_handler,
};

pub fn role_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(role_handler::create))
        .route("/", get(role_handler::get_all))
        .route("/{id}", get(role_handler::get_by_id))
}