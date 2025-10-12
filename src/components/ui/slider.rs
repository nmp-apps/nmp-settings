use std::ops::Deref;

use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SliderProps {
    value: ReadSignal<f64>,
    min: ReadSignal<f64>,
    max: ReadSignal<f64>,
    step: ReadSignal<f64>,
    #[props(default = ReadSignal::new(use_signal(|| false)))]
    disabled: ReadSignal<bool>,
    oninput: EventHandler<f64>,
    onmousedownslider: Option<EventHandler<MouseEvent>>,
    onmousedowninput: Option<EventHandler<MouseEvent>>,
    /// lazy mode change oninput inside slider and pass value only onchange
    lazy: Option<bool>
}

#[component]
pub fn Slider(props: SliderProps) -> Element {
    let mut lazy_value = use_signal(|| *props.value.read());
    let slider_value = use_memo(move || {
        match props.lazy {
            Some(lazy) => {
                if lazy {
                    lazy_value()
                } else {
                    *props.value.read()
                }
            }
            None => *props.value.read(),
        }
    });

    let filled_part_rem = use_memo(move || {
        let value_ref = slider_value.read();
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

    let slider_handler = move |evt: Event<FormData>| {
        let min_ref = props.min.read();
        let min = min_ref.deref().clone();
        let new_value: f64 = evt.data.value().parse().unwrap_or(min);
        props.oninput.call(new_value);
    };

    rsx! {
        div { class: "ui-slider", class: if *props.disabled.read() { "disabled" },
            input {
                class: "ui-slider__input",
                min: "{props.min}",
                max: "{props.max}",
                value: "{slider_value}",
                r#type: "number",
                disabled: *props.disabled.read(),
                oninput: input_handler,
                onmousedown: move |evt| {
                    if let Some(handler) = props.onmousedowninput {
                        handler.call(evt)
                    }
                },
            }
            input {
                class: "ui-slider__slider",
                r#type: "range",
                style: "--private-slider-fill: {filled_part_rem}rem",
                min: "{props.min}",
                max: "{props.max}",
                step: "{props.step}",
                value: "{slider_value}",
                disabled: *props.disabled.read(),
                oninput: move |evt| {
                    match props.lazy {
                        Some(lazy) => {
                            if lazy {
                                let min_ref = props.min.read();
                                let min = min_ref.deref().clone();
                                let new_value: f64 = evt.data.value().parse().unwrap_or(min);
                                lazy_value.set(new_value);
                            } else {
                                slider_handler(evt)
                            }
                        }
                        None => slider_handler(evt),
                    }
                },
                onchange: slider_handler,
                onmousedown: move |evt| {
                    if let Some(handler) = props.onmousedownslider {
                        handler.call(evt)
                    }
                },
            }
        }
    }
}