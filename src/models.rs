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

#[derive(Serialize, Deserialize, Debug)]
pub struct PresignRequest {
    pub uuid: String,
    pub extension: String,
}
