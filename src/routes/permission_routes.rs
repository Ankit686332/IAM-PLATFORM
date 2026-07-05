use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::permission_handler,
};

pub fn permission_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(permission_handler::create))
        .route("/", get(permission_handler::get_all))
        .route("/{id}", get(permission_handler::get_by_id))
}