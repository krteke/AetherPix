use dioxus::prelude::*;

use crate::{components::{AuthGuard, RouteButton}, server::Route};

#[component]
pub fn Home() -> Element {
    rsx! {
        AuthGuard {
            div { class: "flex flex-row justify-center items-center w-full min-h-dvh",
                RouteButton {
                    name: "浏览图片",
                    class: "px-6 py-3 bg-blue-600 text-white rounded-lg cursor-pointer hover:bg-blue-700 transition-colors",
                    goto: Route::Gallery {},
                }
                RouteButton {
                    name: "上传图片",
                    class: "ml-4 px-6 py-3 bg-green-600 text-white rounded-lg cursor-pointer hover:bg-green-700 transition-colors",
                    goto: Route::Upload {},
                }
            }
        }
    }
}
