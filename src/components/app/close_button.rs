use std::cell::RefCell;
use std::rc::Rc;

use dioxus::desktop::tao::platform::unix::{WindowBuilderExtUnix, WindowExtUnix};
use dioxus::desktop::tao::window::{WindowId, WindowSizeConstraints};
use dioxus::desktop::wry::dpi::{LogicalUnit, PixelUnit, Size};
use dioxus::desktop::{use_window, Config, DesktopContext, LogicalPosition, LogicalSize, WindowBuilder};
use dioxus::logger::tracing::error;
use dioxus::prelude::*;

use crate::components::{ClosingApp, IconButton, WindowWrapper};
use crate::constants::IS_WINDOW_CONTEXT_MENU_DISABLED;
use crate::hooks::use_close_child_windows;
use crate::models::Icon;
use crate::stores::{AppStore, AppWindowName, SettingsStore};

const APP_WINDOW_NAME: AppWindowName = AppWindowName::ClosingApp;

#[component]
pub fn CloseButton() -> Element {
    let window: DesktopContext = use_window();
    let app_store = use_context::<Signal<AppStore>>();
    let settings_store = use_context::<SettingsStore>();
    let (close_child_windows, close_window_by_name) = use_close_child_windows();
    let close_child_windows = Rc::new(RefCell::new(close_child_windows));
    let close_window_by_name = Rc::new(RefCell::new(close_window_by_name));

    let handler = move |_| {
        let mut app_store = app_store.clone();
        let window = window.clone();
        let close_child_windows = close_child_windows.clone();
        let close_window_by_name = close_window_by_name.clone();
        
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

        // if no changes, close app and it's child windows
        if settings_store.changed_settings().read().len() <= 0 {
            close_child_windows.borrow_mut()();
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

        let width: f64 = ((monitor_size.width - 650) / 2).into();
        let height: f64 = ((monitor_size.height - 250) / 2).into();
        let window_position = LogicalPosition::new(width, height);

        spawn(async move {
            let closing_window = window.new_window(
                VirtualDom::new_with_props(UnsavedSettingsDialog, UnsavedSettingsDialogProps {
                    parent_window_id: window.id(),
                    close_windows: Callback::new(move |_| {
                        close_child_windows.borrow_mut()();
                    }),
                    close_self: Callback::new(move |_| {
                        close_window_by_name.borrow_mut()(&APP_WINDOW_NAME);
                    })
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
                            .with_transparent(true)
                            .with_theme(None)
                            .with_transient_for(window.gtk_window())
                    )
                ).await;
                // save dialog in app wide HashMap for future control
                app_store.write().push_opened_window(APP_WINDOW_NAME, closing_window);
        });
    };

    rsx! {
        IconButton { onclick: handler, size: "16px", icon: Icon::Close }
    }
}

#[derive(PartialEq, Props, Clone)]
struct UnsavedSettingsDialogProps {
    parent_window_id: WindowId,
    close_windows: Callback,
    close_self: Callback
}

#[component]
fn UnsavedSettingsDialog(props: UnsavedSettingsDialogProps) -> Element {
    rsx! {
        WindowWrapper {
            ClosingApp {
                parent_window_id: props.parent_window_id,
                close_windows: props.close_windows,
                close_self: props.close_self,
            }
        }
    }
}