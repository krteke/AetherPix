use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileResponse {
    pub email: String,
    pub name: String,
    pub api_token: String,
}
