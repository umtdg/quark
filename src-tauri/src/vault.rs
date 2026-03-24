use std::{collections::HashSet, hash::Hash};

use serde::Deserialize;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

use crate::config::AppConfig;
use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct Vault {
    pub name: String,
    pub vault_id: String,
    pub share_id: String,
}

impl Eq for Vault {}

impl PartialEq for Vault {
    fn eq(&self, other: &Self) -> bool {
        self.vault_id == other.vault_id && self.share_id == other.share_id
    }
}

impl Hash for Vault {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.vault_id.hash(state);
        self.share_id.hash(state);
    }
}

#[derive(Debug, Deserialize)]
pub struct VaultListOutput {
    vaults: HashSet<Vault>,
}

pub async fn vaults_from_pass_cli(
    app_handle: &AppHandle,
    app_config: &AppConfig,
) -> Result<HashSet<Vault>> {
    log::debug!("Getting vaults from pass-cli");

    let pass_cli_path = app_config.get_pass_cli_path();

    let shell = app_handle.shell();
    let output = shell
        .command(pass_cli_path)
        .args(["vault", "list", "--output", "json"])
        .output()
        .await?;

    log::trace!("Decoding pass-cli stdout");
    let stdout = String::from_utf8(output.stdout)?;

    log::trace!("Parsing pass-cli output as JSON");
    let json: VaultListOutput = serde_json::from_str(&stdout)?;

    Ok(json.vaults)
}
