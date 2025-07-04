use dioxus::prelude::*;

use crate::stores::AppStore;

pub fn use_close_child_windows() -> impl Fn() {
    let app_store = use_context::<Signal<AppStore>>();

    let close_windows  = move || {
        if app_store.read().opened_windows().len() > 0 {
            app_store.read().opened_windows().iter().for_each(|(_, window_service)| {
                if let Some(service) = window_service.upgrade() {
                    service.close();
                }
            });
        }
    };
    close_windows
}