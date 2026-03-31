use clap::Parser;
use tauri::{AppHandle, Manager, Runtime, State, Window, WindowEvent};

use crate::app::cli::{Cli, Command};
use crate::app::state::ItemState;
use crate::commands::lock;
use crate::error::{Error, Result};

pub fn show_window<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    log::info!("Showing and focusing main window");

    let window = app.get_webview_window("main").ok_or(Error::ShowWindow(
        "cannot find the main window, try to kill any dangling/zombie processes".into(),
    ))?;

    window
        .show()
        .map_err(|err| Error::ShowWindow(err.to_string()))?;
    window
        .set_focus()
        .map_err(|err| Error::ShowWindow(err.to_string()))?;

    Ok(())
}

pub fn lock_app<R: Runtime>(app: &AppHandle<R>) {
    log::info!("Locking application");

    let app_clone = app.clone();
    let item_state: State<'_, ItemState> = app.state();

    let lock_task = async move { lock(app_clone, item_state).await };
    let _ = tauri::async_runtime::block_on(lock_task);
}

pub fn on_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            log::debug!("Closing to system tray");

            window.hide().unwrap();
            api.prevent_close();
        }
        WindowEvent::Focused(false) => {
            log::debug!("Window lost focus, hiding to system tray");

            window.hide().unwrap();
        }
        _ => (),
    }
}

pub fn on_multiple_instance<R: Runtime>(app: &AppHandle<R>, args: Vec<String>, _cwd: String) {
    log::debug!("App re-launched with args {:?}", args);

    // TODO: these actions are the exact same as tray actions
    let args = Cli::parse_from(args);
    match args.command.unwrap_or(Command::Show) {
        Command::Show => {
            show_window(app).expect("failed to show main window");
        }
        Command::Lock => {
            lock_app(app);
        }
        Command::Quit => {
            log::info!("Quitting application");

            app.exit(0);
        }
    }
}
