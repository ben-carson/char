use tauri::{
    AppHandle, Result,
    menu::{MenuItem, MenuItemKind},
};

use super::MenuItemHandler;

pub struct TrayQuit;

impl MenuItemHandler for TrayQuit {
    const ID: &'static str = "hypr_tray_quit";

    fn build(app: &AppHandle<tauri::Wry>) -> Result<MenuItemKind<tauri::Wry>> {
        let shortcut = if cfg!(target_os = "macos") {
            Some("cmd+shift+q")
        } else {
            Some("ctrl+shift+q")
        };
        let item = MenuItem::with_id(app, Self::ID, "Quit Completely", true, shortcut)?;
        Ok(MenuItemKind::MenuItem(item))
    }

    fn handle(app: &AppHandle<tauri::Wry>) {
        #[cfg(target_os = "macos")]
        {
            hypr_host::kill_processes_by_matcher(hypr_host::ProcessMatcher::Sidecar);
            hypr_intercept::set_force_quit();
        }

        app.exit(0);
    }
}
