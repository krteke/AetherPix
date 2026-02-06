use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub upload_max_size: u64,
    pub allow_registration: bool,
    pub site_name: String,
    pub allow_everyone_upload: bool,
    pub local_base_url: String,
    pub r2_base_url: String,
}
