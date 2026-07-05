use sqlx::PgPool;

use crate::models::organization::Organization;

pub struct OrganizationRepository;

impl OrganizationRepository {
    pub async fn create(
        pool: &PgPool,
        name: &str,
        description: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO organizations
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

    pub async fn get_by_id(
        pool: &PgPool,
        id: uuid::Uuid,
    ) -> Result<Option<Organization>, sqlx::Error> {
        let organization = sqlx::query_as::<_, Organization>(
            "
            SELECT *
            FROM organizations
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(organization)
    }
    pub async fn get_all(
    pool: &PgPool,
    search:&str,
    limit: i64,
    offset: i64,
) -> Result<Vec<Organization>, sqlx::Error> {
    let organizations = sqlx::query_as::<_, Organization>(
        "
        SELECT *
        FROM organizations
        WHERE ($1='' OR name ILIKE '%' || $1 || '%')
        ORDER BY created_at DESC
        LIMIT $2
        OFFSET $3
        ",
    )
    
    .bind(search)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(organizations)
}
}