use axum::{
    routing::post,
    Router,
};

use crate::{
    app_state::AppState,
    handlers::auth_handler,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/register",
            post(auth_handler::register),
        )
        .route(
            "/login",
            post(auth_handler::login),
        )
        .route(
            "/refresh",
            post(auth_handler::refresh),
        )
        .route(
            "/logout",
            post(auth_handler::logout),
        )

}