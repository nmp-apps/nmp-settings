use dioxus::prelude::*;

use crate::components::Setting;
use crate::get_asset;
use crate::stores::{AppStore, StoredSetting};

#[derive(PartialEq, Clone, Props)]
pub struct SettingsListProps {
    settings: Vec<StoredSetting>
}

#[component]
pub fn SettingsList(props: SettingsListProps) -> Element {
    let app_store = use_context::<Signal<AppStore>>();
    let styles: String = use_hook(|| get_asset!("/assets/styles/settings/settings_list.css"));

    rsx! {
        ul { class: "settings-list",
            document::Stylesheet { href: "{styles}" }

            for setting in props.settings {
                li {
                    class: "settings-list__item",
                    class: if is_setting_hidden(&setting, app_store.read().is_advanced_settings()) { "hidden" },
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