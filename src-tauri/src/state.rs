use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

use crate::base64_serde;
use crate::crypto::{generate_salt, Dek, EncryptedData, KdfParams, Kek};
use crate::error::Result;
use crate::item::ItemRef;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppState {
    pub kdf: String,
    pub kdf_params: KdfParams,

    #[serde(with = "base64_serde")]
    pub salt: [u8; 16],

    pub wrapped_dek: EncryptedData,
    pub items: HashMap<String, ItemRef>,
}

impl AppState {
    pub fn new(password: &[u8]) -> Result<Self> {
        let salt = generate_salt();

        // Derive KEK from password + salt
        let kdf_params = KdfParams::new();
        let mut kek = Kek::new(password, salt, kdf_params.clone())?;

        // Generate and encrypt DEK
        let dek = Dek::new();
        let wrapped_dek = dek.encrypt(&kek)?;

        // Zeroize KEK as we don't need it anymore
        kek.zeroize();

        Ok(AppState {
            kdf: "argon2".into(),
            kdf_params,
            salt,
            wrapped_dek,
            items: HashMap::new(),
        })
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Option<Self>> {
        let path = path.as_ref();

        let exists = fs::exists(path)?;
        if !exists {
            return Ok(None);
        }

        let state_json = fs::read_to_string(path)?;
        let app_state = serde_json::from_str(&state_json)?;

        Ok(Some(app_state))
    }

    pub fn load_or_new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if let Some(app_state) = Self::load(path)? {
            return Ok(app_state);
        }

        // TODO: Prompt for password
        let mut password: Vec<u8> = b"password".into();

        let app_state = Self::new(&password)?;

        password.zeroize();

        app_state.save(path)?;
        Ok(app_state)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
        }

        let state_json = serde_json::to_string_pretty(self)?;
        fs::write(path, state_json)?;

        Ok(())
    }
}
