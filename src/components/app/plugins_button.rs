use dioxus::desktop::tao::window::WindowSizeConstraints;
use dioxus::desktop::wry::dpi::{LogicalUnit, PixelUnit, Size};
use dioxus::desktop::{use_window, Config, DesktopContext, LogicalPosition, LogicalSize, WindowBuilder};
use dioxus::logger::tracing::error;
use dioxus::prelude::*;

use crate::components::{IconButton, Plugins, WindowWrapper};
use crate::constants::IS_WINDOW_CONTEXT_MENU_DISABLED;
use crate::models::Icon;
use crate::stores::{AppStore, AppWindowName, PluginsStore};

const APP_WINDOW_NAME: AppWindowName = AppWindowName::Plugins;

#[component]
pub fn PluginsButton() -> Element {
    let window: DesktopContext = use_window();
    let plugins_store = use_context::<PluginsStore>();
    let mut app_store = use_context::<Signal<AppStore>>();

    let handler = move |_| {
        let monitor_size = match window.current_monitor() {
            Some(cm) => cm.size(),
            None => {
                error!("Failed get monitor size");
                return;
            }
        };
        let window_position = LogicalPosition::new(
            (monitor_size.width - 900) / 2,
            (monitor_size.height - 900) / 2,
        );
        let plugins_window = window.new_window(
            VirtualDom::new_with_props(PluginsListDialog, PluginsListDialogProps {
                app_window_name: APP_WINDOW_NAME,
                app_store: app_store.clone(),
                plugins_store: plugins_store.clone()
            }),
            Config::new()
                .with_as_child_window()
                .with_disable_context_menu(IS_WINDOW_CONTEXT_MENU_DISABLED)
                .with_window(
                    WindowBuilder::new()
                        .with_always_on_top(true)
                        .with_closable(true)
                        .with_focused(true)
                        .with_decorations(false)
                        .with_visible(false) // prevents performance issues on opening with decorations = false
                        .with_inner_size_constraints(
                            WindowSizeConstraints::new(
                                Option::Some(PixelUnit::Logical(LogicalUnit::new(900.0))),
                                Option::Some(PixelUnit::Logical(LogicalUnit::new(900.0))),
                                Option::Some(PixelUnit::Logical(LogicalUnit::new(900.0))),
                                Option::Some(PixelUnit::Logical(LogicalUnit::new(900.0)))
                            )
                        )
                        .with_inner_size(Size::Logical(LogicalSize { height: 900.0, width: 900.0 }))
                        .with_maximizable(false)
                        .with_minimizable(false)
                        .with_position(window_position)
                        .with_resizable(false)
                        .with_transparent(true)
                        // .with_transient_for(window.gtk_window())
                        .with_theme(None)
                )
        );
        app_store.write().push_opened_window(AppWindowName::Plugins, plugins_window);
    };
    
    rsx! {
        IconButton {
            icon: Icon::Extension,
            title: "Manage Extensions",
            onclick: handler,
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct PluginsListDialogProps {
    app_window_name: AppWindowName,
    app_store: Signal<AppStore>,
    plugins_store: PluginsStore
}

#[component]
fn PluginsListDialog(props: PluginsListDialogProps) -> Element {
    rsx! {
        WindowWrapper {
            Plugins {
                plugins_store: props.plugins_store,
                app_store: props.app_store,
                app_window_name: props.app_window_name,
            }
        }
    }
}