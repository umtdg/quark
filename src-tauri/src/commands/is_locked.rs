use tauri::State;

use crate::app::state::ItemState;
use crate::error::Result;

#[tauri::command]
pub fn is_locked(item_state: State<'_, ItemState>) -> Result<bool> {
    item_state.is_locked()
}
