use tauri::{AppHandle, State};
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::app::state::ItemState;
use crate::error::Result;
use crate::handlers::hide_window;
use crate::item::ItemRef;

#[tauri::command]
pub async fn copy_primary(
    app_handle: AppHandle,
    item_state: State<'_, ItemState>,
    item_ref: ItemRef,
) -> Result<()> {
    let item = item_state.get_by_ref(&item_ref)?;
    match item {
        Some(item) => {
            let secret = item.content.get_primary();
            app_handle.clipboard().write_secret(secret)?;

            hide_window(&app_handle)?;
        }
        None => {
            log::debug!(
                "No item matching composite key {:?}",
                item_ref.composite_key()
            );
        }
    }

    Ok(())
}
