mod item;
mod vault;

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use item::{items_from_pass_cli, Item, ItemFields, ItemRef};
use tauri::{Builder, State};
use vault::Vault;

struct AppState {
    items: Vec<Item>,
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
fn get_item_list(
    state: State<'_, Mutex<AppState>>,
    pagination: PaginationInput,
) -> PageResult<ItemRef> {
    let state = state.lock().unwrap();

    let offset = pagination.offset.unwrap_or(0);
    let limit = pagination.limit.unwrap_or(10);

    PageResult {
        total: state.items.len(),
        items: state
            .items
            .iter()
            .skip(offset)
            .take(limit)
            .map(|item| ItemRef {
                id: item.id.clone(),
                share_id: item.share_id.clone(),
                title: item.content.title.clone(),
                itype: match item.content.content {
                    ItemFields::Login(_) => "Login".into(),
                },
            })
            .collect(),
    }
}

#[tauri::command]
fn get_item_info(state: State<'_, Mutex<AppState>>, item_ref: ItemRef) -> Option<Item> {
    let state = state.lock().unwrap();

    state
        .items
        .iter()
        .filter(|i| i.id == item_ref.id && i.share_id == item_ref.share_id)
        .next()
        .cloned()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let vaults = Vault::from_pass_cli().expect("failed to get list of vaults");
    let mut items = vec![];
    for vault in &vaults {
        if (vault.name != "Personal") {
            continue;
        }

        let share_id = &vault.share_id;
        let mut vault_items = items_from_pass_cli(share_id)
            .expect(&format!("failed to get items from vault {}", vault.name));

        items.append(&mut vault_items.items);
    }

    let state = AppState { items };

    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(Mutex::new(state))
        .invoke_handler(tauri::generate_handler![get_item_list, get_item_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
