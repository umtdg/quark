use tauri::{AppHandle, Emitter, State};

use crate::app::state::ItemState;
use crate::error::Result;

#[tauri::command]
pub async fn lock(app_handle: AppHandle, item_state: State<'_, ItemState>) -> Result<()> {
    item_state.lock()?;

    app_handle.emit("state-changed", None::<&str>)?;

    Ok(())
}
