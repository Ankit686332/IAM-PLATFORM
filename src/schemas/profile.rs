use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
}