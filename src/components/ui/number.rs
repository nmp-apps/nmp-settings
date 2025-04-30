use std::ops::Deref;

use dioxus::{document, prelude::*};

use crate::get_asset;

#[derive(PartialEq, Props, Clone)]
pub struct NumberProps {
    value: ReadOnlySignal<f64>,
    min: ReadOnlySignal<f64>,
    max: ReadOnlySignal<f64>,
    step: ReadOnlySignal<f64>,
    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    disabled: ReadOnlySignal<bool>,
    oninput: EventHandler<f64>
}

#[component]
pub fn Number(props: NumberProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/ui/number.css"));

    let input_handler = move |evt: Event<FormData>| {
        let min_ref = props.min.read();
        let min = min_ref.deref().clone();
        let max_ref = props.max.read();
        let max = max_ref.deref().clone();
        let new_value: f64 = evt.data.value().parse().unwrap_or(min);
        if new_value < min {
            props.oninput.call(min);
        } else if new_value > max {
            props.oninput.call(max);
        } else {
            props.oninput.call(new_value);
        };
    };

    let increase_handler = move |_: Event<MouseData>| {
        let min_ref = props.min.read();
        let min = min_ref.deref().clone();
        let max_ref = props.max.read();
        let max = max_ref.deref().clone();
        let step = props.step.read();
        let step = step.deref().clone();
        if props.value.read().deref() + step > max {
            props.oninput.call(max);
        } else if props.value.read().deref() + step < min {
            props.oninput.call(min);
        } else {
            let cv = props.value.read().deref().clone();
            let result_float = cv + step;
            let formatted_result = round_float(result_float);
            props.oninput.call(formatted_result);
        }
    };

    let decrease_handler = move |_: Event<MouseData>| {
        let min_ref = props.min.read();
        let min = min_ref.deref().clone();
        let max_ref = props.max.read();
        let max = max_ref.deref().clone();
        let step = props.step.read();
        let step = step.deref().clone();
        if props.value.read().deref() - step > max {
            props.oninput.call(max);
        } else if props.value.read().deref() - step < min {
            props.oninput.call(min);
        } else {
            let cv = props.value.read().deref().clone();
            let result_float = cv - step;
            let formatted_result = round_float(result_float);
            props.oninput.call(formatted_result);
        }
    };

    rsx! {
        div {
            class: "ui-number",
            class: if props.disabled.read().clone() { "disabled" },
            document::Stylesheet { href: "{styles}" }

            button {
                class: "ui-number__step-button left",
                onclick: decrease_handler,
                "<"
            }
            input {
                class: "ui-number__input",
                r#type: "number",
                min: "{props.min}",
                max: "{props.max}",
                value: "{props.value}",
                step: "{props.step}",
                disabled: *props.disabled.read(),
                oninput: input_handler,
            }
            button {
                class: "ui-number__step-button right",
                onclick: increase_handler,
                ">"
            }
        }
    }
}

fn round_float(float: f64) -> f64 {
    const VALUE_FOR_ROUND: f64 = 10000000.0;
    (float * VALUE_FOR_ROUND).round() / VALUE_FOR_ROUND
}