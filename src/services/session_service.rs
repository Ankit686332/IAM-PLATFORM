use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::session::Session,
    repositories::session_repository::SessionRepository,
};

pub struct SessionService;

impl SessionService {
    pub async fn get_sessions(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<Session>, sqlx::Error> {
        SessionRepository::get_user_sessions(
            pool,
            user_id,
        )
        .await
    }

    pub async fn delete_session(
        pool: &PgPool,
        session_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        SessionRepository::delete_session(
            pool,
            session_id,
        )
        .await
    }
}