use sqlx::PgPool;
use uuid::Uuid;

pub struct RbacRepository;

impl RbacRepository {
    pub async fn has_permission(
        pool: &PgPool,
        membership_id: Uuid,
        permission: &str,
    ) -> Result<bool, sqlx::Error> {
        let result: Option<(i64,)> =
            sqlx::query_as(
                "
                SELECT COUNT(*)
                FROM member_roles mr
                JOIN role_permissions rp
                    ON mr.role_id = rp.role_id
                JOIN permissions p
                    ON rp.permission_id = p.id
                WHERE mr.membership_id = $1
                AND p.name = $2
                ",
            )
            .bind(membership_id)
            .bind(permission)
            .fetch_optional(pool)
            .await?;

        Ok(result.map(|r| r.0 > 0).unwrap_or(false))
    }
}