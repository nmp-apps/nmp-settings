use dioxus::prelude::*;

use crate::components::SettingsList;
use crate::models::SettingsCategory;
use crate::stores::SettingsStore;
use crate::get_asset;
#[derive(PartialEq, Props, Clone)]
pub struct CategorizedSettingsProps {
    category_name: String,
}

#[component]
pub fn CategorizedSettings(props: CategorizedSettingsProps) -> Element {
    let settings_store = use_context::<SettingsStore>();
    let styles: String = use_hook(|| get_asset!("/assets/styles/views/categorized_settings.css"));

    let categorized_settings = settings_store.categorized_settings()();
    let category_settings = categorized_settings.get(&SettingsCategory::get_by_id(props.category_name.as_str()));
    let category_settings = match category_settings {
        Some(list) => list.clone(),
        None => vec![],
    };

    let title = props.category_name.clone().replace("_", " ");

    rsx! {
        div { class: "categorized-settings",
            document::Stylesheet { href: "{styles}" }

            div { class: "categorized-settings__container",
                h1 { class: "categorized-settings__title", "{title}" }
                SettingsList { settings: category_settings }
            }
        }
    }
}
