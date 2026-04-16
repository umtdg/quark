pub mod app;
pub mod commands;
pub mod error;
pub mod handlers;
pub mod item;
pub mod serde;

use clap::Parser;
use tauri::{App, Builder, Context, Manager};

use crate::app::cli::Cli;
use crate::app::config::AppConfig;
use crate::app::state::{AppState, CryptoState, ItemState, RuntimeState};
use crate::app::tray::create_icon;
use crate::commands::{
    copy_alt, copy_primary, copy_secondary, get_items, get_shortcut_action, init_crypto,
    is_first_launch, is_locked, lock, refresh_items, unlock,
};
use crate::error::Result;
use crate::handlers::{global_shortcut_handler, on_multiple_instance, on_window_event};

fn build_app(context: Context, app_config: &AppConfig) -> Result<App> {
    let tauri_log = tauri_plugin_log::Builder::new()
        .targets([tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir { file_name: None },
        )])
        .max_file_size(50 * 1024)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(2))
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .level(app_config.get_level_filter())
        .build();

    let tauri_global_shortcut = tauri_plugin_global_shortcut::Builder::new()
        .with_shortcuts(app_config.get_global_shortcuts())?
        .with_handler(global_shortcut_handler)
        .build();

    let builder = Builder::default()
        .plugin(tauri_log)
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(on_multiple_instance))
        .plugin(tauri_global_shortcut)
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![
            copy_primary,
            copy_secondary,
            copy_alt,
            refresh_items,
            get_items,
            init_crypto,
            lock,
            unlock,
            is_locked,
            is_first_launch,
            get_shortcut_action,
        ]);

    builder.build(context).map_err(Into::into)
}

fn launch_app(
    app: App,
    app_config: AppConfig,
    runtime_state: RuntimeState,
) -> Result<()> {
    log::info!("Launching application with config: {:?}", app_config);

    let _tray_icon = create_icon(app.handle())?;

    // unwrap is safe since we return Some() from the callback of load_or
    let item_state_path = runtime_state.data_dir.join(ItemState::FILE_NAME);
    let item_state = ItemState::load_or_new(item_state_path);

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

    let app = build_app(context, &app_config)?;

    // if we end up here, it means that single instance plugin didn't pick up
    // and this is the first launch of the app

    if cli.command.unwrap_or(app::cli::Command::Show) != app::cli::Command::Show {
        eprintln!("There is no instance of the application running");
        return Ok(());
    }

    launch_app(app, app_config, runtime_state)
}
