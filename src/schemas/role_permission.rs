use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AssignPermissionRequest {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct AssignPermissionResponse {
    pub message: String,
}