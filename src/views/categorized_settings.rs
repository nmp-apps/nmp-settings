use dioxus::prelude::*;

use crate::stores::PluginsStore;
use crate::get_asset;
#[derive(PartialEq, Props, Clone)]
pub struct CategorizedSettingsProps {
    category_name: String,
}

#[component]
pub fn CategorizedSettings(props: CategorizedSettingsProps) -> Element {
    let pluginsStore = use_context::<PluginsStore>();
    let styles: String = use_hook(|| get_asset!("/assets/styles/views/categorized_settings.css"));

    let title = props.category_name.clone().replace("_", " ");

    rsx! {
        div { class: "categorized-settings",
            document::Stylesheet { href: "{styles}" }

            h1 { class: "categorized-settings__title", "{title}" }
            p { {format!("{:?}", pluginsStore.get_plugins())} }
        }
    }
}
