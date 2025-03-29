use dioxus::prelude::*;

use super::IconProps;

#[component]
pub fn CloseIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 -960 960 960",
            height: props.size.clone().unwrap_or(String::from("24px")),
            width: props.size.unwrap_or(String::from("24px")),
            fill: props.color.unwrap_or(String::from("currentColor")),
            path { d: "m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z" }
        }
    }
}