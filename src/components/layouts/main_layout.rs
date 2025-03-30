use dioxus::prelude::*;
use crate::router::Route;

use crate::components::{LeftBar, TopBar};

const STYLES: Asset = asset!("/assets/styles/layouts/main_layout.css");

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLES }

        TopBar {}

        div { class: "main-layout__wrapper",
            LeftBar {}
            Outlet::<Route> {}
        }
    }
}
