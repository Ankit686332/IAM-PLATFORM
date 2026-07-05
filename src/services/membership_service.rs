use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    repositories::membership_repository::MembershipRepository,
    schemas::membership::{
        CreateMembershipRequest,
        MembershipDto,
    },
};

pub struct MembershipService;

impl MembershipService {
    pub async fn create(
        pool: &PgPool,
        request: CreateMembershipRequest,
    ) -> Result<(), sqlx::Error> {
        MembershipRepository::create(
            pool,
            request.user_id,
            request.organization_id,
        )
        .await
    }

    pub async fn list(
        pool: &PgPool,
    ) -> Result<Vec<MembershipDto>, sqlx::Error> {
        MembershipRepository::list(pool).await
    }

    pub async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<MembershipDto, sqlx::Error> {
        MembershipRepository::get_by_id(pool, id).await
    }
}