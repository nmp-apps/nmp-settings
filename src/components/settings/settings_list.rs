use dioxus::prelude::*;

use crate::components::Setting;
use crate::stores::{ConfigStore, StoredSetting};

#[derive(PartialEq, Clone, Props)]
pub struct SettingsListProps {
    settings: Vec<StoredSetting>
}

#[component]
pub fn SettingsList(props: SettingsListProps) -> Element {
    let config_store = use_context::<Signal<ConfigStore>>();

    rsx! {
        ul { class: "settings-list",
            for setting in props.settings {
                li {
                    class: "settings-list__item",
                    class: if is_setting_hidden(&setting, config_store.read().is_advanced_settings()) { "hidden" },
                    Setting { setting: setting.clone() }
                }
            }
        }
    }
}

fn is_setting_hidden(setting: &StoredSetting, is_advanced_enabled: bool) -> bool {
    if is_advanced_enabled {
        false
    } else {
        setting.setting().is_advanced()
    }
}