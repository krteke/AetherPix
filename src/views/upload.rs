use std::collections::HashMap;

use dioxus::{html::FileData, prelude::*};
use reqwest::Client;
use uuid::Uuid;
use web_sys::{wasm_bindgen::JsCast, window, HtmlInputElement};

use crate::{
    components::{AuthGuard, RouteButton},
    models::PresignRequest,
    server::Route,
    utils::get_browser_origin,
};

type FileMap = HashMap<Uuid, FileData>;

#[component]
pub fn Upload() -> Element {
    const INPUT_FILE_ID: &str = "file-picker";
    let mut uploaded_files = use_signal(|| FileMap::new());

    let on_file_change = move |e: Event<FormData>| {
        let file_list = e.files();
        if file_list.is_empty() {
            return;
        }

        let mut file_write = uploaded_files.write();
        let mut uuid_vec = Vec::new();

        // TODO: Need to optimize
        for file in file_list {
            let file_uuid = Uuid::new_v4();
            let extension = file
                .name()
                .rsplit_once(".")
                .map(|(_, ext)| ext)
                .unwrap_or("dat")
                .to_string();

            file_write.insert(file_uuid, file);
            uuid_vec.push(PresignRequest {
                uuid: file_uuid.to_string(),
                extension: extension,
            });
        }
        //

        spawn(async move {
            let client = Client::new();

            let base_url = get_browser_origin().unwrap_or_else(|| "".to_string());
            let url = format!("{}/api/presign", base_url);

            let response = client.post(url).json(&uuid_vec).send().await;

            // TODO: Implement error handling for presign request
            match response {
                Ok(res) => {
                    tracing::debug!("{:#?}", res);
                }
                Err(e) => {
                    tracing::error!("Failed to presign files: {}", e);
                }
            }
        });
    };

    // TODO: Implement image show
    rsx! {
        AuthGuard {
            div { class: "flex flex-row justify-center items-center w-full min-h-dvh",
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
                                    return;
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
            }
        }
    }
}
