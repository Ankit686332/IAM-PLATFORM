use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    app_state::AppState,
    schemas::role_permission::{
        AssignPermissionRequest,
        AssignPermissionResponse,
    },
    services::role_permission_service::RolePermissionService,
};

pub async fn assign(
    State(state): State<AppState>,
    Json(request): Json<AssignPermissionRequest>,
) -> Result<Json<AssignPermissionResponse>, StatusCode> {
    RolePermissionService::assign(&state.db, request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AssignPermissionResponse {
        message: String::from("Permission assigned successfully"),
    }))
}
