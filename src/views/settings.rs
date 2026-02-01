use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub upload_max_size: u64,
    pub allow_registration: bool,
    pub site_name: String,
    pub allow_everyone_upload: bool,
}
