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
            Icon::Appearance => AppearanceIcon(IconProps { color: None, size: Some(size), }),
            Icon::Bluetooth => BluetoothIcon(IconProps { color: None, size: Some(size), }),
            Icon::Close => CloseIcon(IconProps { color: None, size: Some(size), }),
            Icon::Desktop => DesktopIcon(IconProps { color: None, size: Some(size), }),
            Icon::Display => DisplayIcon(IconProps { color: None, size: Some(size), }),
            Icon::Extension => ExtensionIcon(IconProps { color: None, size: Some(size), }),
            Icon::LockScreen => LockScreenIcon(IconProps { color: None, size: Some(size), }),
            Icon::Minimize => MinimizeIcon(IconProps { color: None, size: Some(size), }),
            Icon::Network => NetworkIcon(IconProps { color: None, size: Some(size), }),
            Icon::Notifications => NotificationsIcon(IconProps { color: None, size: Some(size), }),
            Icon::Security => SecurityIcon(IconProps { color: None, size: Some(size), }),
            Icon::Sound => SoundIcon(IconProps { color: None, size: Some(size), }),
            Icon::Square => SquareIcon(IconProps { color: None, size: Some(size), }),
            Icon::Users => UsersIcon(IconProps { color: None, size: Some(size), }),
        }
    }
}