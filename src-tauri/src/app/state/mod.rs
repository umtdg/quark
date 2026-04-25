pub(crate) mod crypto;
pub(crate) mod item;
pub(crate) mod runtime;

use std::path::Path;

use crate::error::Result;
pub use crypto::CryptoState;
pub use item::ItemState;
pub use runtime::RuntimeState;

pub trait AppState: Sized {
    const FILE_NAME: &str;

    fn load<P>(path: P) -> Result<Option<Self>>
    where
        P: AsRef<Path>;

    fn save<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>;

    fn load_or<F, P>(path: P, f: F) -> Result<Option<Self>>
    where
        F: Fn(P) -> Result<Option<Self>> + Send + Sync,
        P: AsRef<Path>;
}

#[macro_export]
macro_rules! impl_state {
    ($ty: ty, $file_name: expr) => {
        impl $crate::app::state::AppState for $ty {
            const FILE_NAME: &str = $file_name;

            fn load<P>(path: P) -> $crate::error::Result<Option<Self>>
            where
                P: AsRef<std::path::Path>,
            {
                let path = path.as_ref();

                let exists = std::fs::exists(path)?;
                if !exists {
                    return Ok(None);
                }

                log::debug!("Reading state from JSON");
                let state_json = std::fs::read_to_string(&path)?;
                serde_json::from_str(&state_json).map_err(Into::into)
            }

            fn save<P>(&self, path: P) -> $crate::error::Result<()>
            where
                P: AsRef<std::path::Path>,
            {
                let path = path.as_ref();

                if !path.exists() {
                    if let Some(parent) = path.parent() {
                        log::debug!("Creating application local data directory");
                        std::fs::create_dir_all(parent)?;
                    }
                }

                log::debug!("Serializing state to JSON");
                let state_json = serde_json::to_string_pretty(self)?;
                std::fs::write(path, state_json)?;

                Ok(())
            }

            fn load_or<F, P>(path: P, f: F) -> $crate::error::Result<Option<Self>>
            where
                F: Fn(P) -> $crate::error::Result<Option<Self>> + Send + Sync,
                P: AsRef<std::path::Path>,
            {
                let state = match Self::load(path.as_ref())? {
                    Some(state) => {
                        log::info!("Loaded existing state from {}", Self::FILE_NAME);
                        Some(state)
                    }
                    None => f(path)?,
                };

                Ok(state)
            }
        }
    };
}
