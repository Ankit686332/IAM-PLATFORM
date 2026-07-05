use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::AppState,
    models::organization::Organization,
    schemas::{
        filter::SearchQuery,
        organization::{
            CreateOrganizationRequest,
            OrganizationResponse,
        },
        pagination::PaginationQuery,
    },
    services::organization_service::OrganizationService,
};

fn validate_request<T: Validate>(request: &T) -> Result<(), StatusCode> {
    request.validate().map_err(|_| StatusCode::BAD_REQUEST)
}

pub async fn create(
    State(state): State<AppState>,
    Json(request): Json<CreateOrganizationRequest>,
) -> Result<Json<OrganizationResponse>, StatusCode> {
    validate_request(&request)?;

    OrganizationService::create(
        &state.db,
        request,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(OrganizationResponse {
        message: "Organization Created Successfully".to_string(),
    }))
}

pub async fn get_all(
    State(state): State<AppState>,
    Query(search): Query<SearchQuery>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<Organization>>, StatusCode> {
    let organizations = OrganizationService::get_all(
        &state.db,
        search.search.as_deref().unwrap_or(""),
        pagination.limit(),
        pagination.offset(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(organizations))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<Organization>>, StatusCode> {
    let organization = OrganizationService::get_by_id(
        &state.db,
        id,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(organization))
}