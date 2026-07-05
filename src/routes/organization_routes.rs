use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::organization_handler,
};

pub fn organization_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(organization_handler::create))
        .route("/", get(organization_handler::get_all))
        .route("/{id}", get(organization_handler::get_by_id))
}