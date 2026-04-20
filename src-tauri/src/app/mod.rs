pub mod cli;
pub mod config;
pub mod crypto;
pub mod shell;
pub mod state;
pub mod tray;

use tauri::{App, Context, Emitter, Manager, Runtime, State, WebviewWindow};
use tauri_plugin_clipboard_manager::ClipboardExt;

use config::AppConfig;
use state::{AppState, CryptoState, ItemState, RuntimeState};
use tray::create_icon;

use crate::error::{Error, Result};

pub trait QuarkAppExt<R: Runtime> {
    fn get_main_window(&self) -> Result<WebviewWindow<R>>;

    fn show_window(&self) -> Result<()>;

    fn hide_window(&self) -> Result<()>;

    fn lock(&self) -> Result<()>;

    fn clear_clipboard(&self) -> Result<()>;
}

impl<R, T> QuarkAppExt<R> for T
where
    R: Runtime,
    T: Manager<R>,
    T: Emitter<R>,
{
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
}

pub fn launch_app<R: Runtime>(
    app: App<R>,
    app_config: AppConfig,
    runtime_state: RuntimeState,
) -> Result<()> {
    log::info!("Launching application with config: {:?}", app_config);

    let _tray_icon = create_icon(app.handle())?;

    // unwrap is safe since we return Some() from the callback of load_or
    let item_state_path = runtime_state.data_dir.join(ItemState::FILE_NAME);
    let item_state: ItemState = ItemState::load_or_new(item_state_path)?;

    let crypto_state_path = runtime_state.data_dir.join(CryptoState::FILE_NAME);
    let crypto_state: Option<CryptoState> = CryptoState::load_or(&crypto_state_path, |_| {
        log::info!("No crypto state is found. Setting first_launch = true");
        runtime_state.set_first_launch(true)?;

        Ok(None)
    })?;

    app.manage(app_config);
    app.manage(runtime_state);
    app.manage(item_state);
    if let Some(crypto_state) = crypto_state {
        app.manage(crypto_state);
    }

    log::info!("Runing application");
    app.run(|_, _| {});

    Ok(())
}

pub trait QuarkAppContextExt<R: Runtime> {
    fn print_version(&self);

    fn print_info(&self);
}

impl<R> QuarkAppContextExt<R> for Context<R>
where
    R: Runtime,
{
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
