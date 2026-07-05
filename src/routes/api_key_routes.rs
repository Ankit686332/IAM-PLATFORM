use axum::{
    middleware,
    routing::{delete, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::api_key_handler,
    middleware::auth::auth,
};

pub fn api_key_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(api_key_handler::create_api_key)
                .get(api_key_handler::list_api_keys),
        )
        .route(
            "/{id}",
            delete(api_key_handler::delete_api_key),
        )
        .layer(middleware::from_fn(auth))
}