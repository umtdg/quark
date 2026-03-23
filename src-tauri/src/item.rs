use std::{collections::HashSet, hash::Hash};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

use crate::{config::DEFAULT_PASS_CLI_PATH, date};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemLogin {
    pub email: String,
    pub username: String,
    pub password: String,
    pub urls: Vec<String>,
    pub totp_uri: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemCreditCard {
    pub cardholder_name: String,
    pub card_type: String,
    pub number: String,
    pub verification_number: String,
    pub expiration_date: String,
    pub pin: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemData {
    Login(ItemLogin),
    CreditCard(ItemCreditCard),
    // SshKey,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemContent {
    pub title: String,
    pub note: String,
    pub content: ItemData,
}

impl ItemContent {
    pub fn get_primary(&self) -> &str {
        log::trace!("Get primary field from item");

        match &self.content {
            ItemData::Login(item_login) => {
                if item_login.username.len() > 0 {
                    &item_login.username
                } else {
                    &item_login.email
                }
            }
            ItemData::CreditCard(item_credit_card) => &item_credit_card.number,
        }
    }

    pub fn get_secondary(&self) -> &str {
        log::trace!("Get secondary field from item");

        match &self.content {
            ItemData::Login(item_login) => &item_login.password,
            ItemData::CreditCard(item_credit_card) => &item_credit_card.verification_number,
        }
    }

    pub fn get_alt(&self) -> String {
        log::trace!("Get alt field from item");

        "Alt Field".into()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: String,
    pub share_id: String,
    pub vault_id: String,
    pub content: ItemContent,

    #[serde(with = "date")]
    pub create_time: DateTime<Utc>,

    #[serde(with = "date")]
    pub modify_time: DateTime<Utc>,
}

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.share_id == other.share_id
    }
}

impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.share_id.hash(state);
    }
}

#[derive(Deserialize, Serialize)]
pub struct ItemRef {
    pub id: String,
    pub share_id: String,
    pub title: String,
    pub itype: String,
}

impl From<&Item> for ItemRef {
    fn from(value: &Item) -> Self {
        ItemRef {
            id: value.id.clone(),
            share_id: value.share_id.clone(),
            title: value.content.title.clone(),
            itype: match value.content.content {
                ItemData::Login(_) => "Login".into(),
                ItemData::CreditCard(_) => "CreditCard".into(),
            },
        }
    }
}

impl Eq for ItemRef {}

impl PartialEq for ItemRef {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.share_id == other.share_id
    }
}

impl Hash for ItemRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.share_id.hash(state);
    }
}

#[derive(Debug, Deserialize)]
pub struct ItemsOutput {
    pub items: HashSet<Item>,
}

pub async fn items_from_pass_cli(app_handle: AppHandle, share_id: &str) -> Result<HashSet<Item>> {
    log::debug!("Getting items from pass-cli for share {}", share_id);

    let pass_cli_path = DEFAULT_PASS_CLI_PATH;

    log::debug!("pass-cli: {:?}", pass_cli_path);

    let shell = app_handle.shell();
    let output = shell
        .command(pass_cli_path)
        .args([
            "item",
            "list",
            "--share-id",
            share_id,
            "--output",
            "json",
            "--filter-type",
            "login",
        ])
        .output()
        .await?;

    log::trace!("Decoding pass-cli stdout");
    let stdout = String::from_utf8(output.stdout)
        .context("Vault list output contains non unicode characters")?;

    log::trace!("Parsing pass-cli output as JSON");
    let json: ItemsOutput = serde_json::from_str(&stdout)?;

    Ok(json.items)
}
