use dioxus::prelude::*;
use dioxus::desktop::window;

use crate::components::IconButton;
use crate::get_asset;
use crate::models::Icon;
use crate::stores::PluginsStore;

#[derive(PartialEq, Props, Clone)]
pub struct PluginsProps {
    plugins_store: PluginsStore
}

#[component]
pub fn Plugins(props: PluginsProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/app/dialogs/plugins.css"));
    let plugins_store = props.plugins_store.clone();
    let disabled_plugin_names: Memo<Vec<String>> = use_memo(move || {
        plugins_store.get_disabled_plugins().read().iter().map(|dp| {
            dp.name().clone()
        }).collect()
    });

    rsx! {
        main { class: "plugins-dialog",
            document::Stylesheet { href: "{styles}", rel: "preload" }
            div { class: "plugins-dialog__top-bar",
                div { class: "plugins-dialog__top-bar-left" }
                div { class: "plugins-dialog__top-bar-center",
                    h1 { class: "plugins-dialog__title", "Extensions" }
                }
                div { class: "plugins-dialog__top-bar-right",
                    IconButton {
                        onclick: move |_| {
                            window().close();
                        },
                        size: "16px",
                        icon: Icon::Close,
                    }
                }
            }
            section { class: "plugins-dialog__content",
                ul { class: "plugins-dialog__plugin-list plugins-dialog__enabled-plugins",
                    for plugin in props.plugins_store.get_plugins() {
                        {
                            if !disabled_plugin_names.read().contains(&plugin.get_name()) {
                                rsx! {
                                    li { class: "plugins-dialog__list-item",
                                        h3 { class: "plugins-dialog__item-title", {format!("{}", plugin.get_name())} }
                                        div { class: "plugins-dialog__plugin-data",
                                            p {
                                                span { "Version: " }
                                                {plugin.get_version().clone()}
                                            }
                                            p {
                                                span { "Required settings version: " }
                                                {plugin.get_nmp_settings_version().clone()}
                                            }
                                        }
                                    }
                                }
                            } else {
                                rsx! {}
                            }
                        }
                    }
                }

                if props.plugins_store.get_disabled_plugins().read().len() > 0 {
                    h2 { class: "plugins-dialog__disabled-plugins-title", "Disabled extensions" }
                    ul { class: "plugins-dialog__plugin-list",
                        for plugin in props.plugins_store.get_disabled_plugins().read().clone() {
                            li { class: "plugins-dialog__plugin-list-item",
                                h3 { class: "plugins-dialog__item-title",
                                    {format!("{}", plugin.name())}
                                }
                                if let Some(version) = plugin.version() {
                                    p {
                                        span { "Version: " }
                                        {version.clone()}
                                    }
                                }
                                if let Some(version) = plugin.nmp_settings_version() {
                                    p {
                                        span { "Required settings version: " }
                                        {version.clone()}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}