use dioxus::prelude::*;

use crate::components::*;

const STYLES: Asset = asset!("/assets/styles/ui/icon_button.css");

#[derive(PartialEq, Clone)]
pub enum Icon {
    Close,
    Extension,
    Minimize,
    Square,
}

impl Icon {
    fn to_component(&self, size: String) -> Element {
        match self {
            Icon::Close => CloseIcon(IconProps { color: None, size: Some(size), }),
            Icon::Extension => ExtensionIcon(IconProps { color: None, size: Some(size), }),
            Icon::Minimize => MinimizeIcon(IconProps { color: None, size: Some(size), }),
            Icon::Square => SquareIcon(IconProps { color: None, size: Some(size), }),
        }
    }
}

// impl Icon {
//     fn to_component(&self, p: IconProps) -> Box<dyn Fn(IconProps) -> Element> {
//         match self {
//             Icon::Close => Box::new(move |p| CloseIcon(p)),
//             Icon::Extension => Box::new(move |p| ExtensionIcon(p)),
//             Icon::Minimize => Box::new(move |p| MinimizeIcon(p)),
//             Icon::Square => Box::new(move |p| SquareIcon(p)),
//         }
//     }
// }

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

            document::Link { rel: "stylesheet", href: STYLES }
            {props.icon.to_component(props.size)}
        }
    }
}