use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use dioxus::desktop::tao::window::{Theme, WindowId, WindowSizeConstraints};
use dioxus::desktop::wry::dpi::{LogicalUnit, PixelUnit, Size};
use dioxus::desktop::{use_window, Config, DesktopContext, LogicalPosition, LogicalSize, WindowBuilder};

use crate::get_asset;
use crate::components::{ClosingApp, IconButton, SaveSettingsButton, Switch, WindowWrapper};
use crate::models::Icon;
use crate::stores::SettingsStore;


#[derive(PartialEq, Props, Clone)]
pub struct TopBarProps {
    #[props(into)]
    class: Option<String>
}

#[component]
pub fn TopBar(props: TopBarProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/app/top_bar.css"));
    let settings_store = use_context::<SettingsStore>();
    let window: DesktopContext = use_window();
    let mut is_advanced_settings_enabled = use_signal(|| false);
    let top_bar_classes = match &props.class {
        Some(classes) => format!("top-bar {classes}"),
        None => "top-bar".to_string()
    };

    let click_close_handler = {
        let window = window.clone();
        move || {
            if settings_store.changed_settings().read().len() <= 0 {
                window.close();
                return;
            }
            
            // open closing dialog for unsaved changes
            let monitor_size = match window.current_monitor() {
                Some(cm) => cm.size(),
                None => {
                    error!("Failed get monitor size");
                    return;
                }
            };
            // let child_window = child_window.upgrade().unwrap();
            let window_position = LogicalPosition::new(
                (monitor_size.width - 650) / 2,
                (monitor_size.height - 250) / 2,
            );
            window.new_window(
                VirtualDom::new_with_props(UnsavedSettingsDialog, UnsavedSettingsDialogProps { parent_window_id: window.id() }),
                Config::new()
                    .with_as_child_window()
                    .with_disable_context_menu(true)
                    .with_window(
                        WindowBuilder::new()
                            .with_always_on_top(true)
                            .with_closable(true)
                            .with_focused(true)
                            .with_decorations(false)
                            .with_inner_size_constraints(
                                WindowSizeConstraints::new(
                                    Option::Some(PixelUnit::Logical(LogicalUnit::new(650.0))),
                                    Option::Some(PixelUnit::Logical(LogicalUnit::new(250.0))),
                                    Option::Some(PixelUnit::Logical(LogicalUnit::new(650.0))),
                                    Option::Some(PixelUnit::Logical(LogicalUnit::new(250.0)))
                                )
                            )
                            .with_inner_size(Size::Logical(LogicalSize { height: 250.0, width: 600.0 }))
                            .with_maximizable(false)
                            .with_minimizable(false)
                            .with_position(window_position)
                            .with_resizable(false)
                            // .with_transparent(true)
                            // .with_transient_for(window.gtk_window())
                            .with_theme(Some(Theme::Dark))
                    )
            );
        }
    };

    let click_minimize_handler = {
        let window = window.clone();
        move || {
            window.set_minimized(true);
        }
    };

    let click_maximize_handler = {
        let window = window.clone();
        move || {
            window.set_maximized(true);
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: "{styles}" }

        div { class: top_bar_classes,
            div { class: "top-bar__left" }
            div { class: "top-bar__center",
                h1 { class: "top-bar__title", "Settings" }
            }
            div { class: "top-bar__right",
                IconButton { icon: Icon::Extension, title: "Manage Extensions" }
                Switch {
                    title: "Enable advanced settings",
                    value: is_advanced_settings_enabled(),
                    onchange: move |event: Event<FormData>| is_advanced_settings_enabled.set(event.checked()),
                }
                SaveSettingsButton {}
                div { class: "top-bar__window-actions",
                    IconButton {
                        onclick: move |_| click_minimize_handler(),
                        size: "16px",
                        icon: Icon::Minimize,
                    }
                    IconButton {
                        onclick: move |_| click_maximize_handler(),
                        size: "16px",
                        icon: Icon::Square,
                    }
                    IconButton {
                        onclick: move |_| click_close_handler(),
                        size: "16px",
                        icon: Icon::Close,
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct UnsavedSettingsDialogProps {
    parent_window_id: WindowId
}

#[component]
fn UnsavedSettingsDialog(props: UnsavedSettingsDialogProps) -> Element {
    rsx! {
        WindowWrapper {
            ClosingApp { parent_window_id: props.parent_window_id }
        }
    }
}