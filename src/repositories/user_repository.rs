use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &PgPool,
        full_name: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO users
            (full_name,email,password_hash)
            VALUES
            ($1,$2,$3)
            ",
        )
        .bind(full_name)
        .bind(email)
        .bind(password_hash)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "
            SELECT *
            FROM users
            WHERE email = $1
            ",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
    pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "
        SELECT *
        FROM users
        WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
pub async fn update_profile(
    pool: &PgPool,
    id: Uuid,
    full_name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        UPDATE users
        SET full_name = $1
        WHERE id = $2
        ",
    )
    .bind(full_name)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn get_all(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT *
        FROM users
        ORDER BY created_at DESC
        LIMIT $1
        OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}
}