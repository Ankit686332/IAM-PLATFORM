use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrganizationRequest {
    #[validate(length(min = 3))]
    pub name: String,

    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationResponse {
    pub message: String,
}