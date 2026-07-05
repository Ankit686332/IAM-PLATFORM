use axum::{
    routing::post,
    Router,
};

use crate::{
    app_state::AppState,
    handlers::role_permission_handler,
};

pub fn role_permission_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/assign",
            post(role_permission_handler::assign),
        )
}