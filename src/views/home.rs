use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "home",

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
