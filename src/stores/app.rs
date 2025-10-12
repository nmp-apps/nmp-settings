use std::collections::HashMap;
use std::rc::{Rc};
use std::time::{Duration, SystemTime};
use std::fmt;

use dioxus::desktop::DesktopService;
use dioxus::prelude::*;
use dioxus::logger::tracing::trace;
use tokio::time::sleep;

use crate::models::Notification;

/// User app settings store.
#[derive(Clone)]
pub struct AppStore {
    notifications: Vec<Notification>,
    notifications_timeout_queue: Vec<SystemTime>,
    opened_windows: HashMap<AppWindowName, Rc<DesktopService>>
}

impl fmt::Debug for AppStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppStore")
            .field("notifications", &self.notifications)
            .field("notifications_timeout_queue", &self.notifications_timeout_queue)
            .field("opened_windows", &format!("{:?}", self.opened_windows().keys())) // only keys because DesktopService doesn't implement Debug
            .finish()
    }
}

impl AppStore {
    fn new() -> AppStore {
        AppStore { notifications: vec![], notifications_timeout_queue: vec![], opened_windows: HashMap::new() }
    }
    pub fn notifications(&self) -> &Vec<Notification> {
        &self.notifications
    }
    pub fn notifications_mut(&mut self) -> &mut Vec<Notification> {
        &mut self.notifications
    }
    pub fn push_notification(&mut self, notification: &Notification) {
        let notification_clone = notification.clone();
        self.notifications_mut().push(notification_clone);
    }
    pub fn remove_notification(&mut self, notification: &Notification) {
        self.notifications_mut().retain(|item| item != notification);
    }
    pub fn notifications_timeout_queue(&self) -> &Vec<SystemTime> {
        &self.notifications_timeout_queue
    }
    pub fn push_notifications_timeout_queue(&mut self, notification_id: &SystemTime) {
        self.notifications_timeout_queue.push(notification_id.clone());
    }
    pub fn remove_notifications_timeout_queue(&mut self, notification_id: &SystemTime) {
        self.notifications_timeout_queue.retain(|item| item != notification_id);
    }
    pub fn opened_windows(&self) -> &HashMap<AppWindowName, Rc<DesktopService>> {
        &self.opened_windows
    }
    pub fn push_opened_window(&mut self, window_name: AppWindowName, window_service: Rc<DesktopService>) {
        self.opened_windows.insert(window_name, window_service);
    }
    pub fn remove_opened_window_by_name(&mut self, window_name: AppWindowName) {
        self.opened_windows.remove(&window_name);
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum AppWindowName {
    Plugins,
    SettingConfirmation,
    ClosingApp
}

pub fn use_app_store() {
    trace!("Init app store...");

    let app_store = use_context_provider(|| Signal::new(AppStore::new()));
    let app_store = app_store.clone();

    use_effect(move || {
        let app_store = app_store.clone();
        let _ = app_store.read().notifications(); // bind notifications reactivity

        spawn(async move {
            let (last_notification, contains_timeout_queue): (Option<Notification>, bool) = {
                let app_store_read = app_store.read();
                let last = app_store_read.notifications().last().cloned();
                match last {
                    Some(n) => (Some(n.clone()), app_store_read.notifications_timeout_queue().contains(&n.id())),
                    None => (None, false)
                }
            };

            if let Some(notification) = last_notification {
                    if !*notification.permanent() && !contains_timeout_queue {
                    // timeout case
                    let sleep_duration = *notification.timeout();
                    let notification = notification.clone();

                    let mut app_store = app_store.clone();
                    app_store.write().push_notifications_timeout_queue(&notification.id());

                    spawn(async move {
                        sleep(Duration::from_millis(sleep_duration.into())).await;
                        let mut app_store = app_store.write();
                        app_store.remove_notification(&notification);
                        app_store.remove_notifications_timeout_queue(&notification.id());
                    });
                }
            }
        });
    });
}
