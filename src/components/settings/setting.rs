use dioxus::prelude::*;

use crate::components::{
    ButtonGroupSetting, IconButton, NumberSetting, SelectSetting, SelectSettingData, SliderSetting, SwitchSetting, TextSetting
};
use crate::models::{Icon, SettingComponent};
use crate::stores::{PluginsStore, StoredSetting};

#[derive(PartialEq, Props, Clone)]
pub struct SettingProps {
    setting: ReadSignal<StoredSetting>
}

/// Main Setting component
#[component]
pub fn Setting(props: SettingProps) -> Element {
    let plugins_store = use_context::<PluginsStore>();
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
                        disabled: is_disabled(),
                    }
                }
            },
            SettingComponent::MultiSelect(data) => {
                rsx! {
                    SelectSetting {
                        data: SelectSettingData::Multiple(data.clone()),
                        setting: props.setting,
                        disabled: is_disabled(),
                    }
                }
            },
            SettingComponent::Number(data) => {
                rsx! {
                    NumberSetting {
                        data: data.clone(),
                        setting: props.setting,
                        disabled: is_disabled(),
                    }
                }
            },
            SettingComponent::Select(data) => {
                rsx! {
                    SelectSetting {
                        data: SelectSettingData::Single(data.clone()),
                        setting: props.setting,
                        disabled: is_disabled(),
                    }
                }
            },
            SettingComponent::Slider(data) => {
                rsx! {
                    SliderSetting {
                        data: data.clone(),
                        setting: props.setting,
                        disabled: is_disabled(),
                    }
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
                    TextSetting {
                        data: data.clone(),
                        setting: props.setting,
                        disabled: is_disabled(),
                    }
                }
            },
        }
    });

    rsx! {
        div { class: "setting", class: if is_disabled() { "disabled" },
            div { class: "setting__content",
                div { class: "setting__title",
                    h6 { {format!("{}", props.setting.read().setting().title())} }
                    IconButton {
                        class: "setting__plugin-icon",
                        icon: Icon::Info,
                        title: props.setting.read().owner().clone(),
                        disabled: true,
                        size: 18,
                    }
                }
                if props.setting.read().setting().description().len() > 0 {
                    p { class: "setting__description",
                        {format!("{}", props.setting.read().setting().description())}
                    }
                }
            }

            div { class: "setting__actions", {component} }
        }
    }
}
