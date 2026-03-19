use std::process::Command;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Vault {
    pub(crate) name: String,
    pub(crate) vault_id: String,
    pub(crate) share_id: String,
}

impl Vault {
    pub fn from_pass_cli() -> Result<Vec<Vault>> {
        Ok(VaultListOutput::from_pass_cli()
            .context("Failed to parse vault list JSON")?
            .vaults)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct VaultListOutput {
    vaults: Vec<Vault>,
}

impl VaultListOutput {
    pub fn from_pass_cli() -> Result<VaultListOutput> {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg("pass-cli vault list --output json");

        let output = cmd.output().context("Failed to run command")?;
        let stdout = String::from_utf8(output.stdout)
            .context("Vault list output contains non unicode characters")?;

        let json = serde_json::from_str(&stdout)?;
        Ok(json)
    }
}
