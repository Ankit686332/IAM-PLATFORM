use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::permission::Permission,
    repositories::permission_repository::PermissionRepository,
    schemas::permission::CreatePermissionRequest,
};

pub struct PermissionService;

impl PermissionService {
    pub async fn create(
        pool: &PgPool,
        request: CreatePermissionRequest,
    ) -> Result<(), sqlx::Error> {
        PermissionRepository::create(
            pool,
            &request.name,
            request.description.as_deref(),
        )
        .await
    }

    pub async fn get_all(
    pool: &PgPool,
    search: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<Permission>, sqlx::Error> {
    PermissionRepository::get_all(
        pool,
        search,
        limit,
        offset,
    )
    .await
}
    pub async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<Permission>, sqlx::Error> {
        PermissionRepository::get_by_id(pool, id).await
    }
}