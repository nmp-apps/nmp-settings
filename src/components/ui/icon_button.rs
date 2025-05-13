use dioxus::prelude::*;

use crate::models::Icon;

#[derive(PartialEq, Props, Clone)]
pub struct IconButtonProps {
    icon: Icon,
    title: Option<String>,
    #[props(into, default = "24px")]
    size: String,
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

            {props.icon.to_component(props.size)}
        }
    }
}