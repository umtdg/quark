use std::fs;
use std::io::BufWriter;
use std::path::Path;
use std::{collections::HashSet, io::BufReader};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::item::{items_from_pass_cli, Item};
use crate::vault::vaults_from_pass_cli;

#[derive(Deserialize, Serialize)]
pub struct AppState {
    pub items: HashSet<Item>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            items: HashSet::with_capacity(128),
        }
    }
}

impl AppState {
    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        log::debug!("Reading state from file {:?}", path);

        let exists = match fs::exists(path) {
            Ok(result) => result,
            Err(_) => false,
        };

        if !exists {
            log::debug!("No state found, create default state");
            return Ok(Self::default());
        }

        let file = fs::File::open(path).context(format!("Failed to open file {:?}", path))?;
        let reader = BufReader::new(file);

        let state: Self = serde_json::from_reader(reader).context("Invalid JSON store file")?;
        Ok(state)
    }

    pub fn to_file<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let file = fs::File::create(path).context(format!("Failed to open {:?}", path))?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, self).context("Failed to convert state to JSON")?;

        Ok(())
    }

    pub async fn items_from_cli(
        app_handle: AppHandle,
        capacity: Option<usize>,
    ) -> Result<HashSet<Item>> {
        let vaults = vaults_from_pass_cli(app_handle.clone())
            .await
            .context("Failed to get list of vaults from Pass CLI")?;

        let mut items: HashSet<Item> = HashSet::with_capacity(capacity.unwrap_or(128));

        log::debug!("Reading items from pass-cli");
        for vault in vaults {
            let share_id = vault.share_id;
            let vault_items = items_from_pass_cli(app_handle.clone(), &share_id)
                .await
                .context(format!("Failed to get items from vault {}", vault.name))?;

            items.extend(vault_items);
        }

        Ok(items)
    }
}
