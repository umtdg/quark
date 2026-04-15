pub mod app;
pub mod commands;
pub mod error;
pub mod handlers;
pub mod item;
pub mod serde;

use clap::Parser;
use tauri::{Builder, Manager};

use crate::app::cli::Cli;
use crate::app::config::AppConfig;
use crate::app::state::{AppState, CryptoState, ItemState, RuntimeState};
use crate::app::tray::create_icon;
use crate::commands::{
    copy_alt, copy_primary, copy_secondary, get_items, get_shortcuts, init_crypto,
    is_first_launch, is_locked, lock, refresh_items, unlock,
};
use crate::error::Result;
use crate::handlers::{on_multiple_instance, on_window_event};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    let context = tauri::generate_context!();
    let bundle_identifier = &context.config().identifier;

    let runtime_state = RuntimeState::new(bundle_identifier.as_str(), false)?;

    let config_path = match &cli.config {
        Some(config_path) => config_path.clone(),
        None => runtime_state.config_dir.join("config.toml"),
    };
    let mut app_config = AppConfig::load(config_path)?;
    app_config.merge(&cli);

    let tauri_log = tauri_plugin_log::Builder::new()
        .targets([tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir { file_name: None },
        )])
        .max_file_size(50 * 1024)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(2))
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .level(app_config.get_level_filter())
        .build();

    let builder = Builder::default()
        .plugin(tauri_log)
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(on_multiple_instance))
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![
            copy_primary,
            copy_secondary,
            copy_alt,
            refresh_items,
            get_items,
            get_shortcuts,
            init_crypto,
            lock,
            unlock,
            is_locked,
            is_first_launch,
        ]);

    let app = builder.build(context)?;
    let app_handle = app.handle();

    if cli.command.unwrap_or(app::cli::Command::Show) != app::cli::Command::Show {
        eprintln!("There is no instance of the application running");
        return Ok(());
    }

    log::info!("Launching application with config: {:?}", app_config);

    let _tray_icon = create_icon(app_handle)?;

    let item_state_path = runtime_state.data_dir.join(ItemState::FILE_NAME);
    let item_state = match ItemState::load(&item_state_path)? {
        Some(item_state) => {
            log::info!("Loaded item state from existing file");
            item_state
        }
        None => {
            log::info!("Creating empty item state");
            let item_state = ItemState::new();
            item_state.save(item_state_path)?;

            item_state
        }
    };

    let crypto_state_path = runtime_state.data_dir.join(CryptoState::FILE_NAME);
    let crypto_state: Option<CryptoState> = match CryptoState::load(&crypto_state_path)? {
        Some(crypto_state) => {
            log::info!("Loaded crypto state from existing file");
            Some(crypto_state)
        }
        None => {
            log::info!("No crypto state is found. Setting first_launch = true");
            runtime_state.set_first_launch(true)?;
            None
        }
    };

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
