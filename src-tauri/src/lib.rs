mod config;
mod date;
mod item;
mod store;
mod vault;

use anyhow::Result;
use anyhow_tauri::TAResult;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use item::ItemRef;
use store::AppState;
use tauri::{AppHandle, Builder, Emitter, Manager, State};

use crate::{config::AppConfig, item::Item};

#[derive(Deserialize)]
struct PaginationInput {
    offset: Option<usize>,
    limit: Option<usize>,
}

#[derive(Serialize)]
struct PageResult<T> {
    items: Vec<T>,
    total: usize,
}

#[tauri::command]
async fn refresh_items(
    app_handle: AppHandle,
    state: State<'_, Mutex<AppState>>,
    app_config: State<'_, AppConfig>,
) -> TAResult<()> {
    log::debug!("Refreshing items from pass-cli");
    app_handle
        .emit::<Option<usize>>("refresh-started", None)
        .unwrap();

    let capacity: Option<usize> = {
        match state.lock() {
            Ok(state) => Some(state.items.capacity()),
            Err(_) => None,
        }
    };

    let items = match AppState::items_from_cli(&app_handle, &app_config, capacity).await {
        Ok(items) => items,
        Err(err) => {
            app_handle
                .emit("refresh-failed", format!("{:?}", err))
                .unwrap();
            return Err(err.into());
        }
    };

    let mut state = state.lock().unwrap();

    state.items.extend(items);

    let store_file_path = app_config.get_store_file();

    log::debug!("Saving state to {:?}", store_file_path);
    match state.to_file(store_file_path) {
        Ok(_) => {}
        Err(err) => {
            app_handle
                .emit("refresh-failed", format!("{:?}", err))
                .unwrap();
            return Err(err.into());
        }
    }

    app_handle
        .emit::<Option<usize>>("refresh-completed", None)
        .unwrap();

    Ok(())
}

#[tauri::command]
fn get_items(
    state: State<'_, Mutex<AppState>>,
    pagination: PaginationInput,
    query: String,
) -> PageResult<ItemRef> {
    log::debug!("Getting a list of items matching '{}'", &query);

    let state = state.lock().unwrap();

    let items: Vec<&Item> = state
        .items
        .iter()
        .filter(|item| item.content.title.to_lowercase().contains(&query))
        .collect();

    let total = items.len();
    let offset = pagination.offset.unwrap_or(0).clamp(0, total);
    let limit = pagination.limit.unwrap_or(10).clamp(10, 50);

    let items = items
        .iter()
        .skip(offset)
        .take(limit)
        .map(|&item| item.into())
        .collect();

    PageResult { total, items }
}

#[tauri::command]
fn copy_primary(state: State<'_, Mutex<AppState>>, item_ref: ItemRef) -> Option<String> {
    let state = state.lock().unwrap();

    let item = state
        .items
        .iter()
        .filter(|i| i.id == item_ref.id && i.share_id == item_ref.share_id)
        .next()
        .cloned()?;

    let secret: String = item.content.get_primary().into();

    Some(secret)
}

#[tauri::command]
fn copy_secondary(state: State<'_, Mutex<AppState>>, item_ref: ItemRef) -> Option<String> {
    let state = state.lock().unwrap();

    let item = state
        .items
        .iter()
        .filter(|i| i.id == item_ref.id && i.share_id == item_ref.share_id)
        .next()
        .cloned()?;

    let secret: String = item.content.get_secondary().into();

    Some(secret)
}

#[tauri::command]
fn copy_alt(state: State<'_, Mutex<AppState>>, item_ref: ItemRef) -> Option<String> {
    let state = state.lock().unwrap();

    let item = state
        .items
        .iter()
        .filter(|i| i.id == item_ref.id && i.share_id == item_ref.share_id)
        .next()
        .cloned()?;

    let secret: String = item.content.get_alt().into();

    Some(secret)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    let mut builder = Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init());

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
    builder = builder.plugin(tauri_log);

    builder = builder.invoke_handler(tauri::generate_handler![
        refresh_items,
        get_items,
        copy_primary,
        copy_secondary,
        copy_alt
    ]);

    let app = builder.build(tauri::generate_context!())?;
    let app_handle = app.handle();

    let app_config = AppConfig::load(&app_handle)?;
    let app_state = AppState::from_file(app_config.get_store_file())?;

    log::debug!("Application config: {:?}", app_config);

    app.manage(app_config);
    app.manage(Mutex::new(app_state));

    app.run(|_, _| {});

    Ok(())
}
