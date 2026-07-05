use sqlx::PgPool;
use uuid::Uuid;

use crate::schemas::membership::MembershipDto;

pub struct MembershipRepository;

impl MembershipRepository {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        organization_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO memberships
            (user_id, organization_id)
            VALUES
            ($1, $2)
            ",
        )
        .bind(user_id)
        .bind(organization_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn list(
        pool: &PgPool,
    ) -> Result<Vec<MembershipDto>, sqlx::Error> {
        sqlx::query_as::<_, MembershipDto>(
            "
            SELECT
                id,
                user_id,
                organization_id
            FROM memberships
            ORDER BY created_at DESC
            ",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<MembershipDto, sqlx::Error> {
        sqlx::query_as::<_, MembershipDto>(
            "
            SELECT
                id,
                user_id,
                organization_id
            FROM memberships
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_one(pool)
        .await
    }
}