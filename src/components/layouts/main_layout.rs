use dioxus::prelude::*;

use crate::get_asset;
use crate::router::Route;
use crate::components::{LeftBar, TopBar};


#[component]
pub fn MainLayout() -> Element {
    let styles: String = get_asset!("/assets/styles/layouts/main_layout.css");
    rsx! {
        document::Link { rel: "stylesheet", href: styles }

        TopBar {}

        div { class: "main-layout__wrapper",
            LeftBar {}
            Outlet::<Route> {}
        }
    }
}
