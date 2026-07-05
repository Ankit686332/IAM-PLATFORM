use sqlx::PgPool;
use uuid::Uuid;

pub struct MemberRoleRepository;

impl MemberRoleRepository {
    pub async fn assign(
        pool: &PgPool,
        membership_id: Uuid,
        role_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO member_roles
            (membership_id, role_id)
            VALUES
            ($1, $2)
            ",
        )
        .bind(membership_id)
        .bind(role_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}