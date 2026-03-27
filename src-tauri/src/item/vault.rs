use std::hash::Hash;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Vault {
    pub name: String,
    pub vault_id: String,
    pub share_id: String,
}

impl Eq for Vault {}

impl PartialEq for Vault {
    fn eq(&self, other: &Self) -> bool {
        self.vault_id == other.vault_id && self.share_id == other.share_id
    }
}

impl Hash for Vault {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.vault_id.hash(state);
        self.share_id.hash(state);
    }
}
