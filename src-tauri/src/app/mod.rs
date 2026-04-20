pub mod cli;
pub mod config;
pub mod crypto;
pub mod shell;
pub mod state;
pub mod tray;

use tauri::{AppHandle, Emitter, Manager, Runtime, State, WebviewWindow};
use tauri_plugin_clipboard_manager::ClipboardExt;

use state::ItemState;

use crate::error::{Error, Result};

pub trait QuarkAppExt<R: Runtime> {
    fn get_main_window(&self) -> Result<WebviewWindow<R>>;

    fn show_window(&self) -> Result<()>;

    fn hide_window(&self) -> Result<()>;

    fn lock(&self) -> Result<()>;

    fn clear_clipboard(&self) -> Result<()>;

    fn print_version(&self);

    fn print_info(&self);
}

impl<R: Runtime> QuarkAppExt<R> for AppHandle<R> {
    fn get_main_window(&self) -> Result<WebviewWindow<R>> {
        self.get_webview_window("main").ok_or(Error::Window(
            "Cannot find the main window. Try to kill any dangling processes".into(),
        ))
    }

    fn show_window(&self) -> Result<()> {
        log::info!("Showing and focusing main window");

        let window = self.get_main_window()?;

        window
            .show()
            .map_err(|err| Error::Window(err.to_string()))?;
        window
            .set_focus()
            .map_err(|err| Error::Window(err.to_string()))
    }

    fn hide_window(&self) -> Result<()> {
        log::info!("Hiding window");

        let window = self.get_main_window()?;

        window.hide().map_err(|err| Error::Window(err.to_string()))
    }

    fn lock(&self) -> Result<()> {
        log::info!("Locking application");

        self.state::<State<'_, ItemState>>().lock()?;
        self.emit("state-changed", None::<&str>)?;

        Ok(())
    }

    fn clear_clipboard(&self) -> Result<()> {
        log::debug!("Clearing clipboard");

        self.clipboard().clear()?;
        self.emit("clipboard-clear", None::<&str>)?;

        Ok(())
    }

    fn print_version(&self) {
        println!("{}", self.package_info().version);
    }

    fn print_info(&self) {
        let package_info = self.package_info();
        println!("{} {}", package_info.name, package_info.version);
        println!("Authors: {}", package_info.authors);
        println!("Description: {}", package_info.description);
    }
}
