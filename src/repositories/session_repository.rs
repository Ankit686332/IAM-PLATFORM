use sqlx::PgPool;
use uuid::Uuid;

use crate::models::session::Session;

pub struct SessionRepository;

impl SessionRepository {
    pub async fn get_user_sessions(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<Session>, sqlx::Error> {
        sqlx::query_as::<_, Session>(
            "
            SELECT *
            FROM sessions
            WHERE user_id = $1
            ORDER BY created_at DESC
            ",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    pub async fn delete_session(
        pool: &PgPool,
        session_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            DELETE FROM sessions
            WHERE id = $1
            ",
        )
        .bind(session_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}