use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoleRequest {
    pub organization_id: Uuid,

    #[validate(length(min = 2))]
    pub name: String,

    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub message: String,
}