use std::path::PathBuf;
use std::sync::{Mutex, RwLock};
use std::time::Duration;

use tauri::{AppHandle, Runtime};
use tokio::task::JoinHandle;
use tokio::time::sleep;

use crate::app::QuarkAppExt;
use crate::error::{Error, Result};

pub struct RuntimeState {
    pub first_launch: RwLock<bool>,
    pub data_dir: PathBuf,
    pub config_dir: PathBuf,
    pub clear_clipboard_handle: Mutex<Option<JoinHandle<()>>>,
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
            clear_clipboard_handle: Mutex::new(None),
        })
    }

    pub fn set_first_launch(&self, value: bool) -> Result<()> {
        log::trace!("Waiting first_launch for write");
        let mut first_launch = self
            .first_launch
            .write()
            .map_err(|_| Error::TryLock("first_launch".into()))?;

        *first_launch = value;

        Ok(())
    }

    pub fn is_first_launch(&self) -> Result<bool> {
        log::trace!("Waiting first_launch for read");
        let first_launch = self
            .first_launch
            .read()
            .map_err(|_| Error::TryLock("data-encryption-key".into()))?;

        Ok(*first_launch)
    }

    pub fn reset_clear_clipboard_handler<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        clear_interval: Duration,
    ) -> Result<()> {
        log::trace!("Waiting clear_clipboard_handle for lock");
        let mut handle = self
            .clear_clipboard_handle
            .lock()
            .map_err(|_| Error::TryLock("clear clipboard handle".into()))?;

        if let Some(handle) = handle.take() {
            log::debug!("Abort existing handler for clearing clipboard");
            handle.abort();
        }

        let app_clone = app.clone();
        *handle = Some(tokio::spawn(async move {
            sleep(clear_interval).await;
            if let Err(err) = app_clone.clear_clipboard() {
                log::error!("Failed to clear clipboard: {}", err);
            }
        }));

        Ok(())
    }
}
