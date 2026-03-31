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
