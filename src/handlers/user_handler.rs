use axum::{
    extract::{Extension,Query, State},
    Json,
};
use uuid::Uuid;
use validator::Validate;


use crate::{
    app_state::AppState,
    models::user::User,
    schemas::profile::ProfileResponse,
    schemas::update_profile::{
        UpdateProfileRequest,
        UpdateProfileResponse,
    },
    schemas::pagination::PaginationQuery,
    services::user_service::UserService,
};

pub async fn profile(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Json<ProfileResponse> {
    let user = UserService::get_profile(
        &state.db,
        user_id,
    )
    .await
    .unwrap()
    .unwrap();

    Json(ProfileResponse {
        id: user.id,
        full_name: user.full_name,
        email: user.email,
    })
    
}
pub async fn update_profile(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(request): Json<UpdateProfileRequest>,
) -> Json<UpdateProfileResponse> {
    request.validate().unwrap();

    UserService::update_profile(
        &state.db,
        user_id,
        &request.full_name,
    )
    .await
    .unwrap();

    Json(UpdateProfileResponse {
        message: "Profile updated successfully".to_string(),
    })
}
pub async fn get_all_users(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Json<Vec<User>> {
    let users = UserService::get_all(
        &state.db,
        pagination.limit(),
        pagination.offset(),
    )
    .await
    .unwrap();

    Json(users)
}