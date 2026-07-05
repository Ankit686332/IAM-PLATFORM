use sqlx::PgPool;
use uuid::Uuid;

pub struct RolePermissionRepository;

impl RolePermissionRepository {
    pub async fn assign(
        pool: &PgPool,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO role_permissions
            (role_id, permission_id)
            VALUES
            ($1, $2)
            ",
        )
        .bind(role_id)
        .bind(permission_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}