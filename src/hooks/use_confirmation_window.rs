use dioxus::prelude::*;
use dioxus::desktop::tao::window::{Theme, WindowSizeConstraints};
use dioxus::desktop::wry::dpi::{LogicalUnit, PixelUnit, Size};
use dioxus::desktop::{use_window, Config, LogicalPosition, LogicalSize, WindowBuilder};
use dioxus::logger::tracing::error;

use crate::components::{Confirm, WindowWrapper};
use crate::constants::IS_WINDOW_CONTEXT_MENU_DISABLED;
use crate::models::{ConfirmationWindow};

// Shows confirmation window with buttons "Yes" and "No"
pub fn use_confirmation_window(confirmation_window: &Option<ConfirmationWindow>, callback: Callback<bool>) -> impl Fn() -> bool {
    let confirmation_window = confirmation_window.clone();
    let show_window = move || -> bool {
        if let Some(confirmation_window) = confirmation_window.clone() {
            let window = use_window();
            let monitor_size = match window.current_monitor() {
                Some(cm) => cm.size(),
                None => {
                    error!("Failed get monitor size");
                    return false;
                }
            };
            let window_position = LogicalPosition::new(
                (monitor_size.width - 650) / 2,
                (monitor_size.height - 250) / 2,
            );
            window.new_window(
                VirtualDom::new_with_props(ConfirmationDialog, ConfirmationDialogProps {
                    title: confirmation_window.title().clone(),
                    description: confirmation_window.description().clone(),
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
                            // .with_transient_for(window.gtk_window())
                            .with_theme(Some(Theme::Dark))
                    )
            );
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
    onconfirm: EventHandler<bool>
}

#[component]
fn ConfirmationDialog(props: ConfirmationDialogProps) -> Element {
    rsx! {
        WindowWrapper {
            Confirm {
                title: props.title,
                description: props.description,
                onconfirm: move |answer| props.onconfirm.call(answer),
            }
        }
    }
}