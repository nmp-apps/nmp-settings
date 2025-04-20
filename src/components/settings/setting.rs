use std::ops::Deref;
use std::rc::Rc;

use dioxus::logger::tracing::{error, trace};
use dioxus::{document, prelude::*};

use crate::components::settings::setting;
use crate::components::{ButtonGroup, Number, Select, SelectItem, SelectValue, Slider, Switch, Text};
use crate::get_asset;
use crate::models::SettingComponent;
use crate::stores::{SettingsStore, StoredSetting};

#[derive(PartialEq, Props, Clone)]
pub struct SettingProps {
    setting: ReadOnlySignal<StoredSetting>
}

/// Main Setting component
#[component]
pub fn Setting(props: SettingProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/settings/setting.css"));
    let settings_store = use_context::<SettingsStore>();

    let number_handler_store = settings_store.clone();
    let number_handler = move |new_value: f64| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        let mut categorized_settings_signal = number_handler_store.categorized_settings(); 
        let categorized_settings = &mut categorized_settings_signal.write();
        let target_category = setting.setting().category().as_ref().unwrap_or_else(|| {
            panic!("Target category for changing setting value is None");
        });
        let category_list = categorized_settings.get_mut(&target_category);
        match category_list {
            Some(list) => {
                let found_setting = list.iter_mut().find(|stored_setting| {
                    stored_setting.id() == setting.id() && stored_setting.setting().component() == setting.setting().component()
                });
                match found_setting {
                    Some(stored_setting) => {
                        match stored_setting.setting_mut().component_mut() {
                            SettingComponent::Number(component) => {
                                component.set_value(new_value);
                                trace!("Number changed: new value is {}, old value is {}", new_value, component.value());

                            },
                            _ => (),
                        }
                    },
                    None => {
                        error!("Can't find setting by id {}", setting.id());
                        return
                    }
                }
            },
            None => {
                error!("Can't find list of settings by category \"{}\"", target_category.get_name());
            }
        }
    };

    let slider_handler_store = settings_store.clone();
    let slider_handler = move |new_value: f64| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        let mut categorized_settings_signal = slider_handler_store.categorized_settings(); 
        let categorized_settings = &mut categorized_settings_signal.write();
        let target_category = setting.setting().category().as_ref().unwrap_or_else(|| {
            panic!("Target category for changing setting value is None");
        });
        let category_list = categorized_settings.get_mut(&target_category);
        match category_list {
            Some(list) => {
                let found_setting = list.iter_mut().find(|stored_setting| {
                    stored_setting.id() == setting.id() && stored_setting.setting().component() == setting.setting().component()
                });
                match found_setting {
                    Some(stored_setting) => {
                        match stored_setting.setting_mut().component_mut() {
                            SettingComponent::Slider(component) => {
                                component.set_value(new_value);
                                trace!("Slider changed: new value is {}, old value is {}", new_value, component.value());

                            },
                            _ => (),
                        }
                    },
                    None => {
                        error!("Can't find setting by id {}", setting.id());
                        return
                    }
                }
            },
            None => {
                error!("Can't find list of settings by category \"{}\"", target_category.get_name());
            }
        }
    };

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
            SettingComponent::Number(data) => {
                let handler = number_handler.clone();
                rsx! {
                    Number {
                        min: data.min(),
                        max: data.max(),
                        step: data.step(),
                        value: data.value(),
                        oninput: move |new_value| handler(new_value),
                    }
                }
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
            SettingComponent::Slider(data) => {
                let handler = slider_handler.clone();
                
                rsx! {
                    Slider {
                        oninput: move |new_value| handler(new_value),
                        min: data.min(),
                        max: data.max(),
                        step: data.step(),
                        value: data.value(),
                    }
                }
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
}