use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 2))]
    pub full_name: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateProfileResponse {
    pub message: String,
}