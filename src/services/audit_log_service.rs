use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::audit_log::AuditLog,
    repositories::audit_log_repository::AuditLogRepository,
};

pub struct AuditLogService;

impl AuditLogService {
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
        AuditLogRepository::create(
            pool,
            user_id,
            organization_id,
            action,
            resource,
            resource_id,
            details,
            ip_address,
        )
        .await
    }

    pub async fn get_all(
        pool: &PgPool,
    ) -> Result<Vec<AuditLog>, sqlx::Error> {
        AuditLogRepository::get_all(pool).await
    }
}