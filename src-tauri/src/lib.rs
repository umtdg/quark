mod base64_serde;
mod config;
mod crypto;
mod date;
mod error;
mod item;
mod shell;
mod state;
mod tray;
mod vault;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Builder, Emitter, Manager, Runtime, State, Window, WindowEvent};
use zeroize::Zeroize;

use crate::crypto::EncryptionState;
use crate::error::{Error, Result};
use crate::item::ItemRef;
use crate::shell::{get_vault_items, get_vaults};
use crate::state::AppState;
use crate::tray::create_icon;

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
async fn refresh_items(app_handle: AppHandle, state: State<'_, AppState>) -> Result<()> {
    app_handle.emit("refresh-started", None::<&str>)?;

    if state.is_locked()? {
        return Err(Error::Locked);
    }

    let pass_cli_path = state.config.get_pass_cli_path();

    let vaults = get_vaults(app_handle.clone(), pass_cli_path).await?;
    for vault in vaults {
        let vault_items =
            get_vault_items(app_handle.clone(), pass_cli_path, &vault.share_id).await?;

        log::debug!("Adding vault items to stored items");
        state.extend(vault_items)?;
    }

    state.save(app_handle.clone())?;

    app_handle.emit("refresh-completed", None::<&str>)?;

    Ok(())
}

#[tauri::command]
fn get_items(
    state: State<'_, AppState>,
    query: String,
    pagination: Pagination,
) -> Result<PageResult<HashSet<ItemRef>>> {
    log::debug!("Check if app is unlocked");
    if state.is_locked()? {
        return Err(Error::Locked);
    }

    log::debug!("Getting decrypted item refs");
    let item_refs = state.get_decrypted_item_refs()?;

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
    let tauri_log = tauri_plugin_log::Builder::new()
        .targets([tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir { file_name: None },
        )])
        .max_file_size(50 * 1024)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(2))
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .level(log::LevelFilter::Trace)
        .build();

    let builder = Builder::default()
        .plugin(tauri_log)
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![refresh_items, get_items]);

    let app = builder.build(tauri::generate_context!())?;
    let app_handle = app.handle();

    let _tray_icon = create_icon(app_handle)?;

    let app_state = match AppState::load(app_handle.clone())? {
        Some(app_state) => {
            log::debug!("Loaded application state from existing file");
            app_state
        }
        None => {
            log::debug!("First launch. Creating new application state");

            // TODO: Prompt for password
            let mut password: Vec<u8> = b"password".into();

            log::debug!("Creating new encryption state");
            let (encryption_state, new_dek) = EncryptionState::new(&password)?;

            log::debug!("Creating new application state");
            let app_state = AppState::new(app_handle.clone(), encryption_state, Some(new_dek))?;

            password.zeroize();

            log::debug!("Saving application state to file");
            app_state.save(app_handle.clone())?;
            app_state
        }
    };

    log::info!("Application config: {:?}", app_state.config);

    log::trace!("Manage application state");
    app.manage(app_state);

    log::info!("Runing application");
    app.run(|_, _| {});

    Ok(())
}
