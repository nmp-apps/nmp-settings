use dioxus::desktop::{use_window, DesktopContext};
use dioxus::prelude::*;

use crate::get_asset;
use crate::components::{Button, IconButton, Switch};
use crate::models::Icon;


#[derive(PartialEq, Props, Clone)]
pub struct TopBarProps {
    #[props(into)]
    class: Option<String>
}

#[component]
pub fn TopBar(props: TopBarProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/app/top_bar.css"));
    let window: DesktopContext = use_window();
    let mut is_advanced_settings_enabled = use_signal(|| false);
    let top_bar_classes = match &props.class {
        Some(classes) => format!("top-bar {classes}"),
        None => "top-bar".to_string()
    };

    let click_close_handler = {
        let window = window.clone();
        move || {
            window.close();
        }
    };
    let click_minimize_handler = {
        let window = window.clone();
        move || {
            window.set_minimized(true);
        }
    };
    let click_maximize_handler = {
        let window = window.clone();
        move || {
            window.set_maximized(true);
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: "{styles}" }

        div { class: top_bar_classes,
            div { class: "top-bar__left" }
            div { class: "top-bar__center",
                h1 { class: "top-bar__title", "Settings" }
            }
            div { class: "top-bar__right",
                IconButton { icon: Icon::Extension, title: "Manage Extensions" }
                Switch {
                    title: "Enable advanced settings",
                    value: is_advanced_settings_enabled(),
                    onchange: move |event: Event<FormData>| is_advanced_settings_enabled.set(event.checked()),
                }
                Button { title: "Save changes", primary: true, "Save" }
                div { class: "top-bar__window-actions",
                    IconButton {
                        onclick: move |_| click_minimize_handler(),
                        size: "16px",
                        icon: Icon::Minimize,
                    }
                    IconButton {
                        onclick: move |_| click_maximize_handler(),
                        size: "16px",
                        icon: Icon::Square,
                    }
                    IconButton {
                        onclick: move |_| click_close_handler(),
                        size: "16px",
                        icon: Icon::Close,
                    }
                }
            }
        }
    }
}