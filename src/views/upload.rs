use serde::Serialize;

use crate::controllers::upload::UploadResult;

#[derive(Serialize)]
pub struct UploadResponse {
    pub url: Option<String>,
    // pub status: String,
}

impl From<UploadResult> for UploadResponse {
    fn from(value: UploadResult) -> Self {
        UploadResponse {
            url: Some(value.url),
            // status: value.status,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PresignResponse {
    pub upload_url: String,
}
