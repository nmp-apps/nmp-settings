use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    children: Option<Element>,
    #[props(default = true)]
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

            onclick: move |event| {
                if let Some(handler) = props.onclick {
                    handler.call(event)
                }
            },
            document::Link {
                rel: "stylesheet",
                href: asset!("/assets/styles/ui_button.css"),
            }
            if let Some(children) = props.children {
                {children}
            } else {
                "Button"
            }
        }
    }
}