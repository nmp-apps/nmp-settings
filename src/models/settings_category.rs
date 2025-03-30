use crate::models::Icon;

#[derive(PartialEq, Clone)]
pub enum SettingsCategory {
    Appearance,
    Bluetooth,
    Desktop,
    Display,
    LockScreen,
    Network,
    Notifications,
    Security,
    Sound,
    Users
}

impl SettingsCategory {
    pub fn get_name(&self) -> String {
        match self {
            Self::Appearance => String::from("appearance"),
            Self::Bluetooth => String::from("bluetooth"),
            Self::Desktop => String::from("desktop"),
            Self::Display => String::from("display"),
            Self::LockScreen => String::from("lock screen"),
            Self::Network => String::from("network"),
            Self::Notifications => String::from("notifications"),
            Self::Security => String::from("security"),
            Self::Sound => String::from("sound"),
            Self::Users => String::from("users")
        }
    }
    pub fn get_icon(&self) -> Icon {
        match self {
            Self::Appearance => Icon::Appearance,
            Self::Bluetooth => Icon::Bluetooth,
            Self::Desktop => Icon::Desktop,
            Self::Display => Icon::Display,
            Self::LockScreen => Icon::LockScreen,
            Self::Network => Icon::Network,
            Self::Notifications => Icon::Notifications,
            Self::Security => Icon::Security,
            Self::Sound => Icon::Sound,
            Self::Users => Icon::Users
        }
    }
}