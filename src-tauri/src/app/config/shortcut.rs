use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri_plugin_global_shortcut::Shortcut;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShortcutAction {
    CopyPrimary,
    CopySecondary,
    CopyAlt,
    RefreshItems,
    Lock,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ShortcutConfig {
    copy_primary: Shortcut,
    copy_secondary: Shortcut,
    copy_alt: Shortcut,
    refresh_items: Shortcut,
    lock: Shortcut,
}

pub type ShortcutMap = HashMap<Shortcut, ShortcutAction>;

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            copy_primary: "CmdOrCtrl+C".parse().unwrap(),
            copy_secondary: "CmdOrCtrl+Shift+C".parse().unwrap(),
            copy_alt: "CmdOrCtrl+Alt+C".parse().unwrap(),
            refresh_items: "CmdOrCtrl+r".parse().unwrap(),
            lock: "CmdOrCtrl+l".parse().unwrap(),
        }
    }
}

impl From<&ShortcutConfig> for ShortcutMap {
    fn from(value: &ShortcutConfig) -> Self {
        ShortcutMap::from([
            (value.copy_primary, ShortcutAction::CopyPrimary),
            (value.copy_secondary, ShortcutAction::CopySecondary),
            (value.copy_alt, ShortcutAction::CopyAlt),
            (value.refresh_items, ShortcutAction::RefreshItems),
            (value.lock, ShortcutAction::Lock),
        ])
    }
}
