use dioxus::prelude::*;

use crate::components::{Button, Switch};

const STYLES: Asset = asset!("/assets/styles/top_bar.css");

#[component]
pub fn TopBar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLES }

        div { class: "top-bar",
            div { class: "top-bar__left" }
            div { class: "top-bar__center",
                h1 { class: "top-bar__title", "Settings" }
            }
            div { class: "top-bar__right",
                Switch {}
                Button { "Save" }
            }
        }
    }
}