use dioxus::prelude::*;

use crate::components::CategoryNavigation;

const STYLES: Asset = asset!("/assets/styles/app/left_bar.css");

#[component]
pub fn LeftBar() -> Element {
    rsx! {
        div { class: "left-bar",

            document::Link { rel: "stylesheet", href: STYLES }
            CategoryNavigation {}
            hr { class: "left-bar__divider" }
        }
    }
}