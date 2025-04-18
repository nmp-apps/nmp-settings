use dioxus::prelude::*;

use super::IconProps;

#[component]
pub fn BluetoothIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            class: props.class.unwrap_or(String::from("")),
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 -960 960 960",
            height: props.size.clone().unwrap_or(String::from("24px")),
            width: props.size.unwrap_or(String::from("24px")),
            fill: props.color.unwrap_or(String::from("currentColor")),
            path { d: "M450-136v-272L277-235q-9 9-21 9t-21-9q-9-9-9-21t9-21l203-203-203-203q-9-9-9-21t9-21q9-9 21-9t21 9l173 173v-272q0-14 9.5-22t20.5-8q5 0 10.5 2t10.5 7l172 172q5 5 7 10t2 11q0 6-2 11t-7 10L522-480l151 151q5 5 7 10t2 11q0 6-2 11t-7 10L501-115q-5 5-10.5 7t-10.5 2q-11 0-20.5-8t-9.5-22Zm60-416 100-100-100-98v198Zm0 342 100-98-100-100v198Z" }
        }
    }
}