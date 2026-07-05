use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct RefreshTokenRepository;

impl RefreshTokenRepository {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        token_hash: &str,
    ) -> Result<(), sqlx::Error> {
        let expires_at = Utc::now() + Duration::days(30);

        sqlx::query(
            "
            INSERT INTO refresh_tokens
            (user_id, token_hash, expires_at)
            VALUES
            ($1,$2,$3)
            ",
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .execute(pool)
        .await?;

        Ok(())
    }
    pub async fn exists(
    pool: &PgPool,
    token_hash: &str,
       ) -> Result<bool, sqlx::Error> {
    let result: Option<(uuid::Uuid,)> =
        sqlx::query_as(
            "
            SELECT id
            FROM refresh_tokens
            WHERE token_hash = $1
            AND expires_at > NOW()
            ",
        )
        .bind(token_hash)
        .fetch_optional(pool)
        .await?;

    Ok(result.is_some())
}
pub async fn delete(
    pool: &PgPool,
    token_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        DELETE FROM refresh_tokens
        WHERE token_hash = $1
        ",
    )
    .bind(token_hash)
    .execute(pool)
    .await?;

    Ok(())
}
}