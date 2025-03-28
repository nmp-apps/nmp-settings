use dioxus::prelude::*;

const STYLES: Asset = asset!("/assets/styles/ui/button.css");

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    children: Option<Element>,
    title: Option<String>,
    #[props(default = false)]
    primary: bool,
    #[props(default = false)]
    disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "ui-button",
            class: if props.primary { "primary" },
            class: if props.disabled { "disabled" },

            disabled: props.disabled,
            title: props.title.unwrap_or(String::new()),

            onclick: move |event| {
                if let Some(handler) = props.onclick {
                    handler.call(event)
                }
            },
            document::Link { rel: "stylesheet", href: STYLES }
            if let Some(children) = props.children {
                {children}
            } else {
                "Button"
            }
        }
    }
}