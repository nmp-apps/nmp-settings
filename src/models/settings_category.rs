use crate::models::Icon;
use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
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
    /// Human readable name of category
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
    /// Value is a name of category in snake_case
    pub fn get_id(&self) -> String {
        match self {
            Self::Appearance => String::from("appearance"),
            Self::Bluetooth => String::from("bluetooth"),
            Self::Desktop => String::from("desktop"),
            Self::Display => String::from("display"),
            Self::LockScreen => String::from("lock_screen"),
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
    pub fn get_by_id(id: &str) -> SettingsCategory {
        match id {
            "appearance" => Self::Appearance,  
            "bluetooth" => Self::Bluetooth,  
            "desktop" => Self::Desktop,  
            "display" => Self::Display,  
            "lock_screen" => Self::LockScreen,  
            "network" => Self::Network,  
            "notifications" => Self::Notifications,  
            "security" => Self::Security,  
            "sound" => Self::Sound,  
            "users" => Self::Users,
            _ => panic!("Setting category: wrong id"),
        } 
    }
}