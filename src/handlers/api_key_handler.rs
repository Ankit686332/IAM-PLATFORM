use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::AppState,
    models::api_key::ApiKey,
    schemas::api_key::{
        CreateApiKeyRequest,
        CreateApiKeyResponse,
    },
    schemas::auth::AuthResponse,
    schemas::pagination::PaginationQuery,
    services::{
        api_key_service::ApiKeyService,
        audit_log_service::AuditLogService,
    },
    schemas::filter::SearchQuery,
};

fn validate_request<T: Validate>(request: &T) -> Result<(), StatusCode> {
    request.validate().map_err(|_| StatusCode::BAD_REQUEST)
}

pub async fn create_api_key(
    State(state): State<AppState>,
    Json(request): Json<CreateApiKeyRequest>,
) -> Result<Json<CreateApiKeyResponse>, StatusCode> {
    validate_request(&request)?;

    let (api_key, raw_key) = ApiKeyService::create(
        &state.db,
        request.organization_id,
        request.name,
        request.description,
        request.scopes,
        request.expires_at,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    AuditLogService::create(
        &state.db,
        None,
        Some(request.organization_id),
        "CREATE",
        "API_KEY",
        Some(api_key.id),
        Some("API Key created"),
        None,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateApiKeyResponse { api_key: raw_key }))
}

pub async fn list_api_keys(
    State(state): State<AppState>,
    Query(search): Query<SearchQuery>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<ApiKey>>, StatusCode> {
    let api_keys = ApiKeyService::get_all(
        &state.db,
        search.search.as_deref().unwrap_or(""),
        pagination.limit(),
        pagination.offset(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(api_keys))
}

pub async fn delete_api_key(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AuthResponse>, StatusCode> {
    ApiKeyService::delete(&state.db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        message: "API Key deleted successfully".to_string(),
    }))
}