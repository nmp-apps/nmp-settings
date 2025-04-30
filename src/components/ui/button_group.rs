use dioxus::prelude::*;

use crate::{components::Button, get_asset};

#[derive(PartialEq, Props, Clone)]
pub struct ButtonGroupProps {
    value: ReadOnlySignal<String>,
    items: ReadOnlySignal<Vec<ButtonGroupItem>>,
    onchange: Option<EventHandler<String>>,
    disabled: ReadOnlySignal<bool>,
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
    let styles: String = use_hook(|| get_asset!("/assets/styles/ui/button_group.css"));

    rsx! {
        div { class: "ui-button-group",
            document::Stylesheet { href: "{styles}" }

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