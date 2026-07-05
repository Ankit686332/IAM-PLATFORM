use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AssignRoleRequest {
    pub membership_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct AssignRoleResponse {
    pub message: String,
}