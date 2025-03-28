use dioxus::prelude::*;

use crate::components::*;

const STYLES: Asset = asset!("/assets/styles/ui/icon_button.css");

#[derive(PartialEq, Clone)]
pub enum Icon {
    // Close,
    Extension,
    // Minimize,
    // Square,
}

impl Icon {
    fn to_component(&self) -> Element {
        match self {
            // Icon::Close => rsx! {
            //     CloseIcon {}
            // },
            Icon::Extension => rsx! {
                ExtensionIcon {}
            },
            // Icon::Minimize => rsx! {
            //     MinimizeIcon {}
            // },
            // Icon::Square => rsx! {
            //     SquareIcon {}
            // }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct IconButtonProps {
    icon: Icon,
    title: Option<String>,
    #[props(default = false)]
    disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>
}

#[component]
pub fn IconButton(props: IconButtonProps) -> Element {
    rsx! {
        button {
            class: "ui-icon-button",
            class: if props.disabled { "disabled" },
            disabled: props.disabled,
            title: props.title.unwrap_or(String::new()),

            onclick: move |event| {
                if let Some(handler) = props.onclick {
                    handler.call(event)
                }
            },

            document::Link { rel: "stylesheet", href: STYLES }
            {props.icon.to_component()}
        }
    }
}