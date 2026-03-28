use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

use crate::app::crypto::{generate_salt, Dek, EncryptedData, KdfParams, Kek};
use crate::error::Result;
use crate::impl_state;
use crate::serde::base64_serde;

#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoState {
    pub kdf: String,
    pub kdf_params: KdfParams,

    #[serde(with = "base64_serde")]
    pub salt: [u8; 16],

    pub wrapped_dek: EncryptedData,
}

impl_state!(CryptoState, "crypto.json");

impl CryptoState {
    pub fn new(password: &[u8]) -> Result<(Self, Dek)> {
        let salt = generate_salt();
        let kdf_params = KdfParams::new();
        let mut kek = Kek::new(password, &salt, &kdf_params)?;

        let dek = Dek::new();
        let wrapped_dek = dek.encrypt(&kek)?;

        kek.zeroize();

        Ok((
            Self {
                kdf: "argon2".into(),
                kdf_params,
                salt,
                wrapped_dek,
            },
            dek,
        ))
    }
}
