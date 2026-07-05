use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::user::User,
    repositories::user_repository::UserRepository,
};

pub struct UserService;

impl UserService {
    pub async fn get_profile(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<User>, sqlx::Error> {
        UserRepository::find_by_id(
            pool,
            id,
        )
        .await
    }
    pub async fn update_profile(
    pool: &PgPool,
    id: Uuid,
    full_name: &str,
) -> Result<(), sqlx::Error> {
    UserRepository::update_profile(
        pool,
        id,
        full_name,
    )
    .await
}
pub async fn get_all(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<User>, sqlx::Error> {
    UserRepository::get_all(pool, limit, offset).await
}
}