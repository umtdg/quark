use tauri::{AppHandle, Emitter, State};

use crate::app::config::AppConfig;
use crate::app::shell::{get_vault_items, get_vaults};
use crate::app::state::{AppState, ItemState, RuntimeState};
use crate::error::{Error, Result};

#[tauri::command]
pub async fn refresh_items(
    app: AppHandle,
    runtime_state: State<'_, RuntimeState>,
    item_state: State<'_, ItemState>,
    config: State<'_, AppConfig>,
) -> Result<()> {
    app.emit("refresh-started", None::<&str>)?;

    if item_state.is_locked()? {
        return Err(Error::Locked);
    }

    let pass_cli_path = config.get_pass_cli_path();

    let vaults = get_vaults(&app, pass_cli_path).await?;
    for vault in vaults {
        let vault_items =
            get_vault_items(&app, pass_cli_path, &vault.share_id).await?;

        log::debug!("Adding vault items to stored items");
        item_state.extend(vault_items)?;
    }

    item_state.save(runtime_state.data_dir.join(ItemState::FILE_NAME))?;

    app.emit("refresh-completed", None::<&str>)?;

    Ok(())
}
