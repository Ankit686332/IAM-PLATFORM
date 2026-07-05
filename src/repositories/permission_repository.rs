use sqlx::PgPool;
use uuid::Uuid;

use crate::models::permission::Permission;

pub struct PermissionRepository;

impl PermissionRepository {
    pub async fn create(
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO permissions
            (name, description)
            VALUES
            ($1,$2)
            ",
        )
        .bind(name)
        .bind(description)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_all(
    pool: &PgPool,
    search: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<Permission>, sqlx::Error> {
    let permissions = sqlx::query_as::<_, Permission>(
        "
        SELECT *
        FROM permissions
        WHERE ($1 = '' OR name ILIKE '%' || $1 || '%')
        ORDER BY name
        LIMIT $2
        OFFSET $3
        ",
    )
    .bind(search)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(permissions)
}
    pub async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<Permission>, sqlx::Error> {
        let permission = sqlx::query_as::<_, Permission>(
            "
            SELECT *
            FROM permissions
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(permission)
    }
}