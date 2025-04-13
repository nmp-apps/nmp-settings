use dioxus::prelude::*;

use crate::components::CategoryNavigation;
use crate::get_asset;


#[component]
pub fn LeftBar() -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/app/left_bar.css"));

    rsx! {
        div { class: "left-bar",
            document::Link { rel: "stylesheet", href: "{styles}" }
            CategoryNavigation {}
            hr { class: "left-bar__divider" }
        }
    }
}