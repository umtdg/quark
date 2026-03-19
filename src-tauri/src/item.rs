use std::process::Command;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemLogin {
    pub email: String,
    pub username: String,
    pub password: String,
    pub urls: Vec<String>,
    pub totp_uri: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemFields {
    // Note,
    Login(ItemLogin),
    // Alias,
    // CreditCard,
    // Identity,
    // SshKey,
    // Wifi,
    // Custom,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemContent {
    pub title: String,
    pub note: String,
    pub content: ItemFields,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: String,
    pub share_id: String,
    pub vault_id: String,
    pub content: ItemContent,
}

#[derive(Deserialize, Serialize)]
pub struct ItemRef {
    pub id: String,
    pub share_id: String,
    pub title: String,
    pub itype: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemsOutput {
    pub items: Vec<Item>,
}

pub fn items_from_pass_cli(share_id: &str) -> Result<ItemsOutput> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(format!(
        "pass-cli item list --share-id {} --output json --filter-type login",
        share_id
    ));

    let output = cmd.output().context("Failed to run command")?;
    let stdout = String::from_utf8(output.stdout)
        .context("Vault list output contains non unicode characters")?;

    let json = serde_json::from_str(&stdout)?;

    Ok(json)
}
