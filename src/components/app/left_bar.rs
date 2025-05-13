use dioxus::prelude::*;

use crate::components::{CategoryNavigation, PluginCategoryNavigation};


#[component]
pub fn LeftBar() -> Element {

    rsx! {
        div { class: "left-bar",
            CategoryNavigation {}
            hr { class: "left-bar__divider" }
            PluginCategoryNavigation {}
        }
    }
}