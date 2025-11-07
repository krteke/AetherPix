use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresignRequest {
    pub uuid: String,
    pub extension: String,
}

#[derive(Serialize, Deserialize)]
pub struct PresignedResponse {
    pub urls: HashMap<String, String>,
}
