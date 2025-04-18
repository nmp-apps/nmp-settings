use dioxus::prelude::*;

use super::IconProps;

#[component]
pub fn KeyboardArrowUpIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            class: props.class.unwrap_or(String::from("")),
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 -960 960 960",
            height: props.size.clone().unwrap_or(String::from("24px")),
            width: props.size.unwrap_or(String::from("24px")),
            fill: props.color.unwrap_or(String::from("currentColor")),
            path { d: "M480-528 324-372q-11 11-28 11t-28-11q-11-11-11-28t11-28l184-184q12-12 28-12t28 12l184 184q11 11 11 28t-11 28q-11 11-28 11t-28-11L480-528Z" }
        }
    }
}