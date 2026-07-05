use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState,
    handlers::membership_handler,
};

pub fn membership_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(membership_handler::create)
                .get(membership_handler::list),
        )
        .route(
            "/{id}",
            get(membership_handler::get_by_id),
        )
}