use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::item::{Item, ItemData};

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

impl ItemRef {
    pub fn composite_key(&self) -> String {
        format!("{}/{}", self.share_id, self.id)
    }
}
