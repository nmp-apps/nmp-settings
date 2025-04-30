#[derive(Clone, Debug, PartialEq)]
pub struct DisabledPlugin {
    plugin_name: String,
    plugin_version: Option<String>,
    nmp_settings_version: Option<String>,
}

impl DisabledPlugin {
    pub fn new(plugin_name: String, plugin_version: Option<String>, nmp_settings_version: Option<String>) -> DisabledPlugin {
        DisabledPlugin { plugin_name, plugin_version, nmp_settings_version }
    }
}