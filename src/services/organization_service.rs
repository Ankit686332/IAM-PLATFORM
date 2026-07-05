use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::organization::Organization,
    repositories::organization_repository::OrganizationRepository,
    schemas::organization::CreateOrganizationRequest,
};

pub struct OrganizationService;

impl OrganizationService {
    pub async fn create(
        pool: &PgPool,
        request: CreateOrganizationRequest,
    ) -> Result<(), sqlx::Error> {
        OrganizationRepository::create(
            pool,
            &request.name,
            request.description.as_deref(),
        )
        .await
    }

    pub async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<Organization>, sqlx::Error> {
        OrganizationRepository::get_by_id(pool, id).await
    }

    pub async fn get_all(
        pool: &PgPool,
        search: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Organization>, sqlx::Error> {
        OrganizationRepository::get_all(
            pool,
            search,
            limit,
            offset,
        )
        .await
    }
}