use std::ops::Deref;

use dioxus::logger::tracing::{error, trace};
use dioxus::{document, prelude::*};

use crate::components::{ButtonGroup, ButtonGroupItem, Number, Select, SelectItem, SelectValue, Slider, Switch, Text};
use crate::get_asset;
use crate::models::SettingComponent;
use crate::stores::{PluginsStore, SettingsStore, StoredSetting};

#[derive(PartialEq, Props, Clone)]
pub struct SettingProps {
    setting: ReadOnlySignal<StoredSetting>
}

/// Main Setting component
#[component]
pub fn Setting(props: SettingProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/settings/setting.css"));
    let settings_store = use_context::<SettingsStore>();
    let plugins_store = use_context::<PluginsStore>();

    let mut handler_settings_store = settings_store.clone();
    let handler_plugins_store = plugins_store.clone();
    let button_group_handler = move |new_value: String| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        handler_settings_store.mutate_setting_value(
            setting,
            |setting: &mut StoredSetting| {
                match setting.setting_mut().component_mut() {
                    SettingComponent::ButtonGroup(component) => {
                        trace!("Button Group changed: new value is {}, old value is {}", new_value, component.value());
                        component.set_value(new_value.clone());
                    },
                    _ => (),
                }
            }
        );
        handler_settings_store.mutate_changed_settings(
            setting,
            &handler_plugins_store,
            |plugin_setting_component| {
                match plugin_setting_component {
                    SettingComponent::ButtonGroup(plugin_component) => {
                        *plugin_component.value() == new_value      
                    },
                    _ => false
                }
            }
        );
    };

    let mut handler_settings_store = settings_store.clone();
    let handler_plugins_store = plugins_store.clone();
    let number_handler = move |new_value: f64| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        handler_settings_store.mutate_setting_value(
            setting,
            |setting: &mut StoredSetting| {
                match setting.setting_mut().component_mut() {
                    SettingComponent::Number(component) => {
                        trace!("Number changed: new value is {}, old value is {}", new_value, component.value());
                        component.set_value(new_value);
                    },
                    _ => (),
                }
            }
        );
        handler_settings_store.mutate_changed_settings(
            setting,
            &handler_plugins_store,
            |plugin_setting_component| {
                match plugin_setting_component {
                    SettingComponent::Number(plugin_component) => {
                        plugin_component.value() == new_value      
                    },
                    _ => false
                }
            }
        );
    };

    let mut handler_settings_store = settings_store.clone();
    let handler_plugins_store = plugins_store.clone();
    let select_handler = move |new_value: SelectValue<String>| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        let new_value_cloned = new_value.clone();
        handler_settings_store.mutate_setting_value(
            setting,
            |setting: &mut StoredSetting| {
                match setting.setting_mut().component_mut() {
                    SettingComponent::Select(component) => {
                        match new_value_cloned {
                            SelectValue::Single(new_value) => {
                                trace!("Slider changed: new value is {:?}, old value is {:?}", new_value, component.value());
                                component.set_value(new_value);
                            },
                            SelectValue::Multiple(_) => {
                                error!("Value can't be multiple in select");
                            }
                        }
                    },
                    SettingComponent::MultiSelect(component) => {
                        match new_value_cloned {
                            SelectValue::Multiple(new_value) => {
                                trace!("Slider changed: new value is {:?}, old value is {:?}", new_value, component.value());
                                component.set_value(new_value);
                            },
                            SelectValue::Single(_) => {
                                error!("Value can't be single in multiselect");
                            }
                        }
                    },
                    _ => (),
                }
            }
        );
        handler_settings_store.mutate_changed_settings(
            setting,
            &handler_plugins_store,
            |plugin_setting_component| {
                match plugin_setting_component {
                    SettingComponent::Select(plugin_component) => {
                        match &new_value {
                            SelectValue::Single(new_value) => {
                                *plugin_component.value() == *new_value 
                            },
                            SelectValue::Multiple(_) => {
                                error!("Value can't be multiple in select");
                                false
                            }
                        }
                    },
                    SettingComponent::MultiSelect(plugin_component) => {
                        match &new_value {
                            SelectValue::Multiple(new_value) => {
                                let mut result = true;
                                if plugin_component.value().len() != new_value.len() {
                                    result = false;
                                }
                                plugin_component.value().iter().for_each(|v| {
                                    if !new_value.contains(v) {
                                        result = false;
                                    }
                                });
                                return result || false;
                            },
                            SelectValue::Single(_) => {
                                error!("Value can't be single in multiselect");
                                false
                            }
                        }
                    },
                    _ => false
                }
            }
        );
    };

    let mut handler_settings_store = settings_store.clone();
    let handler_plugins_store = plugins_store.clone();
    let slider_handler = move |new_value: f64| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        handler_settings_store.mutate_setting_value(
            setting,
            |setting: &mut StoredSetting| {
                match setting.setting_mut().component_mut() {
                    SettingComponent::Slider(component) => {
                        trace!("Slider changed: new value is {}, old value is {}", new_value, component.value());
                        component.set_value(new_value);
                    },
                    _ => (),
                }
            }
        );
        handler_settings_store.mutate_changed_settings(
            setting,
            &handler_plugins_store,
            |plugin_setting_component| {
                match plugin_setting_component {
                    SettingComponent::Slider(plugin_component) => {
                        plugin_component.value() == new_value      
                    },
                    _ => false
                }
            }
        );
    };

    let mut handler_settings_store = settings_store.clone();
    let handler_plugins_store = plugins_store.clone();
    let switch_handler = move |new_value: bool| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        handler_settings_store.mutate_setting_value(
            setting,
            |setting: &mut StoredSetting| {
                match setting.setting_mut().component_mut() {
                    SettingComponent::Switch(component) => {
                        trace!("Switch changed: new value is {}, old value is {}", new_value, component.value());
                        component.set_value(new_value);
                    },
                    _ => (),
                }
            }
        );
        handler_settings_store.mutate_changed_settings(
            setting,
            &handler_plugins_store,
            |plugin_setting_component| {
                match plugin_setting_component {
                    SettingComponent::Switch(plugin_component) => {
                        plugin_component.value() == new_value      
                    },
                    _ => false
                }
            }
        );
    };

    let mut handler_settings_store = settings_store.clone();
    let handler_plugins_store = plugins_store.clone();
    let text_handler = move |new_value: String| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        let new_value_cloned = new_value.clone();
        handler_settings_store.mutate_setting_value(
            setting,
            |setting: &mut StoredSetting| {
                match setting.setting_mut().component_mut() {
                    SettingComponent::Text(component) => {
                        trace!("Text changed: new value is {}, old value is {}", new_value_cloned, component.value());
                        component.set_value(new_value_cloned);
                    },
                    _ => (),
                }
            }
        );
        handler_settings_store.mutate_changed_settings(
            setting,
            &handler_plugins_store,
            |plugin_setting_component| {
                match plugin_setting_component {
                    SettingComponent::Text(plugin_component) => {
                        *plugin_component.value() == new_value      
                    },
                    _ => false
                }
            }
        );
    };

    let component = use_memo(move || {
        match props.setting.read().setting().component() {
            SettingComponent::ButtonGroup(data) => {
                let converted_items: Vec<ButtonGroupItem> = data.items().iter().map(|item| ButtonGroupItem::new(item.text().clone(), item.value().clone())).collect();
                let mut handler = button_group_handler.clone();
                
                rsx! {
                    ButtonGroup {
                        onchange: move |new_value| handler(new_value),
                        items: converted_items,
                        value: data.value(),
                    }
                }
            },
            SettingComponent::MultiSelect(data) => {
                let converted_items: Vec<SelectItem<String>> = data.items().iter().map(|item| {
                    SelectItem::new(item.text().clone(), item.value().clone())
                }).collect();
                let mut handler = select_handler.clone();

                rsx! {
                    Select {
                        multiple: true,
                        items: converted_items,
                        value: SelectValue::Multiple(data.value().clone()),
                        onclick: move |v| handler(v),
                    }
                }
            },
            SettingComponent::Number(data) => {
                let mut handler = number_handler.clone();
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
                let mut handler = select_handler.clone();

                rsx! {
                    Select {
                        items: new_vec,
                        value: SelectValue::Single(data.value().clone()),
                        onclick: move |v| handler(v),
                    }
                }
            },
            SettingComponent::Slider(data) => {
                let mut handler = slider_handler.clone();
                
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
            SettingComponent::Switch(data) => {
                let mut handler = switch_handler.clone();

                rsx! {
                    Switch {
                        onchange: move |evt: Event<FormData>| {
                            let value: bool = evt.data.value().parse().unwrap_or(false);
                            handler(value);
                        },
                        value: data.value(),
                    }
                }
            },
            SettingComponent::Text(data) => {
                let mut handler = text_handler.clone();

                rsx! {
                    Text {
                        oninput: move |new_value| handler(new_value),
                        value: data.value(),
                        min_length: data.min_length(),
                        max_length: data.max_length(),
                    }
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
