use serde::{Deserialize, Serialize};
use totp_rs::TOTP;
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

        match &self.content {
            ItemData::Login(item_login) => {
                if !item_login.username.is_empty() {
                    item_login.username.clone()
                } else {
                    item_login.email.clone()
                }
            }
            ItemData::CreditCard(item_credit_card) => item_credit_card.number.clone(),
        }
    }

    pub fn get_secondary(&self) -> String {
        log::trace!("Get secondary field from item");

        match &self.content {
            ItemData::Login(item_login) => item_login.password.clone(),
            ItemData::CreditCard(item_credit_card) => item_credit_card.verification_number.clone(),
        }
    }

    pub fn get_alt(&self) -> String {
        log::trace!("Get alt field from item");

        match &self.content {
            ItemData::Login(item_login) => {
                match TOTP::from_url(&item_login.totp_uri) {
                    Ok(totp) => {
                        match totp.generate_current() {
                            Ok(token) => token,
                            Err(err) => {
                                log::error!("Error when generating TOTP token: {:?}", err);
                                String::new()
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("Error when parsing TOTP URL: {:?}", err);
                        String::new()
                    }
                }
            },
            ItemData::CreditCard(item_credit_card) => item_credit_card.expiration_date.clone()
        }
    }
}
