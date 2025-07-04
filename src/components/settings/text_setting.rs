use std::ops::Deref;
use std::rc::Rc;

use dioxus::logger::tracing::trace;
use dioxus::prelude::*;

use crate::components::Text as UiText;
use crate::hooks::use_confirmation_window;
use crate::models::{Text, SettingComponent};
use crate::stores::{AppWindowName, PluginsStore, SettingsStore, StoredSetting};
use crate::utils::confirmation_window_handler;

#[derive(PartialEq, Clone, Props)]
pub struct TextSettingProps {
    setting: ReadOnlySignal<StoredSetting>,
    data: ReadOnlySignal<Text>,
    disabled: ReadOnlySignal<Option<bool>>
}

#[component]
pub fn TextSetting(props: TextSettingProps) -> Element {
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
        move |new_value: String| {
            let setting_ref = props.setting.read();
            let setting = setting_ref.deref();
            let new_value_cloned = new_value.clone();
            settings_store.mutate_setting_value(
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
            settings_store.mutate_changed_settings(
                setting,
                &plugins_store,
                |plugin_setting_component| {
                    match plugin_setting_component {
                        SettingComponent::Text(plugin_component) => {
                            *plugin_component.value() == new_value      
                        },
                        _ => false
                    }
                }
            );
        }
    };

    rsx! {
        UiText {
            onmousedown: {
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
                    if !confirmed_setting_ids.contains(&props.setting.read().id().clone())
                        && !check_confirmation()
                    {
                        return;
                    }
                    handler(new_value);
                }
            },
            value: props.data.read().value(),
            min_length: props.data.read().min_length(),
            max_length: props.data.read().max_length(),
            disabled: match *props.disabled.read() {
                Some(v) => v,
                None => false,
            },
        }
    }
}