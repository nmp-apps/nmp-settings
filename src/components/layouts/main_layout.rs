use dioxus::prelude::*;

use crate::get_asset;
use crate::router::Route;
use crate::components::{LeftBar, Notifications, TopBar};


#[component]
pub fn MainLayout() -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/layouts/main_layout.css"));
    rsx! {
        document::Link { rel: "stylesheet", href: "{styles}" }

        Notifications {}

        TopBar {}

        div { class: "main-layout__wrapper",
            LeftBar {}
            Outlet::<Route> {}
        }
    }
}
