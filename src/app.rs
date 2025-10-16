use dioxus::prelude::*;

use crate::stores::{
  use_app_store,
  use_config_store,
  use_plugins_store,
  use_settings_store
};
use crate::router::Route;
use crate::components::{StylesLoader, WindowResize};

static FAVICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn App() -> Element {
  use_app_store();
  use_config_store();
  use_plugins_store();
  use_settings_store();

    rsx! {
      // Global app resources
      document::Link { rel: "icon", href: "{FAVICON}" }
      StylesLoader {}
      WindowResize {}
      Router::<Route> {}
    }
}