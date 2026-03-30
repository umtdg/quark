use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::app::state::ItemState;
use crate::error::Result;
use crate::item::ItemRef;

#[derive(Serialize)]
pub struct PageResult<T> {
    items: T,
    total: usize,
}

#[derive(Deserialize)]
pub struct Pagination {
    offset: usize,
    limit: usize,
}

#[tauri::command]
pub fn get_items(
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
