use sqlx::PgPool;
use uuid::Uuid;

use crate::models::audit_log::AuditLog;

pub struct AuditLogRepository;

impl AuditLogRepository {
    pub async fn create(
        pool: &PgPool,
        user_id: Option<Uuid>,
        organization_id: Option<Uuid>,
        action: &str,
        resource: &str,
        resource_id: Option<Uuid>,
        details: Option<&str>,
        ip_address: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO audit_logs
            (
                user_id,
                organization_id,
                action,
                resource,
                resource_id,
                details,
                ip_address
            )
            VALUES
            ($1,$2,$3,$4,$5,$6,$7)
            ",
        )
        .bind(user_id)
        .bind(organization_id)
        .bind(action)
        .bind(resource)
        .bind(resource_id)
        .bind(details)
        .bind(ip_address)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_all(
        pool: &PgPool,
    ) -> Result<Vec<AuditLog>, sqlx::Error> {
        let logs = sqlx::query_as::<_, AuditLog>(
            "
            SELECT *
            FROM audit_logs
            ORDER BY created_at DESC
            ",
        )
        .fetch_all(pool)
        .await?;

        Ok(logs)
    }
}