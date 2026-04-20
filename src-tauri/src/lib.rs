pub mod app;
pub mod commands;
pub mod error;
pub mod handlers;
pub mod item;
pub mod serde;

use clap::Parser;
use tauri::plugin::TauriPlugin;
use tauri::{App, Builder, Context, Runtime, Wry};

use crate::app::cli::{Cli, CommandContext};
use crate::app::config::AppConfig;
use crate::app::state::RuntimeState;
use crate::commands::{
    copy_alt, copy_primary, copy_secondary, get_items, get_shortcut_action, init_crypto,
    is_first_launch, is_locked, lock, refresh_items, unlock,
};
use crate::error::Result;
use crate::handlers::{global_shortcut_handler, on_multiple_instance, on_window_event};

fn build_log<R: Runtime>(app_config: &AppConfig) -> TauriPlugin<R> {
    tauri_plugin_log::Builder::new()
        .targets([tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir { file_name: None },
        )])
        .max_file_size(50 * 1024)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(2))
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .level(app_config.get_level_filter())
        .build()
}

fn build_global_shortcut<R: Runtime>(app_config: &AppConfig) -> Result<TauriPlugin<R>> {
    Ok(tauri_plugin_global_shortcut::Builder::new()
        .with_shortcuts(app_config.get_global_shortcuts())?
        .with_handler(global_shortcut_handler)
        .build())
}

fn build_app<R: Runtime>(context: Context<R>, app_config: &AppConfig) -> Result<App<R>> {
    Builder::<R>::new()
        .plugin(build_log(app_config))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(on_multiple_instance))
        .plugin(build_global_shortcut(app_config)?)
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
        ])
        .build(context)
        .map_err(Into::into)
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

    let app: App<Wry> = build_app(context, &app_config)?;

    // if we end up here, it means that single instance plugin didn't pick up
    // and this is the first launch of the app

    cli.run(CommandContext::FirstLaunch {
        app,
        app_config: Box::new(app_config),
        runtime_state,
    });

    Ok(())
}
