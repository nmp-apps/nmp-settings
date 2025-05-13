use dioxus::prelude::*;

use crate::components::SettingsList;
use crate::stores::SettingsStore;
#[derive(PartialEq, Props, Clone)]
pub struct UncategorizedSettingsProps {
    plugin_name: String,
}

#[component]
pub fn UncategorizedSettings(props: UncategorizedSettingsProps) -> Element {
    let settings_store = use_context::<SettingsStore>();

    let uncategorized_settings = settings_store.uncategorized_settings()();
    let plugin_settings = uncategorized_settings.get(&props.plugin_name);
    let plugin_settings = match plugin_settings {
        Some(list) => list.clone(),
        None => vec![],
    };

    rsx! {
        div { class: "categorized-settings",
            div { class: "categorized-settings__container",
                h1 { class: "categorized-settings__title", "{props.plugin_name}" }
                SettingsList { settings: plugin_settings }
            }
        }
    }
}
