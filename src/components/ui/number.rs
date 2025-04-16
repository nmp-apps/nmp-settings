use dioxus::{document, prelude::*};

use crate::get_asset;

#[component]
pub fn Number() -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/ui/number.css"));
    rsx! {
        div { class: "ui-number",
            document::Stylesheet { href: "{styles}" }

            button { class: "ui-number__step-button left", "<" }
            input { class: "ui-number__input", r#type: "number" }
            button { class: "ui-number__step-button right", ">" }
        }
    }
}