use std::{path::PathBuf, sync::RwLock};

use crate::error::{Error, Result};

pub struct RuntimeState {
    pub first_launch: RwLock<bool>,
    pub data_dir: PathBuf,
    pub config_dir: PathBuf,
}

impl RuntimeState {
    pub fn new(bundle_identifier: &str, first_launch: bool) -> Result<Self> {
        let data_dir = dirs::data_dir()
            .ok_or(Error::PlatformNotSupported)?
            .join(bundle_identifier);
        let config_dir = dirs::config_dir()
            .ok_or(Error::PlatformNotSupported)?
            .join(bundle_identifier);

        Ok(Self {
            first_launch: RwLock::new(first_launch),
            data_dir,
            config_dir,
        })
    }

    pub fn set_first_launch(&self, value: bool) -> Result<()> {
        log::debug!("Waiting first_launch for write");
        let mut first_launch = self
            .first_launch
            .write()
            .map_err(|_| Error::TryLock("first_launch".into()))?;

        *first_launch = value;

        Ok(())
    }

    pub fn is_first_launch(&self) -> Result<bool> {
        log::debug!("Waiting first_launch for read");
        let first_launch = self
            .first_launch
            .read()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

        Ok(*first_launch)
    }
}
