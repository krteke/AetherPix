use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use dioxus::{html::FileData, prelude::*};
use futures::{future::join, lock::Mutex, stream, StreamExt};
use gloo_net::http::Request;
use reqwest::Client;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use web_sys::{
    wasm_bindgen::{JsCast, JsValue},
    window, HtmlInputElement,
};

use crate::{
    components::{get_web_file, AuthGuard, Dialog, ImageGallery, RouteButton},
    models::{PresignRequest, PresignedResponse},
    server::Route,
    utils::get_browser_origin,
};

pub type FileMap = HashMap<Uuid, FileData>;
const MAX_CONCURRENT_UPLOADS: usize = 5;
const INPUT_FILE_ID: &str = "file-picker";

#[component]
pub fn Upload() -> Element {
    let mut uploaded_files = use_signal(|| FileMap::new());
    let mut open_dialog = use_signal(|| false);
    let mut presigned_urls = use_signal(|| HashMap::new());
    let mut dialog_title = use_signal(|| String::new());
    let mut dialog_content = use_signal(|| String::new());
    let mut failed_uuids = use_signal(|| HashSet::new());
    let mut access_url = use_signal(|| String::new());
    let mut enable_to_upload = use_signal(|| false);
    let mut uuid_vec = Vec::new();
    let mut file_with_ext_and_uuid = use_signal(|| Vec::new());
    let mut file_hashset = HashSet::new();

    let on_file_change = move |e: Event<FormData>| {
        let file_list = e.files();
        if file_list.is_empty() {
            return;
        }

        for file in file_list {
            let hash = calculate_file_hash(&file.name());
            if file_hashset.contains(&hash) {
                tracing::info!("已跳过重复文件: {}", file.name());

                continue;
            } else {
                file_hashset.insert(hash);
            }

            let file_uuid = Uuid::new_v4();
            let extension = file
                .name()
                .rsplit_once(".")
                .map(|(_, ext)| ext)
                .unwrap_or("dat")
                .to_string();

            tracing::debug!(
                "File {} renamed to uuid {}.{}",
                &file.name(),
                file_uuid,
                extension
            );

            uploaded_files.insert(file_uuid, file);
            uuid_vec.push(PresignRequest {
                uuid: file_uuid.to_string(),
                extension: extension.clone(),
            });

            file_with_ext_and_uuid.push((file_uuid, extension));
        }
        //

        let uuid_vec_clone = uuid_vec.clone();

        spawn(async move {
            let client = Client::new();

            let base_url = get_browser_origin().unwrap_or("".to_string());
            let url = format!("{}/api/presign", base_url);
            let post_r2_url = format!("{}/api/r2url", base_url);

            let response = client.post(url).json(&uuid_vec_clone).send();
            let url_response = client.post(post_r2_url).send();
            let res = join(response, url_response).await;

            match res {
                (Ok(res1), Ok(res2)) => {
                    tracing::debug!("Presign response: {:?}", res1);
                    tracing::debug!("R2 URL response: {:?}", res2);

                    if res1.status().is_success() {
                        enable_to_upload.set(true);
                        open_dialog.set(false);

                        match res1.json::<PresignedResponse>().await {
                            Ok(presigned_response) => {
                                presigned_urls.set(presigned_response.urls);
                            }
                            Err(e) => {
                                tracing::error!("Failed to parse presigned response: {}", e);
                                enable_to_upload.set(false);

                                dialog_title.set("解析失败".to_string());
                                dialog_content.set("解析response失败，请重试".to_string());
                                open_dialog.set(true);
                            }
                        }
                    } else {
                        enable_to_upload.set(false);

                        dialog_title.set("预签名失败".to_string());
                        dialog_content.set("预签名失败，请重试".to_string());
                        open_dialog.set(true);
                    }

                    tracing::error!("Failed to get presign response");

                    if res2.status().is_success() {
                        match res2.text().await {
                            Ok(url) => {
                                access_url.set(url);
                            }
                            Err(e) => {
                                tracing::error!("Failed to parse access URL: {}", e);

                                enable_to_upload.set(false);

                                dialog_title.set("解析失败".to_string());
                                dialog_content.set("解析URL失败，请重试".to_string());
                                open_dialog.set(true);
                            }
                        }
                    } else {
                        tracing::error!("Failed to parse access URL");
                        enable_to_upload.set(false);

                        dialog_title.set("解析URL失败".to_string());
                        dialog_content.set("解析URL失败，请重试".to_string());
                        open_dialog.set(true);
                    }
                }
                (Err(e), _) => {
                    tracing::error!("Failed to presign files: {}", e);

                    enable_to_upload.set(false);

                    dialog_title.set("预签名失败".to_string());
                    dialog_content.set("预签名失败，请重试".to_string());
                    open_dialog.set(true);
                }
                (_, Err(e)) => {
                    tracing::error!("Failed to parse access URL: {}", e);

                    enable_to_upload.set(false);

                    dialog_title.set("解析URL失败".to_string());
                    dialog_content.set("解析URL失败，请重试".to_string());
                    open_dialog.set(true);
                }
            }
        });
    };

    rsx! {
        AuthGuard {
            Dialog{
                open: open_dialog,
                title: dialog_title,
                description: dialog_content
            }
            div { class: "flex flex-col justify-center items-center w-full min-h-dvh",
                {if !uploaded_files.is_empty() {
                    rsx! {
                        ImageGallery {
                            files: uploaded_files,
                            failed_uuids,
                            url: access_url(),
                            file_with_ext_and_uuid
                        }
                    }
                } else {
                    rsx! {
                        div {}
                    }
                }}
                div {
                    class: "flex flex-row justify-center items-center",
                    RouteButton {
                        name: "返回首页",
                        class: "px-6 py-3 bg-gray-600 text-white rounded-lg cursor-pointer hover:bg-gray-700 transition-colors",
                        goto: Route::Home {},
                    }
                    button {
                        class: "ml-4 px-6 py-3 bg-green-600 text-white rounded-lg cursor-pointer hover:bg-green-700 transition-colors",
                        onclick: move |_| {
                            if let Some(document) = window().and_then(|w| w.document()) {
                                if let Some(element) = document.get_element_by_id(INPUT_FILE_ID) {
                                    if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                                        input.click();
                                    }
                                }
                            }
                        },
                        "选择文件上传"
                    }
                    input {
                        r#type: "file",
                        id: "{INPUT_FILE_ID}",
                        style: "display: none;",
                        multiple: true,
                        accept: "image/*",
                        onchange: on_file_change,
                    }
                    button {
                        class: "ml-4 px-6 py-3 bg-blue-600 text-white rounded-lg transition-colors",
                        class: if enable_to_upload() { "bg-blue-600 cursor-pointer hover:bg-blue-700"}
                            else { "bg-gray-300" },
                        disabled: !enable_to_upload(),
                        onclick: move |_| async move {
                            let upload_result = start_upload_process(presigned_urls(), uploaded_files()).await;

                            match upload_result {
                                Ok(_) => {
                                    dialog_title.set("上传成功".to_string());
                                    dialog_content.set("所有图片上传成功".to_string());
                                    open_dialog.set(true);
                                },
                                Err(e) => {
                                    tracing::debug!("Some image upload failed: {:#?}", e);

                                    dialog_title.set("上传失败".to_string());
                                    dialog_content.set("部分图片上传失败".to_string());
                                    open_dialog.set(true);
                                    failed_uuids.set(e);
                                    // Handle upload errors
                                }
                            }
                        },
                        "上传"
                    }
                }
            }
        }
    }
}

