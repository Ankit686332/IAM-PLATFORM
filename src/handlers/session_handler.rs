use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    schemas::auth::AuthResponse,
    schemas::session::SessionResponse,
    services::session_service::SessionService,
};

pub async fn get_sessions(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Vec<SessionResponse>>, StatusCode> {
    let sessions = SessionService::get_sessions(
        &state.db,
        user_id,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = sessions
        .into_iter()
        .map(|session| SessionResponse {
            id: session.id,
            user_id: session.user_id,
            created_at: session.created_at,
            expires_at: session.expires_at,
        })
        .collect();

    Ok(Json(response))
}

pub async fn delete_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AuthResponse>, StatusCode> {
    SessionService::delete_session(
        &state.db,
        id,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        message: "Session revoked successfully".to_string(),
    }))
}