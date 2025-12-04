use dioxus::prelude::*;

use crate::components::{CategoryNavigation, IconButton, PluginCategoryNavigation};
use crate::models::Icon;
use crate::stores::AppStore;


#[component]
pub fn LeftBar() -> Element {
    let app_store = use_context::<Signal<AppStore>>();

    rsx! {
        div { class: "left-bar",
            class: if app_store.read().is_left_bar_opened() { "open" },
            div {
                class: "left-bar__card",
                CategoryNavigation {}
                hr { class: "left-bar__divider" }
                PluginCategoryNavigation {}
            }
        }
    }
}

#[component]
pub fn LeftBarHideButton() -> Element {
    let mut app_store = use_context::<Signal<AppStore>>();

    let is_last_small = use_signal(|| false);

    use_effect({
        let mut app_store = app_store.clone();
        let mut is_last_small = is_last_small.clone();
        move || {
            let (w, _) = *app_store.read().window_size();
            let mut store = app_store.write();
            let is_open = store.is_left_bar_opened();

            let is_cur_small = w <= 768;
            let is_prev_small = *is_last_small.read();

            if is_cur_small != is_prev_small {
                // Threshold crossed
                if is_cur_small && is_open {
                    store.set_left_bar_opened(false); // auto-close on small window
                } else if !is_cur_small && !is_open {
                    store.set_left_bar_opened(true); // auto-reopen on large window
                }

                // Update is_last_small to current state
                *is_last_small.write() = is_cur_small;
            }
        }
    });

    let handler = move |_| {
        let current_value = app_store.read().is_left_bar_opened();
        let mut app_store_mut = app_store.write();
        app_store_mut.set_left_bar_opened(!current_value);
    };
    rsx! {
        IconButton {
            icon: {
                if app_store.read().is_left_bar_opened() {
                    Icon::LeftPanelClose
                } else {
                    Icon::LeftPanelOpen
                }
            },
            title: {
                if app_store.read().is_left_bar_opened() {
                    "Hide Left Panel"
                } else {
                    "Show Left Panel"
                }
            },
            size: "20px",
            onclick: handler,
        }
    }
}