use std::fmt::Debug;

use aes_gcm::{
    aead::{rand_core::RngCore, Aead, OsRng, Payload},
    Aes256Gcm, Key, KeyInit, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::serde::base64_serde;
use crate::error::{Error, Result};

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

    pub fn to_argon2_params(&self) -> Result<Params> {
        Params::new(self.m_cost, self.t_cost, self.p_cost, Some(32)).map_err(Into::into)
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
    pub fn encrypt<'msg, 'aad, T: Into<Payload<'msg, 'aad>>>(value: T, key: &[u8]) -> Result<Self> {
        let nonce = generate_nonce();
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), value)?;

        Ok(Self {
            nonce,
            data: ciphertext,
        })
    }

    pub fn decrypt<T: TryFrom<Vec<u8>>>(&self, key: &[u8]) -> Result<T> {
        let nonce = Nonce::from_slice(&self.nonce);
        let aes256_gcm = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let plaintext = aes256_gcm.decrypt(nonce, self.data.as_ref())?;

        plaintext.try_into().map_err(|_| Error::Decoding)
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
        EncryptedData::encrypt(self.0.as_ref(), &kek.0)
    }
}

impl Debug for Dek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("***")
    }
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Kek(pub [u8; 32]);

impl Kek {
    pub fn new(password: &[u8], salt: [u8; 16], params: KdfParams) -> Result<Self> {
        let mut kek = [0; 32];

        Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            params.to_argon2_params()?,
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
