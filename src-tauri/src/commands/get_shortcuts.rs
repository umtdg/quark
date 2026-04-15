use std::collections::HashMap;

use tauri::State;
use tauri_plugin_global_shortcut::Shortcut;

use crate::app::config::{AppConfig, ShortcutAction};
use crate::error::Result;

#[tauri::command]
pub async fn get_shortcuts(
    config: State<'_, AppConfig>,
) -> Result<HashMap<Shortcut, ShortcutAction>> {
    Ok(config.get_shortcut_map())
}
