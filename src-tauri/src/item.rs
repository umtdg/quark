use std::process::Command;
use std::{collections::HashSet, hash::Hash};

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
        match &self.content {
            ItemData::Login(item_login) => &item_login.password,
            ItemData::CreditCard(item_credit_card) => &item_credit_card.verification_number,
        }
    }

    pub fn get_alt(&self) -> String {
        "Alt Field".into()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: String,
    pub share_id: String,
    pub vault_id: String,
    pub content: ItemContent,
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

#[derive(Debug, Deserialize)]
pub struct ItemsOutput {
    pub items: HashSet<Item>,
}

pub fn items_from_pass_cli(share_id: &str) -> Result<HashSet<Item>> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(format!(
        "pass-cli item list --share-id {} --output json --filter-type login",
        share_id
    ));

    let output = cmd.output().context("Failed to run command")?;
    let stdout = String::from_utf8(output.stdout)
        .context("Vault list output contains non unicode characters")?;

    let json: ItemsOutput = serde_json::from_str(&stdout)?;

    Ok(json.items)
}
