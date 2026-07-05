use sqlx::PgPool;
use uuid::Uuid;

use crate::models::api_key::ApiKey;

pub struct ApiKeyRepository;

impl ApiKeyRepository {
    pub async fn create(
        pool: &PgPool,
        organization_id: Uuid,
        name: &str,
        description: Option<&str>,
        key_hash: &str,
        scopes: &str,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<ApiKey, sqlx::Error> {
        sqlx::query_as::<_, ApiKey>(
            r#"
            INSERT INTO api_keys
            (organization_id, name, description, key_hash, scopes, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(name)
        .bind(description)
        .bind(key_hash)
        .bind(scopes)
        .bind(expires_at)
        .fetch_one(pool)
        .await
    }

  pub async fn get_all(
    pool: &PgPool,
    search: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ApiKey>, sqlx::Error> {
    let api_keys = sqlx::query_as::<_, ApiKey>(
        "
        SELECT *
        FROM api_keys
        WHERE ($1 = '' OR name ILIKE '%' || $1 || '%')
        ORDER BY created_at DESC
        LIMIT $2
        OFFSET $3
        ",
    )
    .bind(search)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(api_keys)
}

    pub async fn delete(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM api_keys WHERE id = $1",
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }
}

