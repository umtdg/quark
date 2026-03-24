mod base64_serde;
mod config;
mod crypto;
mod date;
mod item;
mod state;
mod vault;

use std::sync::{Arc, Mutex};

use anyhow::Result;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Builder, Manager};

use crate::config::AppConfig;
use crate::crypto::Dek;
use crate::state::AppState;

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
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                log::debug!("Closing to system tray");

                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![]);

    let app = builder.build(tauri::generate_context!())?;
    let app_handle = app.handle();

    let quit_tray_item = MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>)?;
    let tray_menu = Menu::with_items(app_handle, &[&quit_tray_item])?;
    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&tray_menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                log::info!("Quitting through tray");
                app.exit(0);
            }
            _ => {
                log::debug!("Tray menu item {:?} is not handled", event.id);
            }
        })
        .build(app_handle)?;

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
