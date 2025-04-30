use dioxus::{document, prelude::*};

use crate::components::{
    ButtonGroupSetting,
    NumberSetting,
    SelectSetting,
    SelectSettingData,
    SliderSetting,
    SwitchSetting,
    TextSetting
};
use crate::get_asset;
use crate::models::SettingComponent;
use crate::stores::{PluginsStore, StoredSetting};

#[derive(PartialEq, Props, Clone)]
pub struct SettingProps {
    setting: ReadOnlySignal<StoredSetting>
}

/// Main Setting component
#[component]
pub fn Setting(props: SettingProps) -> Element {
    let plugins_store = use_context::<PluginsStore>();
    let styles: String = use_hook(|| get_asset!("/assets/styles/settings/setting.css"));
    let is_disabled = use_memo(move || {
        let disabled_plugins = plugins_store.get_disabled_plugins().read().clone();
        let found_plugin = disabled_plugins.iter().find(|disabled_plugin| {
            let setting = props.setting.read();
            let setting_plugin_name = setting.owner();
            let disabled_plugin_name = disabled_plugin.name();
            setting_plugin_name == disabled_plugin_name
        });
        if let Some(_) = found_plugin {
            true
        } else {
            false
        }
    });

    let component = use_memo(move || {
        match props.setting.read().setting().component() {
            SettingComponent::ButtonGroup(data) => {
                rsx! {
                    ButtonGroupSetting {
                        data: data.clone(),
                        setting: props.setting,
                        disabled: is_disabled,
                    }
                }
            },
            SettingComponent::MultiSelect(data) => {
                rsx! {
                    SelectSetting {
                        data: SelectSettingData::Multiple(data.clone()),
                        setting: props.setting,
                    }
                }
            },
            SettingComponent::Number(data) => {
                rsx! {
                    NumberSetting { data: data.clone(), setting: props.setting }
                }
            },
            SettingComponent::Select(data) => {
                rsx! {
                    SelectSetting {
                        data: SelectSettingData::Single(data.clone()),
                        setting: props.setting,
                    }
                }
            },
            SettingComponent::Slider(data) => {
                rsx! {
                    SliderSetting { data: data.clone(), setting: props.setting }
                }
            },
            SettingComponent::Switch(data) => {
                rsx! {
                    SwitchSetting {
                        data: data.clone(),
                        setting: props.setting,
                        disabled: is_disabled(),
                    }
                }
            },
            SettingComponent::Text(data) => {
                rsx! {
                    TextSetting { data: data.clone(), setting: props.setting }
                }
            },
        }
    });

    rsx! {
        div { class: "setting",
            document::Stylesheet { href: "{styles}" }

            div { class: "setting__content",
                h6 { {format!("{}", props.setting.read().setting().title())} }
                if props.setting.read().setting().description().len() > 0 {
                    p { {format!("{}", props.setting.read().setting().description())} }
                }
            }

            div { class: "setting__actions", {component} }
        }
    }
}
