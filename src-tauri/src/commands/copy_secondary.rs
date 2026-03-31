use tauri::{AppHandle, State};
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::app::state::ItemState;
use crate::error::{Error, Result};
use crate::handlers::get_main_window;
use crate::item::ItemRef;

#[tauri::command]
pub async fn copy_secondary(
    app_handle: AppHandle,
    item_state: State<'_, ItemState>,
    item_ref: ItemRef,
) -> Result<()> {
    let item = item_state.get_by_ref(&item_ref)?;
    match item {
        Some(item) => {
            let secret = item.content.get_secondary();
            app_handle.clipboard().write_secret(secret)?;

            let window = get_main_window(&app_handle)?;
            window
                .hide()
                .map_err(|err| Error::Window(err.to_string()))?;
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
