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
use crate::stores::StoredSetting;

#[derive(PartialEq, Props, Clone)]
pub struct SettingProps {
    setting: ReadOnlySignal<StoredSetting>
}

/// Main Setting component
#[component]
pub fn Setting(props: SettingProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/settings/setting.css"));

    let component = use_memo(move || {
        match props.setting.read().setting().component() {
            SettingComponent::ButtonGroup(data) => {
                rsx! {
                    ButtonGroupSetting { data: data.clone(), setting: props.setting }
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
                    SwitchSetting { data: data.clone(), setting: props.setting }
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
