use std::hash::Hash;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::date;
use crate::error::{Error, Result};

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

impl TryFrom<Vec<u8>> for Item {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        serde_json::from_slice(value.as_slice()).map_err(Into::into)
    }
}

impl Item {
    pub fn composite_key(&self) -> String {
        format!("{}/{}", self.share_id, self.id)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self).map_err(Into::into)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemRef {
    pub id: String,
    pub share_id: String,
    pub title: String,
    pub itype: String,
}

impl From<Item> for ItemRef {
    fn from(value: Item) -> Self {
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
