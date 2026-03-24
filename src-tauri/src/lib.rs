mod base64_serde;
mod config;
mod crypto;
mod date;
mod item;
mod state;
mod tray;
mod vault;

use std::sync::{Arc, Mutex};

use anyhow::Result;
use tauri::{Builder, Manager, Runtime, Window, WindowEvent};

use crate::config::AppConfig;
use crate::crypto::Dek;
use crate::state::AppState;
use crate::tray::create_icon;

fn on_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    let tauri_log = tauri_plugin_log::Builder::new()
        .targets([tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir {
                file_name: Some("logs".into()),
            },
        )])
        .max_file_size(50 * 1024)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(2))
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .level(log::LevelFilter::Debug)
        .build();

    let builder = Builder::default()
        .plugin(tauri_log)
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![]);

    let app = builder.build(tauri::generate_context!())?;
    let app_handle = app.handle();

    let _ = create_icon(app_handle)?;

    let app_config = AppConfig::load(app_handle)?;
    log::debug!("Application config: {:?}", app_config);

    let app_state = AppState::load_or_new(app_config.get_state_file())?;
    let dek: Option<Dek> = None;

    app.manage(app_config);
    app.manage(Mutex::new(app_state));
    app.manage(Arc::new(Mutex::new(dek)));

    app.run(|_, _| {});

    Ok(())
}
