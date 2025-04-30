use std::ops::Deref;

use dioxus::prelude::*;

use crate::get_asset;

#[derive(PartialEq, Props, Clone)]
pub struct SliderProps {
    value: ReadOnlySignal<f64>,
    min: ReadOnlySignal<f64>,
    max: ReadOnlySignal<f64>,
    step: ReadOnlySignal<f64>,
    #[props(default = ReadOnlySignal::new(use_signal(|| false)))]
    disabled: ReadOnlySignal<bool>,
    oninput: EventHandler<f64>
}

#[component]
pub fn Slider(props: SliderProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/ui/slider.css"));

    let filled_part_rem = use_memo(move || {
        let value_ref = props.value.read();
        let value = value_ref.deref();
        let max_ref = props.max.read();
        let max = max_ref.deref();
        value * 16.0 / max // 16.0 is width in rem from slider's styles
    });

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
        }
    };

    let slider_handler =move |evt: Event<FormData>| {
        let min_ref = props.min.read();
        let min = min_ref.deref().clone();
        let new_value: f64 = evt.data.value().parse().unwrap_or(min);
        props.oninput.call(new_value);
    };

    rsx! {
        div { class: "ui-slider", class: if *props.disabled.read() { "disabled" },
            document::Stylesheet { href: "{styles}" }
            input {
                class: "ui-slider__input",
                min: "{props.min}",
                max: "{props.max}",
                value: "{props.value}",
                r#type: "number",
                disabled: *props.disabled.read(),
                oninput: input_handler,
            }
            input {
                class: "ui-slider__slider",
                r#type: "range",
                style: "--private-slider-fill: {filled_part_rem}rem",
                min: "{props.min}",
                max: "{props.max}",
                step: "{props.step}",
                value: "{props.value}",
                disabled: *props.disabled.read(),
                oninput: slider_handler,
            }
        }
    }
}