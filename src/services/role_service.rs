use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::role::Role,
    repositories::role_repository::RoleRepository,
    schemas::role::CreateRoleRequest,
};

pub struct RoleService;

impl RoleService {
    pub async fn create(
        pool: &PgPool,
        request: CreateRoleRequest,
    ) -> Result<(), sqlx::Error> {
        RoleRepository::create(
            pool,
            request.organization_id,
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
) -> Result<Vec<Role>, sqlx::Error> {
    RoleRepository::get_all(
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
    ) -> Result<Option<Role>, sqlx::Error> {
        RoleRepository::get_by_id(pool, id).await
    }
}