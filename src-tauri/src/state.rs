use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};
use tauri::{Manager, Runtime};

use crate::config::AppConfig;
use crate::crypto::{Dek, EncryptedData, EncryptionState};
use crate::error::{Error, Result};
use crate::item::{Item, ItemRef};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppState {
    pub encryption_state: EncryptionState,
    pub items: RwLock<HashMap<String, EncryptedData>>,

    #[serde(skip_deserializing, skip_serializing)]
    pub dek: RwLock<Option<Dek>>,
}

impl AppState {
    pub const STATE_FILE_NAME: &str = "state.json";

    pub fn state_file_path<M: Manager<R>, R: Runtime>(manager: M) -> Result<PathBuf> {
        Ok(AppConfig::local_data_dir(manager)?.join(Self::STATE_FILE_NAME))
    }

    pub fn new(encryption_state: EncryptionState, dek: Option<Dek>) -> Result<Self> {
        Ok(Self {
            encryption_state,
            items: RwLock::new(HashMap::with_capacity(128)),
            dek: RwLock::new(dek),
        })
    }

    pub fn load<M: Manager<R>, R: Runtime>(manager: M) -> Result<Option<Self>> {
        let path = Self::state_file_path(manager)?;

        let exists = fs::exists(&path)?;
        if !exists {
            return Ok(None);
        }

        let state_json = fs::read_to_string(&path)?;
        serde_json::from_str(&state_json).map_err(Into::into)
    }

    pub fn save<M: Manager<R>, R: Runtime>(&self, manager: M) -> Result<()> {
        let path = Self::state_file_path(manager)?;

        if !path.exists() {
            if let Some(parent) = path.parent() {
                log::debug!("Creating application local data directory");
                fs::create_dir_all(parent)?;
            }
        }

        log::debug!("Serializing application state to JSON");
        let state_json = serde_json::to_string_pretty(self)?;
        fs::write(path, state_json)?;

        Ok(())
    }

    pub fn extend(&self, new_items: HashSet<Item>) -> Result<()> {
        let mut items = self
            .items
            .write()
            .map_err(|_| Error::TryLock("items".into()))?;

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

    pub fn get_decrypted_item_refs(&self) -> Result<HashSet<ItemRef>> {
        log::trace!("Try locking items for read");
        let items = self
            .items
            .read()
            .map_err(|_| Error::TryLock("items".into()))?;

        log::trace!("Try locking DEK for read");
        let dek = self
            .dek
            .read()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

        if dek.is_none() {
            log::trace!("DEK is None, application is locked");
            return Err(Error::Locked);
        }

        log::trace!("Decrypting items and mapping them to refs");
        let key = &dek.as_ref().unwrap().0;
        let mut decrypted_items = HashSet::with_capacity(items.capacity());
        for (_, item) in items.iter() {
            decrypted_items.insert(item.decrypt::<Item>(key)?.into());
        }

        log::trace!("Decrypted {} item(s)", decrypted_items.len());
        Ok(decrypted_items)
    }

    pub fn is_locked(&self) -> Result<bool> {
        let dek = self
            .dek
            .read()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

        Ok(dek.is_none())
    }
}
