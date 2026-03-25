use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use tauri::{Manager, Runtime};

use crate::config::AppConfig;
use crate::crypto::{Dek, EncryptionState};
use crate::error::{Error, Result};
use crate::item::{Item, ItemRef};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppState {
    pub encryption_state: Arc<EncryptionState>,
    pub items: Arc<RwLock<HashMap<String, ItemRef>>>,

    #[serde(skip_deserializing, skip_serializing)]
    pub dek: Arc<RwLock<Option<Dek>>>,

    #[serde(skip_deserializing, skip_serializing)]
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub const STATE_FILE_NAME: &str = "state.json";

    pub fn new<M: Manager<R>, R: Runtime>(
        manager: M,
        encryption_state: EncryptionState,
        dek: Option<Dek>,
    ) -> Result<Self> {
        Ok(Self {
            encryption_state: Arc::new(encryption_state),
            items: Arc::new(RwLock::new(HashMap::with_capacity(128))),
            dek: Arc::new(RwLock::new(dek)),
            config: Arc::new(AppConfig::load(manager)?),
        })
    }

    pub fn load<M: Manager<R>, R: Runtime>(manager: M) -> Result<Option<Self>> {
        let app_handle = manager.app_handle();
        let path = Self::state_file_path(app_handle.clone())?;

        let exists = fs::exists(&path)?;
        if !exists {
            return Ok(None);
        }

        let state_json = fs::read_to_string(&path)?;
        let mut app_state: Self = serde_json::from_str(&state_json)?;
        app_state.config = Arc::new(AppConfig::load(app_handle.clone())?);

        Ok(Some(app_state))
    }

    pub fn save<M: Manager<R>, R: Runtime>(&self, manager: M) -> Result<()> {
        let path = Self::state_file_path(manager)?;

        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
        }

        let state_json = serde_json::to_string_pretty(self)?;
        fs::write(path, state_json)?;

        Ok(())
    }

    pub fn config_dir<M: Manager<R>, R: Runtime>(manager: M) -> Result<PathBuf> {
        manager
            .path()
            .app_config_dir()
            .map_err(|_| Error::PlatformNotSupported)
    }

    pub fn local_data_dir<M: Manager<R>, R: Runtime>(manager: M) -> Result<PathBuf> {
        manager
            .path()
            .app_local_data_dir()
            .map_err(|_| Error::PlatformNotSupported)
    }

    pub fn state_file_path<M: Manager<R>, R: Runtime>(manager: M) -> Result<PathBuf> {
        Ok(Self::local_data_dir(manager)?.join(Self::STATE_FILE_NAME))
    }

    pub fn extend(&self, new_items: HashSet<Item>) -> Result<()> {
        let mut items = self
            .items
            .write()
            .map_err(|_| Error::TryLock("items".into()))?;

        for item in new_items {
            items.insert(item.composite_key(), item.into());
        }

        Ok(())
    }

    pub fn is_locked(&self) -> Result<bool> {
        let dek = self
            .dek
            .read()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

        Ok(dek.is_none())
    }
}
