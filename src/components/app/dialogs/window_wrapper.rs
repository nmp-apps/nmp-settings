use dioxus::prelude::*;

use crate::components::StylesLoader;

#[derive(PartialEq, Clone, Props)]
pub struct WindowWrapperProps {
    children: Element,
}

/// Wrapper over children window component. Provide default styles
#[component]
pub fn WindowWrapper(props: WindowWrapperProps) -> Element {
    rsx! {
        div { class: "dialog-window-wrapper",
            StylesLoader {}
            {props.children}
        }
    }
}