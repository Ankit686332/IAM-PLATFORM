use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::Error as SqlxError;

use validator::Validate;

use crate::{
    app_state::AppState,
    config::settings::Settings,
    schemas::auth::{
        AuthResponse,
        LoginRequest,
        LoginResponse,
        LogoutRequest,
        RefreshRequest,
        RegisterRequest,
    },
    services::audit_log_service::AuditLogService,
    services::auth_service::AuthService,
    utils::{
        jwt::{
            generate_access_token,
            generate_refresh_token,
            verify_token,
        },
        password::verify_password,
    },
};

fn validate_request<T: Validate>(request: &T) -> Result<(), StatusCode> {
    request.validate().map_err(|_| StatusCode::BAD_REQUEST)
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    validate_request(&request)?;

    let register_result = AuthService::register(&state.db, request).await;

    match register_result {
        Ok(()) => {
            AuditLogService::create(
                &state.db,
                None,
                None,
                "REGISTER",
                "USER",
                None,
                Some("User registered"),
                None,
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Json(AuthResponse {
                message: "User Registered Successfully".to_string(),
            }))
        }
        Err(SqlxError::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            Err(StatusCode::CONFLICT)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    validate_request(&request)?;

    let normalized_email = request.email.trim().to_lowercase();
    let password = request.password.trim();

    let user = AuthService::find_user(
        &state.db,
        &normalized_email,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    AuditLogService::create(
        &state.db,
        Some(user.id),
        None,
        "LOGIN",
        "USER",
        Some(user.id),
        Some("User logged in"),
        None,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let valid = verify_password(
        password,
        &user.password_hash,
    );

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let settings = Settings::new();

    let access_token = generate_access_token(
        &user.id.to_string(),
        &settings.jwt_secret,
    );

    let refresh_token = generate_refresh_token(
        &user.id.to_string(),
        &settings.jwt_secret,
    );

    AuthService::save_refresh_token(
        &state.db,
        user.id,
        &refresh_token,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
    }))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(request): Json<RefreshRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let settings = Settings::new();

    let claims = verify_token(
        &request.refresh_token,
        &settings.jwt_secret,
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let valid = AuthService::validate_refresh_token(
        &state.db,
        &request.refresh_token,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let access_token = generate_access_token(
        &claims.sub,
        &settings.jwt_secret,
    );

    Ok(Json(LoginResponse {
        access_token,
        refresh_token: request.refresh_token,
    }))
}
pub async fn logout(
    State(state): State<AppState>,
    Json(request): Json<LogoutRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    AuthService::logout(
        &state.db,
        &request.refresh_token,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    AuditLogService::create(
        &state.db,
        None,
        None,
        "LOGOUT",
        "USER",
        None,
        Some("User logged out"),
        None,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        message: "Logged out successfully".to_string(),
    }))
}