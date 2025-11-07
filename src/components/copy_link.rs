use std::sync::Arc;

use dioxus::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[component]
pub fn CopyLink(url: Arc<String>) -> Element {
    let copy_status = use_signal(|| "点击复制".to_string());

    let url_for_render = url.clone();
    let url_for_copy = url.clone();

    let handle_click = move |event: Event<MouseData>| {
        event.stop_propagation();
        event.prevent_default();

        let url = url_for_copy.clone();
        let mut copy_status = copy_status.to_owned();

        spawn(async move {
            match copy_to_clipboard(&url).await {
                Ok(_) => {
                    copy_status.set("已复制".to_string());

                    gloo_timers::future::sleep(std::time::Duration::from_secs(2)).await;

                    copy_status.set("点击复制".to_string());
                }
                Err(err) => {
                    tracing::error!("复制失败: {}", err);

                    copy_status.set("复制失败".to_string());
                }
            }
        });
    };

    rsx! {
        a {
            href: "{url_for_render}",
            class: "inline-flex items-center justify-center space-x-2 text-blue-500 hover:underline cursor-pointer",
            onclick: handle_click,
            span { "{url_for_render}" }
            span {
                class: "ml-2 px-2 py-0.5 text-xs rounded-full bg-gray-200 text-gray-700",
                "{copy_status}"
            }
        }
    }
}

async fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let window = window().ok_or("No window available")?;
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();

    let promise = clipboard.write_text(text);

    JsFuture::from(promise)
        .await
        .map_err(|e| format!("Failed to copy to clipboard: {:#?}", e))?;

    Ok(())
}
