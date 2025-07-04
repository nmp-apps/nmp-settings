use std::ops::Deref;

use dioxus::logger::tracing::trace;
use dioxus::prelude::*;

use crate::components::{ButtonGroup, ButtonGroupItem};
use crate::hooks::use_confirmation_window;
use crate::models::{ButtonGroup as ButtonGroupModel, SettingComponent};
use crate::stores::{AppWindowName, PluginsStore, SettingsStore, StoredSetting};
use crate::utils::confirmation_window_handler;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonGroupSettingProps {
    setting: ReadOnlySignal<StoredSetting>,
    data: ReadOnlySignal<ButtonGroupModel>,
    disabled: ReadOnlySignal<bool>
}

#[component]
pub fn ButtonGroupSetting(props: ButtonGroupSettingProps) -> Element {
    let settings_store = use_context::<SettingsStore>();
    let plugins_store = use_context::<PluginsStore>();

    let converted_items: Memo<Vec<ButtonGroupItem>> = use_memo(move || {
        let data = props.data.read();
        data.items().iter()
            .map(
                |item| ButtonGroupItem::new(
                    item.text().clone(), 
                    item.value().clone()
                )
            )
            .collect()
    });

    let check_confirmation = {
        let store = settings_store.clone();
        use_confirmation_window(
            AppWindowName::SettingConfirmation,
            props.setting.read().setting().confirmation(),
            Callback::new(move |answer: bool| confirmation_window_handler(answer, props.setting.read().clone(), store.to_owned()))
        )
    };

    let mut handler = {
        let mut settings_store = settings_store.clone();
        move |new_value: String| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        settings_store.mutate_setting_value(
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
        settings_store.mutate_changed_settings(
            setting,
            &plugins_store,
            |plugin_setting_component| {
                match plugin_setting_component {
                    SettingComponent::ButtonGroup(plugin_component) => {
                        *plugin_component.value() == new_value      
                    },
                    _ => false
                }
            }
        );
    }
    };

    rsx! {
        ButtonGroup {
            onchange: {
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
            items: converted_items(),
            value: props.data.read().value(),
            disabled: props.disabled,
        }
    }
}