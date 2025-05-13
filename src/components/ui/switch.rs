use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SwitchProps {
    value: ReadOnlySignal<bool>,
    title: ReadOnlySignal<Option<String>>,
    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    disabled: ReadOnlySignal<bool>,
    onchange: Option<EventHandler<Event<FormData>>>
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let handler = move |event: Event<FormData>| {
        if let Some(handler) = props.onchange {
            handler.call(event);
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
                onchange: handler,
            }
            div { class: "ui-switch-thumb" }
        }
    }
}