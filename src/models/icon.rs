use dioxus::prelude::*;
use crate::components::*;

#[derive(PartialEq, Clone)]
pub enum Icon {
    Appearance,
    Bluetooth,
    Close,
    Desktop,
    Display,
    Extension,
    Info,
    LeftPanelClose,
    LeftPanelOpen,
    LockScreen,
    Minimize,
    Network,
    Notifications,
    Security,
    Sound,
    Square,
    Users

}

impl Icon {
    pub fn to_component(&self, size: String) -> Element {
        match self {
            Icon::Appearance => AppearanceIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Bluetooth => BluetoothIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Close => CloseIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Desktop => DesktopIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Display => DisplayIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Extension => ExtensionIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Info => InfoIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::LeftPanelClose => LeftPanelCloseIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::LeftPanelOpen => LeftPanelOpenIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::LockScreen => LockScreenIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Minimize => MinimizeIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Network => NetworkIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Notifications => NotificationsIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Security => SecurityIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Sound => SoundIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Square => SquareIcon(IconProps { class: None, color: None, size: Some(size), }),
            Icon::Users => UsersIcon(IconProps { class: None, color: None, size: Some(size), }),
        }
    }
}