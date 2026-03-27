use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use crate::app::crypto::{Dek, EncryptedData};
use crate::error::{Error, Result};
use crate::impl_state;
use crate::item::{Item, ItemRef};

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemState {
    pub items: RwLock<HashMap<String, EncryptedData>>,

    #[serde(skip_deserializing, skip_serializing)]
    pub dek: RwLock<Option<Dek>>,
}

impl Default for ItemState {
    fn default() -> Self {
        Self {
            items: RwLock::new(HashMap::with_capacity(128)),
            dek: RwLock::new(None),
        }
    }
}

impl_state!(ItemState, "items.json");

impl ItemState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_locked(&self) -> Result<bool> {
        log::debug!("Waiting DEK for read");
        let dek = self
            .dek
            .read()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

        Ok(dek.is_none())
    }

    pub fn replace_dek(&self, new_dek: Dek) -> Result<()> {
        log::debug!("Waiting DEK for write");
        let mut dek = self
            .dek
            .write()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

        dek.replace(new_dek);

        Ok(())
    }

    pub fn extend(&self, new_items: HashSet<Item>) -> Result<()> {
        log::debug!("Waiting items for write");
        let mut items = self
            .items
            .write()
            .map_err(|_| Error::TryLock("items".into()))?;

        log::debug!("Waiting DEK for read");
        let dek = self
            .dek
            .read()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;
        let key = &dek.as_ref().ok_or(Error::Locked)?.0;

        log::debug!("Encrypting items and adding to state");
        for item in new_items {
            log::trace!("Encrypting and inserting item: {:?}", item);

            let item_bytes = item.to_bytes()?;
            items.insert(
                item.composite_key(),
                EncryptedData::encrypt(item_bytes.as_slice(), key)?,
            );
        }

        Ok(())
    }

    pub fn get_decrypted_item_refs(&self) -> Result<HashSet<ItemRef>> {
        log::trace!("Waiting items for read");
        let items = self
            .items
            .read()
            .map_err(|_| Error::TryLock("items".into()))?;

        log::trace!("Waiting DEK for read");
        let dek = self
            .dek
            .read()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;
        let key = &dek.as_ref().ok_or(Error::Locked)?.0;

        log::trace!("Decrypting items and mapping them to refs");
        let mut decrypted_items = HashSet::with_capacity(items.capacity());
        for (item_id, item) in items.iter() {
            log::trace!("Decrypting item {}", item_id);
            decrypted_items.insert(item.decrypt::<Item>(key)?.into());
        }

        log::trace!("Decrypted {} item(s)", decrypted_items.len());
        Ok(decrypted_items)
    }
}
