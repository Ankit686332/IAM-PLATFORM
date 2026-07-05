use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    services::rbac_service::RbacService,
};

pub async fn require_permission(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let membership_id = request
        .headers()
        .get("x-membership-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok());

    let permission = request
        .headers()
        .get("x-permission")
        .and_then(|v| v.to_str().ok());

    let membership_id = membership_id.ok_or(StatusCode::UNAUTHORIZED)?;
    let permission = permission.ok_or(StatusCode::BAD_REQUEST)?;

    let allowed = RbacService::has_permission(
        &state.db,
        membership_id,
        permission,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !allowed {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}