use std::collections::HashMap;

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

pub type GlobalShortcutMap = HashMap<Shortcut, GlobalShortcutAction>;

impl Default for GlobalShortcutConfig {
    fn default() -> Self {
        // we accept that application panics here if parsing default shortcuts fail
        Self {
            show: "CmdOrCtrl+Shift+Space".parse().unwrap(),
        }
    }
}

impl From<&GlobalShortcutConfig> for GlobalShortcutMap {
    fn from(value: &GlobalShortcutConfig) -> Self {
        GlobalShortcutMap::from([(value.show, GlobalShortcutAction::Show)])
    }
}
