use sqlx::PgPool;

use crate::{
    repositories::member_role_repository::MemberRoleRepository,
    schemas::member_role::AssignRoleRequest,
};

pub struct MemberRoleService;

impl MemberRoleService {
    pub async fn assign(
        pool: &PgPool,
        request: AssignRoleRequest,
    ) -> Result<(), sqlx::Error> {
        MemberRoleRepository::assign(
            pool,
            request.membership_id,
            request.role_id,
        )
        .await
    }
}