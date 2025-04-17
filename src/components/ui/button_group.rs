use dioxus::prelude::*;

use crate::{components::Button, get_asset};

#[component]
pub fn ButtonGroup() -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/ui/button_group.css"));

    rsx! {
        div { class: "ui-button-group",
            document::Stylesheet { href: "{styles}" }

            Button { "Option1" }
            Button { "Option2" }
            Button { "Option3" }
        }
    }
}