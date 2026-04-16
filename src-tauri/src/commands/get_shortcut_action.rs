use tauri::State;
use tauri_plugin_global_shortcut::Shortcut;

use crate::app::config::{AppConfig, ShortcutAction};
use crate::error::Result;

#[tauri::command]
pub async fn get_shortcut_action(
    shortcut: String,
    config: State<'_, AppConfig>,
) -> Result<Option<ShortcutAction>> {
    let shortcut: Shortcut = shortcut.parse()?;
    Ok(config.get_shortcut_action(&shortcut).cloned())
}
