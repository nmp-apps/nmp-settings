use dioxus::prelude::*;
use crate::router::Route;

use crate::components::TopBar;

const STYLES: Asset = asset!("/assets/styles/main_layout.css");

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLES }

        TopBar {}

        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
        }

        Outlet::<Route> {}
    }
}
