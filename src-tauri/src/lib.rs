pub mod app;
pub mod error;
pub mod item;
pub mod serde;

use std::collections::HashSet;

use ::serde::{Deserialize, Serialize};
use clap::Parser;
use tauri::{AppHandle, Builder, Emitter, Manager, Runtime, State, Window, WindowEvent};
use zeroize::Zeroize;

use crate::app::cli::{Cli, Command};
use crate::app::config::AppConfig;
use crate::app::crypto::{Dek, Kek};
use crate::app::shell::{get_vault_items, get_vaults};
use crate::app::state::{AppState, CryptoState, ItemState, RuntimeState};
use crate::app::tray::create_icon;
use crate::error::{Error, Result};
use crate::item::ItemRef;

#[derive(Serialize)]
struct PageResult<T> {
    items: T,
    total: usize,
}

#[derive(Deserialize)]
struct Pagination {
    offset: usize,
    limit: usize,
}

#[tauri::command]
async fn refresh_items(
    app_handle: AppHandle,
    runtime_state: State<'_, RuntimeState>,
    item_state: State<'_, ItemState>,
    config: State<'_, AppConfig>,
) -> Result<()> {
    app_handle.emit("refresh-started", None::<&str>)?;

    if item_state.is_locked()? {
        return Err(Error::Locked);
    }

    let pass_cli_path = config.get_pass_cli_path();

    let vaults = get_vaults(app_handle.clone(), pass_cli_path).await?;
    for vault in vaults {
        let vault_items =
            get_vault_items(app_handle.clone(), pass_cli_path, &vault.share_id).await?;

        log::debug!("Adding vault items to stored items");
        item_state.extend(vault_items)?;
    }

    item_state.save(runtime_state.data_dir.join(ItemState::FILE_NAME))?;

    app_handle.emit("refresh-completed", None::<&str>)?;

    Ok(())
}

#[tauri::command]
fn get_items(
    item_state: State<'_, ItemState>,
    query: String,
    pagination: Pagination,
) -> Result<PageResult<HashSet<ItemRef>>> {
    log::debug!("Getting decrypted item refs");
    let item_refs = item_state.get_decrypted_item_refs()?;

    let query = query.to_lowercase();
    let mut matches: Vec<&ItemRef> = item_refs
        .iter()
        .filter(|item| item.title.to_lowercase().contains(&query))
        .collect();
    matches.sort_by(|a, b| a.title.cmp(&b.title));

    let total = matches.len();
    let offset = pagination.offset.clamp(0, total);
    let limit = pagination.limit.clamp(0, 50);
    let page = matches
        .iter()
        .skip(offset)
        .take(limit)
        .map(|&item_ref| item_ref.clone())
        .collect();

    Ok(PageResult { items: page, total })
}

#[tauri::command]
async fn init_crypto(
    app_handle: AppHandle,
    runtime_state: State<'_, RuntimeState>,
    item_state: State<'_, ItemState>,
    mut password: String,
) -> Result<()> {
    let (crypto_state, new_dek) = CryptoState::new(password.as_bytes())?;
    password.zeroize();

    crypto_state.save(runtime_state.data_dir.join(CryptoState::FILE_NAME))?;

    item_state.replace_dek(new_dek)?;
    runtime_state.set_first_launch(false)?;

    app_handle.manage(crypto_state);
    app_handle.emit("state-changed", None::<&str>)?;

    Ok(())
}

#[tauri::command]
async fn lock(app_handle: AppHandle, item_state: State<'_, ItemState>) -> Result<()> {
    item_state.lock()?;

    app_handle.emit("state-changed", None::<&str>)?;

    Ok(())
}

#[tauri::command]
async fn unlock(
    app_handle: AppHandle,
    item_state: State<'_, ItemState>,
    crypto_state: State<'_, CryptoState>,
    mut password: String,
) -> Result<()> {
    log::debug!("Waiting DEK for write");
    let mut dek = item_state
        .dek
        .write()
        .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

    if dek.is_some() {
        return Ok(());
    }

    let kek = Kek::new(
        password.as_bytes(),
        &crypto_state.salt,
        &crypto_state.kdf_params,
    )?;
    password.zeroize();

    let stored_dek: Dek = crypto_state.wrapped_dek.decrypt(&kek.0)?;
    dek.replace(stored_dek);

    app_handle.emit("state-changed", None::<&str>)?;

    Ok(())
}

#[tauri::command]
fn is_locked(item_state: State<'_, ItemState>) -> Result<bool> {
    item_state.is_locked()
}

#[tauri::command]
fn is_first_launch(runtime_state: State<'_, RuntimeState>) -> Result<bool> {
    runtime_state.is_first_launch()
}

fn on_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
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
        .plugin(tauri_plugin_single_instance::init(|app, args, _| {
            log::debug!("App re-launched with args {:?}", args);

            let args = Cli::parse_from(args);
            match args.command.unwrap_or(Command::Show) {
                Command::Show => {
                    let window = app.get_webview_window("main").expect(
                        "cannot find the main window. try to kill any dangling/zombie processes",
                    );

                    window.show().expect("error when showing main window");
                    window.set_focus().expect("error when focusing main window");
                }
                Command::Lock => {
                    let app_handle = app.clone();
                    let item_state: State<'_, ItemState> = app.state();

                    let _ = tauri::async_runtime::block_on(async move {
                        lock(app_handle, item_state).await
                    });
                }
                Command::Quit => {
                    app.exit(0);
                }
            }
        }))
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![
            refresh_items,
            get_items,
            init_crypto,
            lock,
            unlock,
            is_locked,
            is_first_launch,
        ]);

    let app = builder.build(context)?;
    let app_handle = app.handle();

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
