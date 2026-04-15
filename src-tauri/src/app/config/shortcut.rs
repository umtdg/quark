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
    lock: Shortcut,
    refresh_items: Shortcut,
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            copy_primary: "CmdOrCtrl+C".parse().unwrap(),
            copy_secondary: "CmdOrCtrl+Shift+C".parse().unwrap(),
            copy_alt: "Cmd+Alt+C".parse().unwrap(),
            lock: "CmdOrCtrl+l".parse().unwrap(),
            refresh_items: "CmdOrCtrl+r".parse().unwrap(),
        }
    }
}

impl ShortcutConfig {
    pub fn into_map(&self) -> HashMap<Shortcut, ShortcutAction> {
        HashMap::from([
            (self.copy_primary, ShortcutAction::CopyPrimary),
            (self.copy_secondary, ShortcutAction::CopySecondary),
            (self.copy_alt, ShortcutAction::CopyAlt),
            (self.lock, ShortcutAction::Lock),
            (self.refresh_items, ShortcutAction::RefreshItems),
        ])
    }
}
