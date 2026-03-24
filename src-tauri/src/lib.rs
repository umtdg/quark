mod base64_serde;
mod config;
mod crypto;
mod date;
mod item;
mod state;
mod vault;

use std::sync::Mutex;

use anyhow::Result;
use tauri::{Builder, Manager};

use crate::config::AppConfig;
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
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_log)
        .invoke_handler(tauri::generate_handler![]);

    let app = builder.build(tauri::generate_context!())?;
    let app_handle = app.handle();

    let app_config = AppConfig::load(app_handle)?;
    log::debug!("Application config: {:?}", app_config);

    let app_state = AppState::load_or_new(app_config.get_state_file())?;

    // TODO: Unlock DEK if locked

    app.manage(app_config);
    app.manage(Mutex::new(app_state));

    app.run(|_, _| {});

    Ok(())
}
