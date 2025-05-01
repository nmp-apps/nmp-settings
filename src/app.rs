use dioxus::prelude::*;

use crate::get_asset;
use crate::stores::{use_app_store, use_plugins_store, use_settings_store};
use crate::router::Route;

#[component]
pub fn App() -> Element {
  let favicon: String = use_hook(|| get_asset!("/assets/favicon.ico"));
  let reset_css: String = use_hook(|| get_asset!("/assets/styles/reset.css"));
  let theme_css: String = use_hook(|| get_asset!("/assets/styles/theme.css"));
  let main_css: String = use_hook(|| get_asset!("/assets/styles/main.css"));

  use_app_store();
  use_plugins_store();
  use_settings_store();

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: "{favicon}" }
        document::Stylesheet { href: "{reset_css}" }
        document::Stylesheet { href: "{theme_css}" }
        document::Stylesheet { href: "{main_css}" }
        Router::<Route> {}
    }
}