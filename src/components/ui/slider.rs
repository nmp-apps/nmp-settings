use dioxus::prelude::*;

use crate::get_asset;

#[component]
pub fn Slider() -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/ui/slider.css"));
    let mut value = use_signal(|| 0.0);
    let min_value = use_signal(|| 0.0);
    let max_value = use_signal(|| 100.0);
    let filled_part_rem = use_memo(move || {
        value() * 16.0 / max_value() // 16.0 is width in rem from slider's styles
    });

    rsx! {
        document::Stylesheet { href: "{styles}" }
        input {
            class: "ui-slider",
            r#type: "range",
            style: "--private-slider-fill: {filled_part_rem}rem",
            min: "{min_value}",
            max: "{max_value}",
            value: "{value}",
            oninput: move |evt| {
                let new_value: f64 = evt.data.value().parse().unwrap_or(0.0);
                value.set(new_value);
            },
        }
    }
}