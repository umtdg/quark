use tauri::{AppHandle, Emitter, Runtime, State};
use zeroize::Zeroize;

use crate::app::crypto::{Dek, Kek};
use crate::app::state::{CryptoState, ItemState};
use crate::error::{Error, Result};

#[tauri::command]
pub async fn unlock<R: Runtime>(
    app_handle: AppHandle<R>,
    item_state: State<'_, ItemState>,
    crypto_state: State<'_, CryptoState>,
    mut password: String,
) -> Result<()> {
    log::trace!("Waiting DEK for write");
    let mut dek = item_state
        .dek
        .write()
        .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

    if dek.is_some() {
        return Ok(());
    }

    let kek = Kek::new(
        password.as_bytes(),
        &crypto_state.salt,
        &crypto_state.kdf_params,
    )?;
    password.zeroize();

    let stored_dek: Dek = crypto_state.wrapped_dek.decrypt(&kek.0)?;
    dek.replace(stored_dek);

    app_handle.emit("state-changed", None::<&str>)?;

    Ok(())
}
