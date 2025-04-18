use dioxus::prelude::*;

use super::IconProps;

#[component]
pub fn CheckIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            class: props.class.unwrap_or(String::from("")),
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 -960 960 960",
            height: props.size.clone().unwrap_or(String::from("24px")),
            width: props.size.unwrap_or(String::from("24px")),
            fill: props.color.unwrap_or(String::from("currentColor")),
            path { d: "M378-246 154-470l43-43 181 181 384-384 43 43-427 427Z" }
        }
    }
}

