use tauri::{AppHandle, State};
use tauri_plugin_clipboard_manager::ClipboardExt;
use zeroize::Zeroize;

use crate::{
    app::state::ItemState,
    error::{Error, Result},
    handlers::get_main_window,
    item::ItemRef,
};

#[tauri::command]
pub async fn copy_alt(
    app_handle: AppHandle,
    item_state: State<'_, ItemState>,
    item_ref: ItemRef,
) -> Result<()> {
    let item = item_state.get_by_ref(&item_ref)?;
    match item {
        Some(item) => {
            let mut secret = item.content.get_alt();
            app_handle.clipboard().write_secret(&secret)?;

            let window = get_main_window(&app_handle)?;
            window
                .hide()
                .map_err(|err| Error::Window(err.to_string()))?;

            // get_alt returns String instead of &str
            secret.zeroize();
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
