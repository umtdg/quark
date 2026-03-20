mod item;
mod vault;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;

use item::{items_from_pass_cli, Item, ItemData, ItemRef};
use tauri::{Builder, State};
use vault::Vault;

struct AppState {
    items: HashSet<Item>,
}

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
fn get_items(
    state: State<'_, Mutex<AppState>>,
    pagination: PaginationInput,
    query: String,
) -> PageResult<ItemRef> {
    let state = state.lock().unwrap();

    let total = state.items.len();
    let offset = pagination.offset.unwrap_or(0).clamp(0, total);
    let limit = pagination.limit.unwrap_or(10).clamp(10, 50);

    let items: Vec<ItemRef> = state
        .items
        .iter()
        .filter(|item| item.content.title.to_lowercase().contains(&query))
        .skip(offset)
        .take(limit)
        .map(|item| ItemRef {
            id: item.id.clone(),
            share_id: item.share_id.clone(),
            title: item.content.title.clone(),
            itype: match item.content.content {
                ItemData::Login(_) => "Login".into(),
                ItemData::CreditCard(_) => "CreditCard".into(),
            },
        })
        .collect();

    PageResult {
        total: items.len(),
        items,
    }
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
pub fn run() {
    let vaults = Vault::from_pass_cli().expect("failed to get list of vaults");
    let mut items = HashSet::with_capacity(64);
    for vault in &vaults {
        let share_id = &vault.share_id;
        let vault_items = items_from_pass_cli(share_id)
            .expect(&format!("failed to get items from vault {}", vault.name));

        items.extend(vault_items);
    }

    let state = AppState { items };

    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(Mutex::new(state))
        .invoke_handler(tauri::generate_handler![
            get_items,
            copy_primary,
            copy_secondary,
            copy_alt
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
