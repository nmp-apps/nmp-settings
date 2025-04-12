use dioxus::prelude::*;

use crate::get_asset;
use crate::router::Route;

#[component]
pub fn App() -> Element {
  const FAVICON: Asset = asset!("/assets/favicon.ico");
  let reset_css: String = get_asset!("/assets/styles/reset.css");
  let theme_css: String = get_asset!("/assets/styles/theme.css");
  let main_css: String = get_asset!("/assets/styles/main.css");

    rsx! {
      // Global app resources
      document::Link { rel: "icon", href: FAVICON }
      document::Stylesheet { href: reset_css }
      document::Stylesheet { href: theme_css }
      document::Stylesheet { href: main_css }

      Router::<Route> {}
    }
}