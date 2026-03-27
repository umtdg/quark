use std::hash::Hash;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::item::content::ItemContent;
use crate::serde::date;

#[derive(Debug, Deserialize, Serialize)]
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
