/// Wrapper for `asset!` macro with AppImage assets path compatibility fix
#[macro_export]
macro_rules! get_asset {
    ($x:expr) => {
        {
        let prefix = if cfg!(feature = "appimage_assets") {
            let current_dir_buffer = std::env::current_dir().unwrap();
            let current_dir = current_dir_buffer.to_str().expect("Can't take a current path.");
            &format!("{}/lib/NmpSettings", current_dir)
        } else {
            ""
        };

        let asset = dioxus::prelude::asset!($x);
        format!("{}{}", prefix, asset)
        }
    };
}