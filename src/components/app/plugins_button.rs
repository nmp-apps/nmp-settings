use std::cell::RefCell;
use std::rc::Rc;

use dioxus::desktop::tao::platform::unix::{WindowBuilderExtUnix, WindowExtUnix};
use dioxus::desktop::tao::window::{WindowSizeConstraints};
use dioxus::desktop::wry::dpi::{LogicalUnit, PixelUnit};
use dioxus::desktop::{use_window, Config, DesktopContext, LogicalSize, WindowBuilder};
use dioxus::logger::tracing::error;
use dioxus::prelude::*;

use crate::components::{IconButton, Plugins, WindowWrapper};
use crate::constants::IS_WINDOW_CONTEXT_MENU_DISABLED;
use crate::hooks::use_close_child_windows;
use crate::models::Icon;
use crate::stores::{AppStore, AppWindowName, PluginsStore};
use crate::utils::{WindowLogicalData, WindowPercentSize};

const APP_WINDOW_NAME: AppWindowName = AppWindowName::Plugins;

#[component]
pub fn PluginsButton() -> Element {
    let window: DesktopContext = use_window();
    let (_, close_window) = use_close_child_windows();
    let plugins_store = use_context::<PluginsStore>();
    let app_store = use_context::<Signal<AppStore>>();
    let close_window = Rc::new(RefCell::new(close_window));

    let handler = move |_| {
        let plugins_store = plugins_store.clone();
        let mut app_store = app_store.clone();
        let window = window.clone();
        let close_window = close_window.clone();

        // if window already opened just focus on it
        if app_store.read().opened_windows().contains_key(&APP_WINDOW_NAME) {
            let app_store = app_store.read();
            let opened_window = match app_store.opened_windows().get(&APP_WINDOW_NAME) {
                Some(v) => v,
                None => {
                    return;
                }
            };
            opened_window.set_focus();
            return;
        }

        let window_logical_data = match WindowLogicalData::new(&window, WindowPercentSize::new(35.0, 56.0)) {
            Some(d) => d,
            None => {
                error!("Failed build window data");
                return;
            }
        };

        spawn(async move {
            let plugins_window = window.new_window(
                VirtualDom::new_with_props(PluginsListDialog, PluginsListDialogProps {
                    plugins_store: plugins_store.clone(),
                    on_close: Callback::new(move |_| {
                        close_window.borrow_mut()(&APP_WINDOW_NAME);
                    }),
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
                                    Option::Some(PixelUnit::Logical(LogicalUnit::new(window_logical_data.size.width))),
                                    Option::Some(PixelUnit::Logical(LogicalUnit::new(window_logical_data.size.height))),
                                    Option::Some(PixelUnit::Logical(LogicalUnit::new(window_logical_data.size.width))),
                                    Option::Some(PixelUnit::Logical(LogicalUnit::new(window_logical_data.size.height)))
                                )
                            )
                            .with_inner_size(LogicalSize::new(window_logical_data.size.width, window_logical_data.size.height))
                            .with_maximizable(false)
                            .with_minimizable(false)
                            .with_position(window_logical_data.centered_position)
                            .with_resizable(false)
                            .with_transparent(true)
                            .with_transient_for(window.gtk_window())
                            .with_theme(Option::None)
                    )
            ).await;
            // save dialog in app wide HashMap for future control
            app_store.write().push_opened_window(APP_WINDOW_NAME, plugins_window);
        });
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
    plugins_store: PluginsStore,
    on_close: Callback
}

#[component]
fn PluginsListDialog(props: PluginsListDialogProps) -> Element {
    rsx! {
        WindowWrapper {
            Plugins { plugins_store: props.plugins_store, on_close: props.on_close }
        }
    }
}