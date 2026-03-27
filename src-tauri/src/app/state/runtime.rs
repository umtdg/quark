use std::sync::RwLock;

use crate::error::{Error, Result};

pub struct RuntimeState {
    pub first_launch: RwLock<bool>,
}

impl RuntimeState {
    pub fn new(first_launch: bool) -> Self {
        Self {
            first_launch: RwLock::new(first_launch),
        }
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
