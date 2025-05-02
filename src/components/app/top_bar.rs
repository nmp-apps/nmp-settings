use dioxus::prelude::*;
use dioxus::desktop::{use_window, DesktopContext};

use crate::get_asset;
use crate::components::{AdvancedSettingsSwitch, CloseButton, IconButton, PluginsButton, SaveSettingsButton};
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
    let top_bar_classes = match &props.class {
        Some(classes) => format!("top-bar {classes}"),
        None => "top-bar".to_string()
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
                PluginsButton {}
                AdvancedSettingsSwitch {}
                SaveSettingsButton {}
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
                    CloseButton {}
                }
            }
        }
    }
}