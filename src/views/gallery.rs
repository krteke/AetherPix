use dioxus::prelude::*;

use crate::components::AuthGuard;

#[component]
pub fn Gallery() -> Element {
    rsx! {
        AuthGuard {
            div {
            }
        }
    }
}
