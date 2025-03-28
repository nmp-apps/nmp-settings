use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SwitchProps {
    #[props(default = false)]
    value: bool,
    onchange: Option<EventHandler<MouseEvent>>
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    rsx! { "Switch" }
}