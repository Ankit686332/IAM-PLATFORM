use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMembershipRequest {
    pub user_id: Uuid,
    pub organization_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct MembershipResponse {
    pub message: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MembershipDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
}