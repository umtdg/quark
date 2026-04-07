use serde::{Deserialize, Serialize};
use totp_rs::TOTP;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize, Zeroize, ZeroizeOnDrop)]
pub struct ItemLogin {
    pub email: String,
    pub username: String,
    pub password: String,
    pub urls: Vec<String>,
    pub totp_uri: String,
}

impl ItemLogin {
    pub fn get_login(&self) -> &str {
        if self.username.len() > 0 {
            &self.username
        } else {
            &self.email
        }
    }

    pub fn get_totp_code(&self) -> Result<String> {
        let totp = TOTP::from_url(&self.totp_uri).map_err(|err| Error::Totp(err.to_string()))?;
        totp.generate_current()
            .map_err(|err| Error::Totp(err.to_string()))
    }
}
