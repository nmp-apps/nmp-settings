use dioxus::{desktop::{use_window, DesktopContext}, prelude::*};

use crate::{components::IconButton, models::Icon};

#[derive(PartialEq, Props, Clone)]
pub struct WindowBarProps {
    title: String,
    on_close: Callback
}

#[component]
pub fn WindowBar(props: WindowBarProps) -> Element {
    let window: DesktopContext = use_window();
    let h = props.on_close;
    let move_window = move |_| {
        window.drag();
    };
    rsx! {
        div { class: "window-bar", onmousedown: move_window,
            div { class: "window-bar__left" }
            div { class: "window-bar__center",
                h1 { class: "window-bar__title", {props.title} }
            }
            div {
                class: "window-bar__right",
                onmousedown: |evt| {
                    evt.stop_propagation();
                },
                IconButton {
                    onclick: move |_| h(()),
                    size: "16px",
                    icon: Icon::Close,
                }
            }
        }
    }
}