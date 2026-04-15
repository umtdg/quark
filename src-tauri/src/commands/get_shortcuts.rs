use std::collections::HashMap;

use tauri::State;

use crate::app::config::{AppConfig, ShortcutAction};
use crate::app::shortcut::Shortcut;
use crate::error::Result;

#[tauri::command]
pub async fn get_shortcuts(
    config: State<'_, AppConfig>,
) -> Result<HashMap<Shortcut, ShortcutAction>> {
    Ok(config.get_shortcut_map())
}
