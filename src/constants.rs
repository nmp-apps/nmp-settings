#[cfg(debug_assertions)]
pub const IS_WINDOW_CONTEXT_MENU_DISABLED: bool = false;
#[cfg(not(debug_assertions))]
pub const IS_WINDOW_CONTEXT_MENU_DISABLED: bool = true;
