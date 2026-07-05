use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::api_key::ApiKey,
    repositories::api_key_repository::ApiKeyRepository,
};

pub struct ApiKeyService;

impl ApiKeyService {
    pub async fn create(
        pool: &PgPool,
        organization_id: Uuid,
        name: String,
        description: Option<String>,
        scopes: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<(ApiKey, String), sqlx::Error> {
        let raw_key = format!(
            "iam_{}",
            Uuid::new_v4().simple()
        );

        let mut hasher = Sha256::new();
        hasher.update(raw_key.as_bytes());

        let key_hash = format!("{:x}", hasher.finalize());

        let api_key = ApiKeyRepository::create(
            pool,
            organization_id,
            &name,
            description.as_deref(),
            &key_hash,
            &scopes,
            expires_at,
        )
        .await?;

        Ok((api_key, raw_key))
    }
    

   pub async fn get_all(
    pool: &PgPool,
    search: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ApiKey>, sqlx::Error> {
    ApiKeyRepository::get_all(
        pool,
        search,
        limit,
        offset,
    )
    .await
}

    pub async fn delete(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        ApiKeyRepository::delete(pool, id).await
    }
}