use axum::{
    middleware,
    routing::{delete, get},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::session_handler,
    middleware::auth::auth,
};

pub fn session_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(session_handler::get_sessions),
        )
        .route(
            "/{id}",
            delete(session_handler::delete_session),
        )
        .layer(middleware::from_fn(auth))
}