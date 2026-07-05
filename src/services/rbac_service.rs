use sqlx::PgPool;
use uuid::Uuid;

use crate::repositories::rbac_repository::RbacRepository;

pub struct RbacService;

impl RbacService {
    pub async fn has_permission(
        pool: &PgPool,
        membership_id: Uuid,
        permission: &str,
    ) -> Result<bool, sqlx::Error> {
        RbacRepository::has_permission(
            pool,
            membership_id,
            permission,
        )
        .await
    }
}