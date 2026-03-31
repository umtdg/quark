use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Deserialize, Serialize, Zeroize, ZeroizeOnDrop)]
pub struct ItemCreditCard {
    pub cardholder_name: String,
    pub card_type: String,
    pub number: String,
    pub verification_number: String,
    pub expiration_date: String,
    pub pin: String,
}
