use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! {
        div {
            class: "fixed inset-0 flex items-center justify-center z-10",

            div {
                class: "h-10 w-10 rounded-full border-4 border-gray-200 border-t-blue-500 animate-spin"
            }
        }
    }
}
