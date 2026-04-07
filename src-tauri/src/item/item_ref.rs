use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::item::{Item, ItemData};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ItemRefData {
    Login { login: String, urls: Vec<String> },
    CreditCard { masked_number: String },
}

impl From<&ItemData> for ItemRefData {
    fn from(value: &ItemData) -> Self {
        match value {
            ItemData::Login(item_login) => ItemRefData::Login {
                login: item_login.get_login().to_string(),
                urls: item_login.urls.clone(),
            },
            ItemData::CreditCard(item_credit_card) => ItemRefData::CreditCard {
                masked_number: format!(
                    "{} **** {}",
                    &item_credit_card.number[0..4],
                    &item_credit_card.number[12..16]
                ),
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemRef {
    pub id: String,
    pub share_id: String,
    pub title: String,
    pub data: ItemRefData,
}

impl From<Item> for ItemRef {
    fn from(value: Item) -> Self {
        ItemRef {
            id: value.id.clone(),
            share_id: value.share_id.clone(),
            title: value.content.title.clone(),
            data: (&value.content.content).into(),
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

impl ItemRef {
    pub fn composite_key(&self) -> String {
        format!("{}/{}", self.share_id, self.id)
    }
}
