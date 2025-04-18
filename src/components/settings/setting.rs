use dioxus::logger::tracing::info;
use dioxus::{document, prelude::*};

use crate::components::{ButtonGroup, Number, Select, SelectItem, SelectValue, Slider, Switch, Text};
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
            SettingComponent::MultiSelect(data) => {
                let new_vec: Vec<SelectItem<String>> = data.items().iter().map(|item| {
                    SelectItem::new(item.text().clone(), item.value().clone())
                }).collect();

                rsx! {
                    Select {
                        multiple: true,
                        items: new_vec,
                        value: SelectValue::Multiple(data.value().clone()),
                        onclick: move |v| select_handler(v),
                    }
                }
            },
            SettingComponent::Number(data) => rsx! {
                Number {}
            },
            SettingComponent::Select(data) => {
                let new_vec: Vec<SelectItem<String>> = data.items().iter().map(|item| {
                    SelectItem::new(item.text().clone(), item.value().clone())
                }).collect();

                rsx! {
                    Select {
                        items: new_vec,
                        value: SelectValue::Single(Some(data.value().clone())),
                        onclick: move |v| select_handler(v),
                    }
                }
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

fn select_handler<T: std::fmt::Debug>(selected_value: SelectValue<T>) {
    println!("{:?}", selected_value);
    info!("{:?}", selected_value);
}