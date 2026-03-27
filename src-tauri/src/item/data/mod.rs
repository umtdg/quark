pub(crate) mod credit_card;
pub(crate) mod login;

pub use credit_card::ItemCreditCard;
pub use login::ItemLogin;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ItemData {
    Login(ItemLogin),
    CreditCard(ItemCreditCard),
    // SshKey,
}
