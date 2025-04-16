use dioxus::{document, prelude::*};

use crate::components::{ButtonGroup, Number, Select, Slider, Text, Switch};
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
            SettingComponent::ButtonGroup(data) => rsx! {
                ButtonGroup {}
            },
            SettingComponent::MultiSelect(data) => rsx! {
                Select {}
            },
            SettingComponent::Number(data) => rsx! {
                Number {}
            },
            SettingComponent::Select(data) => rsx! {
                Select {}
            },
            SettingComponent::Slider(data) => rsx! {
                Slider {}
            },
            SettingComponent::Switch(data) => rsx! {
                Switch {}
            },
            SettingComponent::Text(data) => rsx! {
                Text {}
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