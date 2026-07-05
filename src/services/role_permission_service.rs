use sqlx::PgPool;

use crate::{
    repositories::role_permission_repository::RolePermissionRepository,
    schemas::role_permission::AssignPermissionRequest,
};

pub struct RolePermissionService;

impl RolePermissionService {
    pub async fn assign(
        pool: &PgPool,
        request: AssignPermissionRequest,
    ) -> Result<(), sqlx::Error> {
        RolePermissionRepository::assign(
            pool,
            request.role_id,
            request.permission_id,
        )
        .await
    }
}