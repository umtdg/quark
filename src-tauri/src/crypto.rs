use aes_gcm::{
    aead::{rand_core::RngCore, Aead, OsRng},
    Aes256Gcm, Key, KeyInit, Nonce,
};
use anyhow::Result;
use argon2::{Algorithm, Argon2, Params, Version};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::base64_serde;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KdfParams {
    pub m_cost: u32,
    pub t_cost: u32,
    pub p_cost: u32,
}

impl KdfParams {
    pub fn new() -> Self {
        Self {
            m_cost: 65536,
            t_cost: 3,
            p_cost: 4,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptedData {
    #[serde(with = "base64_serde")]
    pub nonce: [u8; 12],

    #[serde(with = "base64_serde")]
    pub data: Vec<u8>,
}

impl EncryptedData {
    pub fn decrypt<T: TryFrom<Vec<u8>>>(&self, key: &[u8]) -> Result<T> {
        let nonce = Nonce::from_slice(&self.nonce);
        let aes256_gcm = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let plaintext = match aes256_gcm.decrypt(nonce, self.data.as_ref()) {
            Ok(plaintext) => plaintext,
            Err(err) => anyhow::bail!(format!("Decryption error: {:?}", err)),
        };

        plaintext
            .try_into()
            .map_err(|_| anyhow::anyhow!("Error converting decrypted data"))
    }
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Dek(pub [u8; 32]);

impl Dek {
    pub fn new() -> Self {
        Self(Self::generate())
    }

    pub fn generate() -> [u8; 32] {
        let mut dek = [0u8; 32];
        OsRng.fill_bytes(&mut dek);

        dek
    }

    pub fn encrypt(&self, kek: &Kek) -> Result<EncryptedData> {
        let nonce = generate_nonce();
        let aes256_gcm = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&kek.0));
        let ciphertext = match aes256_gcm.encrypt(Nonce::from_slice(&nonce), self.0.as_ref()) {
            Ok(ciphertext) => ciphertext,
            Err(err) => anyhow::bail!(format!(
                "Error when encrypting data-encryption-key: {:?}",
                err
            )),
        };

        Ok(EncryptedData {
            nonce,
            data: ciphertext,
        })
    }
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Kek(pub [u8; 32]);

impl Kek {
    pub fn new(password: &[u8], salt: [u8; 16], params: KdfParams) -> Result<Self, argon2::Error> {
        let mut kek = [0; 32];
        Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(params.m_cost, params.t_cost, params.p_cost, Some(32))?,
        )
        .hash_password_into(password, &salt, &mut kek)?;

        Ok(Self(kek))
    }
}

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    salt
}

pub fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    nonce
}
