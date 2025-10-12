use dioxus::prelude::*;

use crate::models::Icon;

#[derive(PartialEq, Props, Clone)]
pub struct IconButtonProps {
    class: ReadSignal<Option<String>>,
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
    let classes = use_memo(move || {
        let mut res = vec![String::from("ui-icon-button")];
        if props.disabled {
            res.push(String::from("diasbled"));
        }
        if let Some(classes) = props.class.read().clone() {
            res.push(classes);
        }
        res.join(" ")
    });
    rsx! {
        button {
            class: classes(),
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