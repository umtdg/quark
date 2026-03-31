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
    pub fn get_primary(&self) -> &str {
        log::trace!("Get primary field from item");

        match &self.content {
            ItemData::Login(item_login) => {
                if !item_login.username.is_empty() {
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
