use std::time::Duration;

use tauri::{AppHandle, Runtime, State};
use tauri_plugin_clipboard_manager::ClipboardExt;
use zeroize::Zeroize;

use crate::app::config::AppConfig;
use crate::app::state::{ItemState, RuntimeState};
use crate::error::Result;
use crate::handlers::hide_window;
use crate::item::ItemRef;

#[tauri::command]
pub async fn copy_primary<R: Runtime>(
    app: AppHandle<R>,
    runtime_state: State<'_, RuntimeState>,
    item_state: State<'_, ItemState>,
    config: State<'_, AppConfig>,
    item_ref: ItemRef,
) -> Result<()> {
    let item = item_state.get_by_ref(&item_ref)?;
    match item {
        Some(item) => {
            let mut secret = item.content.get_primary()?;
            app.clipboard().write_secret(&secret)?;

            hide_window(&app)?;

            secret.zeroize();

            let clear_interval = Duration::from_secs(config.get_clear_interval() as u64);
            runtime_state.reset_clear_clipboard_handler(&app, clear_interval)?;
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
