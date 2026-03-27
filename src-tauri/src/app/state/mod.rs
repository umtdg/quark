pub(crate) mod crypto;
pub(crate) mod item;
pub(crate) mod runtime;

use std::path::PathBuf;

use tauri::{Manager, Runtime};

use crate::error::Result;
pub use crypto::CryptoState;
pub use item::ItemState;
pub use runtime::RuntimeState;

pub trait AppState: Sized {
    const FILE_NAME: &str;

    fn state_file_path<M: Manager<R>, R: Runtime>(manager: M) -> Result<PathBuf>;

    fn load<M: Manager<R>, R: Runtime>(manager: M) -> Result<Option<Self>>;

    fn save<M: Manager<R>, R: Runtime>(&self, manager: M) -> Result<()>;
}

#[macro_export]
macro_rules! impl_state {
    ($ty: ty, $file_name: expr) => {
        impl crate::app::state::AppState for $ty {
            const FILE_NAME: &str = $file_name;

            fn state_file_path<M: tauri::Manager<R>, R: tauri::Runtime>(
                manager: M,
            ) -> crate::error::Result<std::path::PathBuf> {
                Ok(crate::app::config::AppConfig::local_data_dir(manager)?.join(Self::FILE_NAME))
            }

            fn load<M: tauri::Manager<R>, R: tauri::Runtime>(
                manager: M,
            ) -> crate::error::Result<Option<Self>> {
                let path = Self::state_file_path(manager)?;

                let exists = std::fs::exists(&path)?;
                if !exists {
                    return Ok(None);
                }

                log::debug!("Reading items from JSON");
                let state_json = std::fs::read_to_string(&path)?;
                serde_json::from_str(&state_json).map_err(Into::into)
            }

            fn save<M: tauri::Manager<R>, R: tauri::Runtime>(
                &self,
                manager: M,
            ) -> crate::error::Result<()> {
                let path = Self::state_file_path(manager)?;

                if !path.exists() {
                    if let Some(parent) = path.parent() {
                        log::debug!("Creating application local data directory");
                        std::fs::create_dir_all(parent)?;
                    }
                }

                log::debug!("Serializing items to JSON");
                let state_json = serde_json::to_string_pretty(self)?;
                std::fs::write(path, state_json)?;

                Ok(())
            }
        }
    };
}
