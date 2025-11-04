use dioxus::prelude::*;

use crate::server::Route;

#[derive(Clone, Props, PartialEq)]
pub struct RouteButtonProps {
    name: String,
    class: Option<String>,
    goto: Option<Route>,
}

#[component]
pub fn RouteButton(props: RouteButtonProps) -> Element {
    let navigator = use_navigator();

    rsx! {
        button {
            class: props.class,
            onclick: move |_| {
                if props.goto.is_some() {
                    navigator.push(props.goto.clone().unwrap());
                }
            },
            {props.name}
        }
    }
}
