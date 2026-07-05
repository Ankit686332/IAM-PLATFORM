use sqlx::PgPool;

use crate::{
    repositories::user_repository::UserRepository,
    schemas::auth::RegisterRequest,
    utils::password::hash_password,
};

pub struct AuthService;

fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

impl AuthService {
    pub async fn register(
        pool: &PgPool,
        request: RegisterRequest,
    ) -> Result<(), sqlx::Error> {
        let password_hash = hash_password(&request.password);
        let normalized_email = normalize_email(&request.email);

        UserRepository::create(
            pool,
            &request.full_name,
            &normalized_email,
            &password_hash,
        )
        .await
    }

    pub async fn find_user(
        pool: &PgPool,
        email: &str,
    ) -> Result<Option<crate::models::user::User>, sqlx::Error> {
        let normalized_email = normalize_email(email);
        UserRepository::find_by_email(pool, &normalized_email).await
    }
    pub async fn save_refresh_token(
    pool: &PgPool,
    user_id: uuid::Uuid,
    token: &str,
) -> Result<(), sqlx::Error> {
    use crate::repositories::refresh_token_repository::RefreshTokenRepository;

    RefreshTokenRepository::create(
        pool,
        user_id,
        token,
    )
    .await
}
pub async fn validate_refresh_token(
    pool: &PgPool,
    token: &str,
) -> Result<bool, sqlx::Error> {
    use crate::repositories::refresh_token_repository::RefreshTokenRepository;

    RefreshTokenRepository::exists(
        pool,
        token,
    )
    .await
}
pub async fn logout(
    pool: &PgPool,
    token: &str,
) -> Result<(), sqlx::Error> {
    use crate::repositories::refresh_token_repository::RefreshTokenRepository;

    RefreshTokenRepository::delete(
        pool,
        token,
    )
    .await
}
}

#[cfg(test)]
mod tests {
    use super::normalize_email;

    #[test]
    fn normalizes_email_for_login() {
        assert_eq!(normalize_email("  User@Example.com  "), "user@example.com");
    }
}