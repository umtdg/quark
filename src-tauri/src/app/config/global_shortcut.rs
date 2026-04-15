use serde::{Deserialize, Serialize};
use tauri_plugin_global_shortcut::Shortcut;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GlobalShortcutAction {
    Show,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct GlobalShortcutConfig {
    show: Shortcut,
}

impl Default for GlobalShortcutConfig {
    fn default() -> Self {
        // we accept that application panics here if parsing default shortcuts fail
        Self {
            show: "CmdOrCtrl+Alt+Space".parse().unwrap(),
        }
    }
}
