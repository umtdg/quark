use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::item::data::ItemData;

#[derive(Debug, Deserialize, Serialize, Zeroize, ZeroizeOnDrop)]
pub struct ItemContent {
    pub title: String,
    pub note: String,
    pub content: ItemData,
}

impl ItemContent {
    pub fn get_primary(&self) -> String {
        log::trace!("Get primary field from item");

        self.content.get_primary()
    }

    pub fn get_secondary(&self) -> String {
        log::trace!("Get secondary field from item");

        self.content.get_secondary()
    }

    pub fn get_alt(&self) -> String {
        log::trace!("Get alt field from item");

        self.content.get_alt()
    }
}
