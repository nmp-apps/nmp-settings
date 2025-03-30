use dioxus::prelude::*;

use crate::components::{Button, IconButton, Switch};
use crate::models::Icon;

const STYLES: Asset = asset!("/assets/styles/app/top_bar.css");

#[derive(PartialEq, Props, Clone)]
pub struct TopBarProps {
    #[props(into)]
    class: Option<String>
}

#[component]
pub fn TopBar(props: TopBarProps) -> Element {
    let mut is_advanced_settings_enabled = use_signal(|| false);
    let top_bar_classes = match props.class.clone() {
        Some(classes) => format!("top-bar {classes}"),
        None => "top-bar".to_string()
    };

    rsx! {
        document::Link { rel: "stylesheet", href: STYLES }

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
                    IconButton { size: "16px", icon: Icon::Minimize }
                    IconButton { size: "16px", icon: Icon::Square }
                    IconButton { size: "16px", icon: Icon::Close }
                }
            }
        }
    }
}