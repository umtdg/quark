pub(crate) mod credit_card;
pub(crate) mod login;

pub use credit_card::ItemCreditCard;
pub use login::ItemLogin;

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Deserialize, Serialize, Zeroize, ZeroizeOnDrop)]
pub enum ItemData {
    Login(ItemLogin),
    CreditCard(ItemCreditCard),
    // SshKey,
}

impl ItemData {
    pub fn get_primary(&self) -> String {
        match self {
            ItemData::Login(item_login) => item_login.get_login().to_string(),
            ItemData::CreditCard(item_credit_card) => item_credit_card.number.clone(),
        }
    }

    pub fn get_secondary(&self) -> String {
        match self {
            ItemData::Login(item_login) => item_login.password.clone(),
            ItemData::CreditCard(item_credit_card) => item_credit_card.verification_number.clone(),
        }
    }

    pub fn get_alt(&self) -> String {
        match self {
            ItemData::Login(item_login) => item_login.get_totp_code().unwrap_or_default(),
            ItemData::CreditCard(item_credit_card) => item_credit_card.expiration_date.clone(),
        }
    }
}
