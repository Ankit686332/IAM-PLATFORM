use axum::{
    middleware,
    routing::get,
    Router,
};

use crate::{
    app_state::AppState,
    handlers::user_handler,
    middleware::auth::auth,
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/me",
            get(user_handler::profile)
                .patch(user_handler::update_profile),
        )
        .layer(middleware::from_fn(auth))
}