use std::ops::Deref;

use dioxus::logger::tracing::trace;
use dioxus::prelude::*;

use crate::components::Number as UiNumber;
use crate::models::{Number, SettingComponent};
use crate::stores::{PluginsStore, SettingsStore, StoredSetting};

#[derive(PartialEq, Clone, Props)]
pub struct NumberSettingProps {
    setting: ReadOnlySignal<StoredSetting>,
    data: ReadOnlySignal<Number>
}

#[component]
pub fn NumberSetting(props: NumberSettingProps) -> Element {
    let mut settings_store = use_context::<SettingsStore>();
    let plugins_store = use_context::<PluginsStore>();

    let mut handler = move |new_value: f64| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        settings_store.mutate_setting_value(
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
        settings_store.mutate_changed_settings(
            setting,
            &plugins_store,
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

    rsx! {
        UiNumber {
            min: props.data.read().min(),
            max: props.data.read().max(),
            step: props.data.read().step(),
            value: props.data.read().value(),
            oninput: move |new_value| handler(new_value),
        }
    }
}