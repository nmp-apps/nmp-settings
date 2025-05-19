use dioxus::prelude::*;
use dioxus::desktop::{use_window, DesktopContext};

use crate::components::{
    AdvancedSettingsSwitch,
    CloseButton,
    IconButton,
    PluginsButton,
    SaveSettingsButton
};
use crate::models::Icon;

#[derive(PartialEq, Props, Clone)]
pub struct TopBarProps {
    #[props(into)]
    class: Option<String>
}

#[component]
pub fn TopBar(props: TopBarProps) -> Element {
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
    let move_window = move |_| {
        window.drag();
    };

    rsx! {
        div { class: top_bar_classes, onmousedown: move_window,
            div { class: "top-bar__left" }
            div { class: "top-bar__center",
                h1 { class: "top-bar__title", "Settings" }
            }
            div {
                class: "top-bar__right",
                onmousedown: |evt| {
                    evt.stop_propagation();
                },

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