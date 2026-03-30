use tauri::{AppHandle, Emitter, Runtime, State};

use crate::app::state::ItemState;
use crate::error::Result;

#[tauri::command]
pub async fn lock<R: Runtime>(
    app_handle: AppHandle<R>,
    item_state: State<'_, ItemState>,
) -> Result<()> {
    item_state.lock()?;

    app_handle.emit("state-changed", None::<&str>)?;

    Ok(())
}
