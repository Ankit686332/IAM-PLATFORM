use axum::{
    routing::post,
    Router,
};

use crate::{
    app_state::AppState,
    handlers::member_role_handler,
};

pub fn member_role_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/assign",
            post(member_role_handler::assign),
        )
}