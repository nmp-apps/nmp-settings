use dioxus::prelude::*;

use crate::stores::{AppStore, AppWindowName};

pub fn use_close_child_windows() -> (impl FnMut(), impl FnMut(&AppWindowName)) {
    let mut app_store = use_context::<Signal<AppStore>>();

    let mut close_window_by_name = move |window_name: &AppWindowName| {
        println!("close_window_by_name");
        let service = {
            let store = app_store.read();
            store
            .opened_windows()
            .get(&window_name)
            .cloned()
        };
        
        if let Some(window_service) = service {
            println!("Closed {:#?}: {:#?}", window_name, window_service.id());
            window_service.close();
            app_store
                .write()
                .remove_opened_window_by_name(window_name.clone());
        }
    };

    let close_windows  = move || {
        if app_store.read().opened_windows().len() <= 0 {
            return;
        }
        let window_names: Vec<_> = {
            let store_read = app_store.read();
            store_read.opened_windows().keys().cloned().collect()
        };

        // Then close them in turn.
        for name in window_names {
            close_window_by_name(&name);
        }
    };

    (close_windows, close_window_by_name)
}