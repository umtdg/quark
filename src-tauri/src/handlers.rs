use clap::Parser;
use tauri::{AppHandle, Manager, Runtime, State, Window, WindowEvent};

use crate::app::cli::{Cli, Command};
use crate::app::state::ItemState;
use crate::commands::lock;

pub fn on_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            log::debug!("Closing to system tray");

            window.hide().unwrap();
            api.prevent_close();
        }
        WindowEvent::Focused(false) => {
            log::debug!("Window lost focus, hiding to system tray");

            // window.hide().unwrap();
        }
        _ => (),
    }
}

pub fn on_multiple_instance<R: Runtime>(app: &AppHandle<R>, args: Vec<String>, _cwd: String) {
    log::debug!("App re-launched with args {:?}", args);

    let args = Cli::parse_from(args);
    match args.command.unwrap_or(Command::Show) {
        Command::Show => {
            let window = app
                .get_webview_window("main")
                .expect("cannot find the main window. try to kill any dangling/zombie processes");

            window.show().expect("error when showing main window");
            window.set_focus().expect("error when focusing main window");
        }
        Command::Lock => {
            let app_handle = app.clone();
            let item_state: State<'_, ItemState> = app.state();

            let _ =
                tauri::async_runtime::block_on(async move { lock(app_handle, item_state).await });
        }
        Command::Quit => {
            app.exit(0);
        }
    }
}
