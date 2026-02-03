use serde::Serialize;

#[derive(Serialize)]
pub struct ListViewResponse {
    pub images: Vec<Image>,
    pub total: u64,
    pub pages: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: i32,
    pub preview_url: String,
    pub original_url: String,
    pub name: String,
    pub size: String,
}
