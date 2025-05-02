use dioxus::desktop::tao::window::{Theme, WindowSizeConstraints};
use dioxus::desktop::wry::dpi::{LogicalUnit, PixelUnit, Size};
use dioxus::desktop::{use_window, Config, DesktopContext, LogicalPosition, LogicalSize, WindowBuilder};
use dioxus::logger::tracing::error;
use dioxus::prelude::*;

use crate::components::{IconButton, Plugins, WindowWrapper};
use crate::models::Icon;
use crate::stores::PluginsStore;

#[component]
pub fn PluginsButton() -> Element {
    let window: DesktopContext = use_window();
    let plugins_store = use_context::<PluginsStore>();

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
        window.new_window(
            VirtualDom::new_with_props(PluginsListDialog, PluginsListDialogProps { plugins_store: plugins_store.clone() }),
            Config::new()
                .with_as_child_window()
                .with_disable_context_menu(true)
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
                        .with_theme(Some(Theme::Dark))
                )
        );
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
    plugins_store: PluginsStore
}

#[component]
fn PluginsListDialog(props: PluginsListDialogProps) -> Element {
    rsx! {
        WindowWrapper {
            Plugins { plugins_store: props.plugins_store }
        }
    }
}