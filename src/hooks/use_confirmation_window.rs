use dioxus::prelude::*;
use dioxus::desktop::tao::window::{WindowSizeConstraints};
use dioxus::desktop::wry::dpi::{LogicalUnit, PixelUnit};
use dioxus::desktop::{use_window, Config, LogicalSize, WindowBuilder};
use dioxus::logger::tracing::error;

use crate::components::{Confirm, WindowWrapper};
use crate::constants::IS_WINDOW_CONTEXT_MENU_DISABLED;
use crate::models::{ConfirmationWindow};
use crate::stores::{AppStore, AppWindowName};
use crate::utils::{WindowLogicalData, WindowPercentSize};

// Shows confirmation window with buttons "Yes" and "No"
pub fn use_confirmation_window(window_name: AppWindowName, confirmation_window: &Option<ConfirmationWindow>, callback: Callback<bool>) -> impl Fn() -> bool {
    let app_store = use_context::<Signal<AppStore>>();
    let confirmation_window = confirmation_window.clone();
    
    let show_window = move || -> bool {
        let app_store = app_store.clone();
        let window_name = window_name.clone();

        // if window already opened just focus on it
        if app_store.read().opened_windows().contains_key(&window_name) {
            let app_store = app_store.read();
            let opened_window = match app_store.opened_windows().get(&window_name) {
                Some(v) => v,
                None => {
                    return false;
                }
            };
            opened_window.set_focus();
            return false;
        }

        if let Some(confirmation_window) = confirmation_window.clone() {
            let window = use_window();
            let window_logical_data = match WindowLogicalData::new(&window, WindowPercentSize::new(25.4, 15.7)) {
                Some(d) => d,
                None => {
                    error!("Failed build window data");
                    return false;
                }
            };

            spawn(async move {

                let confirmation_window = window.new_window(
                    VirtualDom::new_with_props(ConfirmationDialog, ConfirmationDialogProps {
                        title: confirmation_window.title().clone(),
                        description: confirmation_window.description().clone(),
                        app_window_name: window_name.clone(),
                        app_store: app_store.clone(),
                        onconfirm: callback
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
                                .with_inner_size(LogicalSize::new(650.0, 250.0))
                                .with_maximizable(false)
                                .with_minimizable(false)
                                .with_position(window_logical_data.centered_position)
                                .with_resizable(false)
                                .with_transparent(true)
                                // .with_transient_for(window.gtk_window())
                                .with_theme(Option::None)
                        )
                ).await;
                app_store.clone().write().push_opened_window(window_name.clone(), confirmation_window);
            });
            false
        } else {
            return true;
        }
    };
    show_window
}

#[derive(PartialEq, Props, Clone)]
struct ConfirmationDialogProps {
    title: String,
    description: Option<String>,
    app_window_name: AppWindowName,
    app_store: Signal<AppStore>,
    onconfirm: EventHandler<bool>
}

#[component]
fn ConfirmationDialog(props: ConfirmationDialogProps) -> Element {
    rsx! {
        WindowWrapper {
            Confirm {
                title: props.title,
                description: props.description,
                app_store: props.app_store,
                app_window_name: props.app_window_name,
                onconfirm: move |answer| props.onconfirm.call(answer),
            }
        }
    }
}