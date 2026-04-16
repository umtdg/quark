use tauri::{AppHandle, Emitter, Manager, Runtime, State};
use zeroize::Zeroize;

use crate::app::state::{AppState, CryptoState, ItemState, RuntimeState};
use crate::error::Result;

#[tauri::command]
pub async fn init_crypto<R: Runtime>(
    app_handle: AppHandle<R>,
    runtime_state: State<'_, RuntimeState>,
    item_state: State<'_, ItemState>,
    mut password: String,
) -> Result<()> {
    let (crypto_state, new_dek) = CryptoState::new(password.as_bytes())?;
    password.zeroize();

    crypto_state.save(runtime_state.data_dir.join(CryptoState::FILE_NAME))?;

    item_state.replace_dek(new_dek)?;
    runtime_state.set_first_launch(false)?;

    app_handle.manage(crypto_state);
    app_handle.emit("state-changed", None::<&str>)?;

    Ok(())
}
