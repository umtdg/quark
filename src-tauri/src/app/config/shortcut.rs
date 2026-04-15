use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::app::shortcut::Shortcut;

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
        // we accept that application panics here if parsing default shortcuts fail
        Self {
            copy_primary: "ctrl-c".parse().unwrap(),
            copy_secondary: "ctrl-shift-c".parse().unwrap(),
            copy_alt: "ctrl-alt-c".parse().unwrap(),
            lock: "ctrl-l".parse().unwrap(),
            refresh_items: "ctrl-r".parse().unwrap(),
        }
    }
}

impl ShortcutConfig {
    pub fn into_map(self) -> HashMap<Shortcut, ShortcutAction> {
        HashMap::from([
            (self.copy_primary, ShortcutAction::CopyPrimary),
            (self.copy_secondary, ShortcutAction::CopySecondary),
            (self.copy_alt, ShortcutAction::CopyAlt),
            (self.lock, ShortcutAction::Lock),
            (self.refresh_items, ShortcutAction::RefreshItems),
        ])
    }
}
