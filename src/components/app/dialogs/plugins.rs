use dioxus::prelude::*;

use crate::stores::PluginsStore;

#[derive(PartialEq, Props, Clone)]
pub struct PluginsProps {
    plugins_store: PluginsStore
}

#[component]
pub fn Plugins(props: PluginsProps) -> Element {
    rsx! { "Plugins List Window" }
}