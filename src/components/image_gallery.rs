use dioxus::{html::FileData, prelude::*};
use uuid::Uuid;
use web_sys::Url;

use crate::{components::InlineSvg, views::FileMap};

const PREVIOUS: &str = include_str!("../../assets/svg/previous.svg");
const NEXT: &str = include_str!("../../assets/svg/next.svg");

#[component]
pub fn ImageGallery(files: Signal<FileMap>) -> Element {
    let image_list: Vec<(Uuid, FileData)> = files
        .read()
        .iter()
        .map(|(uuid, data)| (*uuid, data.clone()))
        .collect();

    let file_nums = image_list.len();
    let mut image_index = use_signal(|| 0);
    let mut image_url = use_signal(|| String::new());

    use_effect(move || {
        tracing::info!("Current index: {}", image_index());
        tracing::debug!("Image list: {:#?}", image_list);

        let file_data = image_list[image_index()].1.clone();

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
    }
}

fn get_web_file(file_data: &FileData) -> Result<&web_sys::File, String> {
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
