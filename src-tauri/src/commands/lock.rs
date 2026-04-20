use tauri::{AppHandle, Runtime};

use crate::app::QuarkAppExt;
use crate::error::Result;

#[tauri::command]
pub async fn lock<R: Runtime>(app_handle: AppHandle<R>) -> Result<()> {
    app_handle.lock()
}