async fn start_upload_process(
    urls: HashMap<String, String>,
    files: FileMap,
) -> Result<(), HashSet<String>> {
    let upload_stream = stream::iter(urls);
    let failed_uuids_arc = Arc::new(Mutex::new(HashSet::new()));

    upload_stream
        .for_each_concurrent(MAX_CONCURRENT_UPLOADS, |(uuid, url)| {
            let file_ref = &files;
            let failed_uuids = Arc::clone(&failed_uuids_arc);

            async move {
                let parsed_uuid: Uuid = match uuid.parse() {
                    Ok(u) => u,
                    Err(e) => {
                        tracing::error!("Failed to parse {}: {}", uuid, e);
                        let mut failed_uuids = failed_uuids.lock().await;
                        failed_uuids.insert(uuid.clone());

                        return;
                    }
                };

                let file_data = match file_ref.get(&parsed_uuid) {
                    Some(data) => data,
                    None => {
                        tracing::error!("File not found for UUID: {}", uuid);
                        let mut failed_uuids = failed_uuids.lock().await;
                        failed_uuids.insert(uuid.clone());

                        return;
                    }
                };

                if let Err(e) = upload(&url, file_data).await {
                    tracing::error!("Failed to upload {}: {}", uuid, e);
                    let mut failed_uuids = failed_uuids.lock().await;
                    failed_uuids.insert(uuid);
                }
            }
        })
        .await;

    let failed_list = failed_uuids_arc.lock().await;

    if failed_list.is_empty() {
        Ok(())
    } else {
        Err(failed_list.clone())
    }
}

async fn upload(url: &str, file_data: &FileData) -> Result<(), String> {
    let file = get_web_file(file_data)?;

    let content_type = file_data
        .content_type()
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let response = Request::put(url)
        .header("Content-Type", &content_type)
        .body(JsValue::from(file.clone()))
        .map_err(|e| format!("Failed to create request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to get response: {}", e))?;

    if !response.ok() {
        return Err(format!("Failed to upload file: {}", response.status()));
    }

    Ok(())
}

fn calculate_file_hash(filename: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(filename.as_bytes());
    let hash_bytes = hasher.finalize();

    hash_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
