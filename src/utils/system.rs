use std::{
    env,
    fs,
    path::PathBuf,
};

use dioxus::logger::tracing::error;
use directories_next::ProjectDirs;

/// Find path of executables by partial name
pub fn find_apps_by_name(partial_name: &str) -> Vec<String> {
    // All directories
    let mut search_dirs: Vec<String> = vec![
        // String::from("/usr/bin"),
        // String::from("/usr/local/bin"),
        // String::from("/opt"),
        // String::from("/snap"),
    ];

    // Directories starts from "~"
    // let home_dirs = vec![
    //     String::from("/bin"),
    //     String::from("/.local/bin"),
    //     String::from("/.local/share"),
    //     String::from("/Applications"),
    //     String::from("/.var/app")
    // ];
    // // Get the user's home directory and add user-specific paths
    // if let Some(home_dir) = env::var_os("HOME") {
    //     let home_path = PathBuf::from(home_dir);
    //     home_dirs.iter().for_each(|dir_path| {
    //         search_dirs.push(format!("{}{}", home_path.display(), dir_path));
    //     });
    // }

    // Get directories from $PATH environment variable
    if let Ok(path_var) = env::var("PATH") {
        let path_dirs: Vec<&str> = path_var.split(':').collect();
        search_dirs.extend(path_dirs.iter().map(|s| s.to_string()));
    }

    let mut found_apps = Vec::new();

    for dir in &search_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    if file_name.contains(partial_name) {
                        found_apps.push(file_name.to_string());
                    }
                }
            }
        }
    }
    found_apps
}

/// Get app config file path from `nmp/settings/` directory.
pub fn get_app_config_path() -> Option<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "nmp", "nmp");
    match proj_dirs {
        Some(proj_dirs) => {
            let mut nmp_dir = proj_dirs.config_dir().to_path_buf();
            nmp_dir.push("settings");
            match fs::create_dir_all(&nmp_dir) {
                Ok(_) => {
                    nmp_dir.push("config.json");
                    Some(nmp_dir)
                },
                Err(err) => {
                    error!("Can't create nmp-settings config directory: {}", err);
                    None
                }
            }
        },
        None => {
            error!("Can't get nmp-settings dirs");
            None
        }
    }
}