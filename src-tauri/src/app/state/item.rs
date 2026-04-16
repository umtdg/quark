use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use crate::app::crypto::{Dek, EncryptedData};
use crate::app::state::AppState;
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

    pub fn load_or_new<P: AsRef<Path>>(path: P) -> Result<Self> {
        // unwrap is safe since we return Some() from the callback of load_or
        let item_state = Self::load_or(path.as_ref(), |path| {
            log::info!("Creating empty item state");
            let item_state = Self::new();
            item_state.save(path)?;

            Ok(Some(item_state))
        })?
        .unwrap();

        Ok(item_state)
    }

    pub fn is_locked(&self) -> Result<bool> {
        log::trace!("Waiting DEK for read");
        let dek = self
            .dek
            .read()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

        Ok(dek.is_none())
    }

    pub fn lock(&self) -> Result<()> {
        log::trace!("Waiting DEK for write");
        let mut dek = self
            .dek
            .write()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

        dek.take();

        Ok(())
    }

    pub fn replace_dek(&self, new_dek: Dek) -> Result<()> {
        log::trace!("Waiting DEK for write");
        let mut dek = self
            .dek
            .write()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

        dek.replace(new_dek);

        Ok(())
    }

    pub fn extend(&self, new_items: HashSet<Item>) -> Result<()> {
        log::trace!("Waiting items for write");
        let mut items = self
            .items
            .write()
            .map_err(|_| Error::TryLock("items".into()))?;

        log::trace!("Waiting DEK for read");
        let dek = self
            .dek
            .read()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;
        let key = &dek.as_ref().ok_or(Error::Locked)?.0;

        log::debug!("Encrypting items and adding to state");
        for item in new_items {
            let item_bytes = item.to_bytes()?;
            items.insert(
                item.composite_key(),
                EncryptedData::encrypt(item_bytes.as_slice(), key)?,
            );
        }

        Ok(())
    }

    pub fn get_decrypted_item_refs(&self) -> Result<Vec<ItemRef>> {
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
        let mut decrypted_items = Vec::with_capacity(items.capacity());
        for (_, item) in items.iter() {
            decrypted_items.push(item.decrypt::<Item>(key)?.into());
        }

        log::trace!("Decrypted {} item(s)", decrypted_items.len());
        Ok(decrypted_items)
    }

    pub fn get_by_ref(&self, item_ref: &ItemRef) -> Result<Option<Item>> {
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

        let item = match items.get(&item_ref.composite_key()) {
            Some(encrypted_item) => Some(encrypted_item.decrypt(key)?),
            None => None,
        };

        Ok(item)
    }
}
