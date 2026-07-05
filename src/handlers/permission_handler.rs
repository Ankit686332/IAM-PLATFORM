use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::AppState,
    models::permission::Permission,
    schemas::{
        filter::SearchQuery,
        pagination::PaginationQuery,
        permission::{
            CreatePermissionRequest,
            PermissionResponse,
        },
    },
    services::permission_service::PermissionService,
};

fn validate_request<T: Validate>(request: &T) -> Result<(), StatusCode> {
    request.validate().map_err(|_| StatusCode::BAD_REQUEST)
}

pub async fn create(
    State(state): State<AppState>,
    Json(request): Json<CreatePermissionRequest>,
) -> Result<Json<PermissionResponse>, StatusCode> {
    validate_request(&request)?;

    PermissionService::create(&state.db, request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PermissionResponse {
        message: String::from("Permission created successfully"),
    }))
}

pub async fn get_all(
    State(state): State<AppState>,
    Query(search): Query<SearchQuery>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<Permission>>, StatusCode> {
    let permissions = PermissionService::get_all(
        &state.db,
        search.search.as_deref().unwrap_or(""),
        pagination.limit(),
        pagination.offset(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(permissions))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<Option<Permission>> {
    let permission = PermissionService::get_by_id(&state.db, id)
        .await
        .unwrap();

    Json(permission)
}