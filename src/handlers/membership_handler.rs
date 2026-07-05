use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::AppState,
    schemas::membership::{
        CreateMembershipRequest,
        MembershipDto,
        MembershipResponse,
    },
    services::membership_service::MembershipService,
};

fn validate_request<T: Validate>(request: &T) -> Result<(), StatusCode> {
    request.validate().map_err(|_| StatusCode::BAD_REQUEST)
}

pub async fn create(
    State(state): State<AppState>,
    Json(request): Json<CreateMembershipRequest>,
) -> Result<Json<MembershipResponse>, StatusCode> {
    validate_request(&request)?;

    MembershipService::create(&state.db, request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MembershipResponse {
        message: "Membership Created Successfully".to_string(),
    }))
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<MembershipDto>>, StatusCode> {
    let memberships = MembershipService::list(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(memberships))
}

pub async fn get_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<MembershipDto>, StatusCode> {
    let membership = MembershipService::get_by_id(&state.db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(membership))
}