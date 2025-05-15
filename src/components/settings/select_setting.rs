use std::ops::Deref;

use dioxus::logger::tracing::{error, trace};
use dioxus::prelude::*;

use crate::components::{Select, SelectItem, SelectValue};
use crate::hooks::use_confirmation_window;
use crate::stores::{PluginsStore, SettingsStore, StoredSetting};
use crate::models::{MultiSelect, Select as SelectModel, SettingComponent};
use crate::utils::confirmation_window_handler;

#[derive(PartialEq, Clone, Props)]
pub struct SelectSettingProps {
    setting: ReadOnlySignal<StoredSetting>,
    data: ReadOnlySignal<SelectSettingData>,
    disabled: ReadOnlySignal<Option<bool>>
}

pub enum SelectSettingData {
    Single(SelectModel),
    Multiple(MultiSelect)
}

#[component]
pub fn SelectSetting(props: SelectSettingProps) -> Element {
    let settings_store = use_context::<SettingsStore>();
    let plugins_store = use_context::<PluginsStore>();
    
    let converted_items: Memo<Vec<SelectItem<String>>> = use_memo(move || {
        let data = props.data.read();
        match data.deref() {
            SelectSettingData::Multiple(data) => {
                data.items().iter().map(|item| {
                    SelectItem::new(item.text().clone(), item.value().clone())
                }).collect()
            },
            SelectSettingData::Single(data) => {
                data.items().iter().map(|item| {
                    SelectItem::new(item.text().clone(), item.value().clone())
                }).collect()
            }
        }
    });

    let check_confirmation = {
        let store = settings_store.clone();
        use_confirmation_window(
            props.setting.read().setting().confirmation(),
            Callback::new(move |answer: bool| confirmation_window_handler(answer, props.setting.read().clone(), store.to_owned()))
        )
    };

    let mut handler = {
        let mut settings_store = settings_store.clone();
        move |new_value: SelectValue<String>| {
            let setting_ref = props.setting.read();
            let setting = setting_ref.deref();
            let new_value_cloned = new_value.clone();
            settings_store.mutate_setting_value(
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
            settings_store.mutate_changed_settings(
                setting,
                &plugins_store,
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
        }
    };

    rsx! {
        {
            let data = props.data.read();
            match data.deref() {
                SelectSettingData::Multiple(data) => {
                    rsx! {
                        Select {
                            multiple: true,
                            items: converted_items,
                            value: SelectValue::Multiple(data.value().clone()),
                            onclick: {
                                let confirmed_setting_ids = settings_store
                                    .approved_confirmation_setting_ids()
                                    .read()
                                    .clone();
                                move |v| {
                                    if !confirmed_setting_ids.contains(&props.setting.read().id().clone())
                                        && !check_confirmation()
                                    {
                                        return;
                                    }
                                    handler(v)
                                }
                            },
                            disabled: match *props.disabled.read() {
                                Some(v) => v,
                                None => false,
                            },
                        }
                    }
                }
                SelectSettingData::Single(data) => {
                    rsx! {
                        Select {
                            items: converted_items,
                            value: SelectValue::Single(data.value().clone()),
                            onclick: {
                                let confirmed_setting_ids = settings_store
                                    .approved_confirmation_setting_ids()
                                    .read()
                                    .clone();
                                move |v| {
                                    if !confirmed_setting_ids.contains(&props.setting.read().id().clone())
                                        && !check_confirmation()
                                    {
                                        return;
                                    }
                                    handler(v)
                                }
                            },
                            disabled: match *props.disabled.read() {
                                Some(v) => v,
                                None => false,
                            },
                        }
                    }
                }
            }
        }
    }
}