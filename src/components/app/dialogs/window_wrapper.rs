use dioxus::prelude::*;

use crate::get_asset;

#[derive(PartialEq, Clone, Props)]
pub struct WindowWrapperProps {
    children: Element,
}

/// Wrapper over children window component. Provide default styles
#[component]
pub fn WindowWrapper(props: WindowWrapperProps) -> Element {
    let reset_css: String = use_hook(|| get_asset!("/assets/styles/reset.css"));
    let theme_css: String = use_hook(|| get_asset!("/assets/styles/theme.css"));
    let main_css: String = use_hook(|| get_asset!("/assets/styles/main.css"));
    rsx! {
        div {
            document::Stylesheet { href: "{reset_css}", rel: "preload" }
            document::Stylesheet { href: "{theme_css}", rel: "preload" }
            document::Stylesheet { href: "{main_css}", rel: "preload" }

            {props.children}
        }
    }
}