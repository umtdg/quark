use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Cannot execute command: {0}")]
    Shell(#[from] tauri_plugin_shell::Error),

    #[error("Cannot find pass-cli at '{path}'")]
    PassCliNotFound { path: String },

    #[error("pass-cli is not authenticated. Run `pass-cli login` and try again")]
    PassCliAuth,

    #[error("Crypto error: {0}")]
    Encryption(#[from] aes_gcm::Error),

    #[error("Hash error: {0}")]
    Hash(#[from] argon2::Error),

    #[error("Invalid key derivation function parameters: {0}")]
    InvalidKdfParams(String),

    #[error("Decode error: {0}")]
    Decode(String),

    #[error("Incorrect password")]
    IncorrectPassword,

    #[error("Platform is not supported")]
    PlatformNotSupported,

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error when locking mutex for {0}")]
    TryLock(String),

    #[error("Application is locked")]
    Locked,

    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("Clipboard error: {0}")]
    Clipboard(#[from] tauri_plugin_clipboard_manager::Error),

    #[error("{0}")]
    Window(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Error {
    pub fn decode_error<T>(error: T::Error) -> Self
    where
        T: TryFrom<Vec<u8>>,
        T::Error: Debug,
    {
        Error::Decode(format!("{:?}", error))
    }
}

pub type Result<T> = core::result::Result<T, Error>;
