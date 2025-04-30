use std::ops::Deref;

use dioxus::logger::tracing::trace;
use dioxus::prelude::*;

use crate::components::Switch as UiSwitch;
use crate::models::{Switch, SettingComponent};
use crate::stores::{PluginsStore, SettingsStore, StoredSetting};

#[derive(PartialEq, Clone, Props)]
pub struct SwitchSettingProps {
    setting: ReadOnlySignal<StoredSetting>,
    data: ReadOnlySignal<Switch>,
    disabled: ReadOnlySignal<Option<bool>>
}

#[component]
pub fn SwitchSetting(props: SwitchSettingProps) -> Element {
    let mut settings_store = use_context::<SettingsStore>();
    let plugins_store = use_context::<PluginsStore>();

    let mut handler = move |new_value: bool| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        settings_store.mutate_setting_value(
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
        settings_store.mutate_changed_settings(
            setting,
            &plugins_store,
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

    rsx! {
        UiSwitch {
            onchange: move |evt: Event<FormData>| {
                let value: bool = evt.data.value().parse().unwrap_or(false);
                handler(value);
            },
            value: props.data.read().value(),
            disabled: match *props.disabled.read() {
                Some(v) => v,
                None => false,
            },
        }
    }
}