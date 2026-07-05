use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MemberRole {
    pub id: Uuid,
    pub membership_id: Uuid,
    pub role_id: Uuid,
    pub created_at: DateTime<Utc>,
}