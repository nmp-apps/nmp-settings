use dioxus::prelude::*;

use crate::get_asset;

use crate::stores::{
  use_app_store,
  use_config_store,
  use_plugins_store,
  use_settings_store
};
use crate::router::Route;
use crate::components::StylesLoader;

#[component]
pub fn App() -> Element {
  let favicon: String = use_hook(|| get_asset!("/assets/favicon.ico"));

  use_app_store();
  use_config_store();
  use_plugins_store();
  use_settings_store();

    rsx! {
      // Global app resources
      document::Link { rel: "icon", href: "{favicon}" }
      StylesLoader {}
      Router::<Route> {}
    }
}