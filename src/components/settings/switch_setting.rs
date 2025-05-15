use std::ops::Deref;

use dioxus::logger::tracing::trace;
use dioxus::prelude::*;

use crate::components::Switch as UiSwitch;
use crate::hooks::use_confirmation_window;
use crate::models::{Switch, SettingComponent};
use crate::stores::{PluginsStore, SettingsStore, StoredSetting};
use crate::utils::confirmation_window_handler;

#[derive(PartialEq, Clone, Props)]
pub struct SwitchSettingProps {
    setting: ReadOnlySignal<StoredSetting>,
    data: ReadOnlySignal<Switch>,
    disabled: ReadOnlySignal<Option<bool>>
}

#[component]
pub fn SwitchSetting(props: SwitchSettingProps) -> Element {
    let settings_store = use_context::<SettingsStore>();
    let plugins_store = use_context::<PluginsStore>();

    let check_confirmation = {
        let store = settings_store.clone();
        use_confirmation_window(
            props.setting.read().setting().confirmation(),
            Callback::new(move |answer: bool| confirmation_window_handler(answer, props.setting.read().clone(), store.to_owned()))
        )
    };

    let mut handler = {
        let mut settings_store = settings_store.clone();
        move |new_value: bool| {
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
    }
    };

    rsx! {
        UiSwitch {
            onchange: {
                let confirmed_setting_ids = settings_store
                    .approved_confirmation_setting_ids()
                    .read()
                    .clone();
                move |new_value: bool| {
                    if !confirmed_setting_ids.contains(&props.setting.read().id().clone())
                        && !check_confirmation()
                    {
                        return;
                    }
                    handler(new_value);
                }
            },
            value: props.data.read().value(),
            disabled: match *props.disabled.read() {
                Some(v) => v,
                None => false,
            },
        }
    }
}