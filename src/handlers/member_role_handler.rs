use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    app_state::AppState,
    schemas::member_role::{
        AssignRoleRequest,
        AssignRoleResponse,
    },
    services::member_role_service::MemberRoleService,
};

pub async fn assign(
    State(state): State<AppState>,
    Json(request): Json<AssignRoleRequest>,
) -> Result<Json<AssignRoleResponse>, StatusCode> {
    MemberRoleService::assign(&state.db, request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AssignRoleResponse {
        message: String::from("Role assigned successfully"),
    }))
}
