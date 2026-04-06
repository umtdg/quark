use tauri::State;

use crate::app::state::ItemState;
use crate::error::Result;
use crate::item::ItemRef;

#[tauri::command]
pub fn get_items(item_state: State<'_, ItemState>, query: String) -> Result<Vec<ItemRef>> {
    log::debug!("Getting decrypted item refs");

    let query = query.to_lowercase();
    let mut matches: Vec<ItemRef> = item_state.get_decrypted_item_refs()?
        .iter()
        .filter(|item| item.title.to_lowercase().contains(&query))
        .map(Clone::clone)
        .collect();
    matches.sort_by(|a, b| a.title.cmp(&b.title));

    Ok(matches)
}
