use dioxus::prelude::*;

use crate::components::Setting;
use crate::get_asset;
use crate::stores::StoredSetting;

#[derive(PartialEq, Clone, Props)]
pub struct SettingsListProps {
    settings: Vec<StoredSetting>
}

#[component]
pub fn SettingsList(props: SettingsListProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/settings/settings_list.css"));

    rsx! {
        ul { class: "settings-list",
            document::Stylesheet { href: "{styles}" }

            for setting in props.settings {
                li { class: "settings-list__item",
                    Setting { setting: setting.clone() }
                }
            }
        }
    }
}