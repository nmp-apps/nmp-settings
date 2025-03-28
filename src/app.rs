use dioxus::prelude::*;

use crate::router::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const RESET_CSS: Asset = asset!("/assets/styles/reset.css");
const THEME_CSS: Asset = asset!("/assets/styles/theme.css");
const MAIN_CSS: Asset = asset!("/assets/styles/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
      // Global app resources
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "stylesheet", href: RESET_CSS }
      document::Link { rel: "stylesheet", href: THEME_CSS }
      document::Link { rel: "stylesheet", href: MAIN_CSS }


      Router::<Route> {}
    }
}