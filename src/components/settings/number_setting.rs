use std::ops::Deref;
use std::rc::Rc;

use dioxus::logger::tracing::trace;
use dioxus::prelude::*;

use crate::components::Number as UiNumber;
use crate::hooks::use_confirmation_window;
use crate::models::{Number, SettingComponent};
use crate::stores::{AppWindowName, PluginsStore, SettingsStore, StoredSetting};
use crate::utils::confirmation_window_handler;

#[derive(PartialEq, Clone, Props)]
pub struct NumberSettingProps {
    setting: ReadOnlySignal<StoredSetting>,
    data: ReadOnlySignal<Number>,
    disabled: ReadOnlySignal<Option<bool>>
}

#[component]
pub fn NumberSetting(props: NumberSettingProps) -> Element {
    let settings_store = use_context::<SettingsStore>();
    let plugins_store = use_context::<PluginsStore>();

    let check_confirmation = Rc::new({
        let store = settings_store.clone();
        use_confirmation_window(
            AppWindowName::SettingConfirmation,
            props.setting.read().setting().confirmation(),
            Callback::new(move |answer: bool| confirmation_window_handler(answer, props.setting.read().clone(), store.to_owned()))
        )
    });

    let mut handler = {
        let mut settings_store = settings_store.clone();
        move |new_value: f64| {
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
        }
    };

    rsx! {
        UiNumber {
            min: props.data.read().min(),
            max: props.data.read().max(),
            step: props.data.read().step(),
            value: props.data.read().value(),
            onmousedowninput: {
                let confirmed_setting_ids = settings_store
                    .approved_confirmation_setting_ids()
                    .read()
                    .clone();
                let check_confirmation = Rc::clone(&check_confirmation);
                move |evt: MouseEvent| {
                    if let None = props.setting.read().setting().confirmation() {
                        return;
                    }
                    if !confirmed_setting_ids.contains(&props.setting.read().id().clone()) {
                        evt.prevent_default();
                        check_confirmation();
                    }
                }
            },
            oninput: {
                let confirmed_setting_ids = settings_store
                    .approved_confirmation_setting_ids()
                    .read()
                    .clone();
                move |new_value| {
                    if let Some(_) = props.setting.read().setting().confirmation() {
                        if !confirmed_setting_ids.contains(&props.setting.read().id().clone()) {
                            return;
                        }
                    }
                    handler(new_value);
                }
            },
            disabled: match *props.disabled.read() {
                Some(v) => v,
                None => false,
            },
        }
    }
}