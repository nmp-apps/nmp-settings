use dioxus::{document, prelude::*};

use crate::get_asset;

#[component]
pub fn Home() -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/views/home.css"));
    rsx! {
        div { class: "home",
            document::Stylesheet { href: "{styles}" }

            h1 { class: "home__title", "Settings" }
            p { class: "home__description", "All settings in one place. Extensible by plugins." }
            ul { class: "links",
                li {
                    a { href: "https://github.com/keiko37/nmp-settings", "Github" }
                }
            }
        }
    }
}
