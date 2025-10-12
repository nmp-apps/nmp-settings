use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TextProps {
    value: ReadSignal<String>,
    min_length: ReadSignal<u32>,
    max_length: ReadSignal<u32>,
    #[props(default = ReadSignal::new(Signal::new(false)))]
    disabled: ReadSignal<bool>,
    oninput: Option<EventHandler<String>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Text(props: TextProps) -> Element {
    let input_handler = move |evt: Event<FormData>| {
        let min = props.min_length.read();
        let min = min.clone();
        let max = props.max_length.read();
        let max = max.clone();
        let new_value = evt.data.value();
        let length: u32 = new_value.len() as u32;
        match props.oninput {
            Some(handler) => {
                if length < min {
                    handler.call(props.value.read().clone());
                } else if length > max {
                    handler.call(props.value.read().clone());
                } else {
                    handler.call(new_value);
                };
            },
            None => ()
        }
    };

    rsx! {
        div {
            class: "ui-text",
            class: if props.disabled.read().clone() { "disabled" },
            input {
                class: "ui-text__input",
                r#type: "text",
                value: props.value,
                minlength: props.min_length,
                maxlength: props.max_length,
                oninput: input_handler,
                onmousedown: move |evt| {
                    if let Some(handler) = props.onmousedown {
                        handler.call(evt)
                    }
                },
                disabled: *props.disabled.read(),
            }
        }
    }
}