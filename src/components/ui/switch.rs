use dioxus::prelude::*;

use crate::get_asset;

#[derive(PartialEq, Props, Clone)]
pub struct SwitchProps {
    #[props(default = false)]
    value: bool,
    title: Option<String>,
    #[props(default = false)]
    disabled: bool,
    onchange: Option<EventHandler<Event<FormData>>>
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let styles: String = get_asset!("/assets/styles/ui/switch.css");
    let mut is_checked = use_signal(|| props.value);
    use_effect(move || {
        if is_checked() != props.value {
            is_checked.set(props.value)
        }
    });

    let handler = move |event: Event<FormData>| {
        if let Some(handler) = props.onchange {
            handler.call(event);
        }
    };
    rsx! {
        div {
            class: "ui-switch",
            class: if props.disabled { "disabled" },
            title: props.title.unwrap_or(String::new()),
            document::Link { rel: "stylesheet", href: styles }
            input {
                r#type: "checkbox",
                class: "ui-switch-checkbox",
                checked: is_checked,
                onchange: handler,
            }
            div { class: "ui-switch-thumb" }
        }
    }
}