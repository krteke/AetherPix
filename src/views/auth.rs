use serde::{Deserialize, Serialize};

use crate::models::{_entities::users, users::users::UserRole};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub pid: String,
    pub role: UserRole,
    pub username: String,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            role: user.role,
            username: user.username.clone(),
            is_verified: user.email_verified_at.is_some(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiKeyResponse {
    pub key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub role: UserRole,
    pub username: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            role: user.role,
            username: user.username.clone(),
            email: user.email.clone(),
        }
    }
}
