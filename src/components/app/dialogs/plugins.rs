use dioxus::prelude::*;
use crate::stores::PluginsStore;
use crate::components::WindowBar;

static STYLES: Asset = asset!("/assets/styles/app/dialogs/plugins.css");

#[derive(PartialEq, Props, Clone)]
pub struct PluginsProps {
    plugins_store: PluginsStore,
    on_close: Callback
}

#[component]
pub fn Plugins(props: PluginsProps) -> Element {
    let plugins_store = props.plugins_store.clone();
    let disabled_plugin_names: Memo<Vec<String>> = use_memo(move || {
        plugins_store.get_disabled_plugins().read().iter().map(|dp| {
            dp.name().clone()
        }).collect()
    });

    rsx! {
        main { class: "plugins-dialog",
            document::Stylesheet { href: "{STYLES}", rel: "preload" }
            WindowBar {
                title: "Extensions",
                on_close: move |_| props.on_close.call(()),
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
                                                {plugin.get_version()}
                                            }
                                            p {
                                                span { "Required settings version: " }
                                                {plugin.get_nmp_settings_version()}
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