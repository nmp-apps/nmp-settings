use dioxus::prelude::*;

use crate::get_asset;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    children: Option<Element>,
    title: Option<String>,
    #[props(default = false)]
    primary: ReadOnlySignal<Option<bool>>,
    #[props(default = false)]
    disabled: ReadOnlySignal<Option<bool>>,
    onclick: Option<EventHandler<MouseEvent>>
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/ui/button.css"));

    let classes = use_memo(move || {
        let mut result = vec!["ui-button"];
        match *props.disabled.read() {
            Some(v) => {
                if v {
                    result.push("disabled");
                }
            },
            None => (),
        }
        match *props.primary.read() {
            Some(v) => {
                if v {
                    result.push("primary");
                }
            },
            None => (),
        }
        result.join(" ")
    });

    rsx! {
        button {
            class: classes,

            disabled: props.disabled,
            title: props.title.unwrap_or(String::new()),

            onclick: move |event| {
                if let Some(handler) = props.onclick {
                    handler.call(event)
                }
            },

            document::Link { rel: "stylesheet", href: "{styles}" }

            if let Some(children) = props.children {
                {children}
            } else {
                "Button"
            }
        }
    }
}