use dioxus::logger::tracing::info;
use dioxus::prelude::*;

use crate::components::Setting;
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
    let appearance_settings = categorized_settings.get(&SettingsCategory::get_by_id(props.category_name.as_str()));
    let category_settings = match appearance_settings {
        Some(list) => list.iter().filter_map(|v| {
            match v.setting().category() {
                Some(settings_category) => {
                    if *settings_category == SettingsCategory::get_by_id(props.category_name.as_str()) {
                        Some(v)
                    } else {
                        None
                    }
                },
                None => None 
            }
        }).collect(),
        None => vec![],
    };

    let title = props.category_name.clone().replace("_", " ");

    rsx! {
        div { class: "categorized-settings",
            document::Stylesheet { href: "{styles}" }

            div { class: "categorized-settings__container",
                h1 { class: "categorized-settings__title", "{title}" }
                ul { class: "categorized-settings__list",
                    for setting in category_settings {
                        li { class: "categorized-settings__list-item",
                            Setting { setting: setting.clone() }
                        }
                        li { class: "categorized-settings__list-item",
                            Setting { setting: setting.clone() }
                        }
                    }
                }
            }
        }
    }
}
