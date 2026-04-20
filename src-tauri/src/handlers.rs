use clap::Parser;
use tauri::{AppHandle, Manager, Runtime, State, Window, WindowEvent};
use tauri_plugin_global_shortcut::{Shortcut, ShortcutEvent, ShortcutState};

use crate::app::cli::{Cli, CommandContext};
use crate::app::config::AppConfig;
use crate::app::QuarkAppExt;

pub fn on_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            log::debug!("Closing to system tray");

            window.hide().unwrap();
            api.prevent_close();
        }
        WindowEvent::Focused(false) => {
            log::debug!("Window lost focus, hiding");

            window.hide().unwrap();
        }
        _ => (),
    }
}

pub fn on_multiple_instance<R: Runtime>(app: &AppHandle<R>, args: Vec<String>, _cwd: String) {
    log::debug!("App re-launched with args {:?}", args);

    let args = Cli::parse_from(args);
    args.run(CommandContext::SingleInstance {
        app_handle: app.clone(),
    });
}

pub fn global_shortcut_handler<R: Runtime>(
    app: &AppHandle<R>,
    shortcut: &Shortcut,
    event: ShortcutEvent,
) {
    let app_config: State<'_, AppConfig> = app.state();
    if event.state() != ShortcutState::Pressed {
        return;
    }

    log::debug!("Detected global shortcut: '{}'", shortcut);
    if let Some(action) = app_config.get_global_shortcut_action(shortcut) {
        log::debug!("Action for shortcut is '{:?}'", action);
        match action {
            crate::app::config::GlobalShortcutAction::Show => {
                app.show_window().expect("failed to show main window");
            }
        }
    }
}
