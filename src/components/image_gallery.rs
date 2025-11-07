use std::{collections::HashSet, sync::Arc};

use dioxus::{html::FileData, prelude::*};
use uuid::Uuid;
use web_sys::Url;

use crate::{
    components::{copy_link::CopyLink, InlineSvg},
    views::FileMap,
};

const PREVIOUS: &str = include_str!("../../assets/svg/previous.svg");
const NEXT: &str = include_str!("../../assets/svg/next.svg");

#[derive(Props, PartialEq, Clone)]
pub struct ImageGalleryProps {
    files: Signal<FileMap>,
    failed_uuids: Signal<HashSet<String>>,
    file_with_ext_and_uuid: Signal<Vec<(Uuid, String)>>,
    url: String,
}

#[derive(Debug, Clone)]
struct ImageStatus {
    file_data: FileData,
    url: String,
    is_success: bool,
}

#[component]
pub fn ImageGallery(props: ImageGalleryProps) -> Element {
    let image_vec = build_status_vec(
        &props.files.read(),
        (props.failed_uuids)(),
        (props.file_with_ext_and_uuid)(),
    );

    let file_nums = image_vec.len();
    let mut image_index = use_signal(|| 0);
    let mut image_url = use_signal(|| String::new());
    let image_vec_clone = image_vec.clone();

    use_effect(move || {
        tracing::info!("Current index: {}", image_index());
        tracing::debug!("Image list: {:#?}", image_vec);

        let file_data = image_vec[image_index()].file_data.clone();

        spawn(async move {
            match create_preview_url(&file_data) {
                Ok(data) => {
                    image_url.set(data);
                }
                Err(e) => {
                    tracing::error!("Failed to convert file data to data URL: {:?}", e);
                    image_url.set("".to_string());
                }
            }
        });
    });

    rsx! {
        div {
            class: "flex flex-row items-center justify-center w-full pb-8",
            button {
                class: "flex items-center justify-center cursor-pointer h-dvh w-1/10 hover:bg-gray-200/30",
                onclick: move |_| {
                    image_index.set((image_index() + file_nums - 1) % file_nums);
                },
                InlineSvg {
                    class: "w-12 h-12",
                    content: PREVIOUS
                }
            }
            div {
                class: "w-4/5",
                img {
                    class: "w-full h-full object-contain",
                    src: "{image_url}"
                }
            }
            button {
                class: "flex items-center justify-center cursor-pointer h-dvh w-1/10 hover:bg-gray-200/30",
                onclick: move |_| {
                    image_index.set((image_index() + 1) % file_nums);
                },
                InlineSvg {
                    class: "w-12 h-12",
                    content: NEXT
                }
            }
        }
        div {
            class: "flex flex-row items-center justify-center px-4 py-2 mb-8 rounded-lg bg-gray-100/30",
            {if image_vec_clone[image_index()].is_success {
                let url = Arc::new(format!("{}/uploads/{}", props.url, image_vec_clone[image_index()].url));
                rsx! {
                    CopyLink {
                        url
                    }
                }
            } else {
                rsx! {
                    p {
                        class: "text-center text-lg text-red",
                        "上传失败"
                    }
                }
            }}
        }
    }
}

pub fn get_web_file(file_data: &FileData) -> Result<&web_sys::File, String> {
    file_data
        .inner()
        .downcast_ref::<web_sys::File>()
        .ok_or_else(|| {
            tracing::error!("Failed to get web file");
            "Failed to get web file".to_string()
        })
}

fn create_preview_url(file_data: &FileData) -> Result<String, String> {
    let web_file = get_web_file(file_data)?;

    let url = Url::create_object_url_with_blob(web_file).map_err(|e| {
        tracing::error!("Failed to create preview URL: {:?}", e);
        format!("Failed to create preview URL: {:?}", e)
    })?;

    Ok(url)
}

fn build_status_vec(
    filemap: &FileMap,
    failed_vec: HashSet<String>,
    full: Vec<(Uuid, String)>,
) -> Vec<ImageStatus> {
    let statuses: Vec<ImageStatus> = full
        .into_iter()
        .filter_map(|fullname| {
            let value = match filemap.get(&fullname.0) {
                Some(v) => v.clone(),
                None => {
                    tracing::debug!("No uuid: {}", fullname.0);
                    return None;
                }
            };

            let is_successful = !failed_vec.contains(&fullname.0.to_string());

            Some(ImageStatus {
                file_data: value,
                url: format!("{}.{}", fullname.0, fullname.1),
                is_success: is_successful,
            })
        })
        .collect();

    statuses
}
