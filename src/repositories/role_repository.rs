use sqlx::PgPool;
use uuid::Uuid;

use crate::models::role::Role;

pub struct RoleRepository;

impl RoleRepository {
    pub async fn create(
        pool: &PgPool,
        organization_id: Uuid,
        name: &str,
        description: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO roles
            (organization_id,name,description)
            VALUES
            ($1,$2,$3)
            ",
        )
        .bind(organization_id)
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
) -> Result<Vec<Role>, sqlx::Error> {
    let roles = sqlx::query_as::<_, Role>(
        "
        SELECT *
        FROM roles
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

    Ok(roles)
}

    pub async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<Role>, sqlx::Error> {
        let role = sqlx::query_as::<_, Role>(
            "
            SELECT *
            FROM roles
            WHERE id=$1
            ",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(role)
    }
}