use dioxus::prelude::*;

use crate::components::Button;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonGroupProps {
    value: ReadSignal<String>,
    items: ReadSignal<Vec<ButtonGroupItem>>,
    onchange: Option<EventHandler<String>>,
    disabled: ReadSignal<bool>,
}

#[derive(PartialEq, Clone)]
pub struct ButtonGroupItem {
    text: String,
    value: String,
}

impl ButtonGroupItem {
    pub fn new(text: String, value: String) -> ButtonGroupItem {
        ButtonGroupItem { text, value }
    }
}

#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    rsx! {
        div { class: "ui-button-group",
            for item in props.items.read().clone() {
                Button {
                    primary: props.value == item.value,
                    onclick: move |_| {
                        if let Some(handler) = props.onchange {
                            handler.call(item.value.clone())
                        }
                    },
                    disabled: *props.disabled.read(),
                    "{item.text}"
                }
            }
        }
    }
}