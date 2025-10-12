use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SwitchProps {
    value: ReadSignal<bool>,
    title: ReadSignal<Option<String>>,
    #[props(default = ReadSignal::new(Signal::new(false)))]
    disabled: ReadSignal<bool>,
    onchange: Option<Callback<bool>>
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let handler = move |event: Event<MouseData>| {
        event.prevent_default();
        if let Some(handler) = props.onchange {
            handler.call(!*props.value.read());
        }
    };
    rsx! {
        div {
            class: "ui-switch",
            class: if props.disabled.read().clone() { "disabled" },
            title: props.title.read().clone().unwrap_or(String::new()),
            input {
                r#type: "checkbox",
                class: "ui-switch-checkbox",
                checked: props.value,
                onclick: handler,
            }
            div { class: "ui-switch-thumb" }
        }
    }
}