use dioxus::prelude::*;

use crate::components::{Button, Switch};

const STYLES: Asset = asset!("/assets/styles/top_bar.css");

#[component]
pub fn TopBar() -> Element {
    let mut is_advanced_settings_enabled = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: STYLES }

        div { class: "top-bar",
            div { class: "top-bar__left" }
            div { class: "top-bar__center",
                h1 { class: "top-bar__title", "Settings" }
            }
            div { class: "top-bar__right",
                Switch {
                    title: "Enable advanced settings",
                    value: is_advanced_settings_enabled(),
                    onchange: move |event: Event<FormData>| is_advanced_settings_enabled.set(event.checked()),
                }
                Button { primary: true, "Save" }
            }
        }
    }
}