#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("Cannot execute command: {0}")]
    Shell(#[from] tauri_plugin_shell::Error),

    #[error("Cannot find pass-cli at '{path}'")]
    PassCliNotFound { path: String },

    #[error("pass-cli is not authenticated. Run `pass-cli login` and try again")]
    PassCliAuth,

    #[error("current platform may not be supported")]
    PlatformNotSupported,

    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error(transparent)]
    Encryption(#[from] aes_gcm::Error),

    #[error(transparent)]
    Hash(#[from] argon2::Error),

    #[error("error when decoding decrypted data")]
    Decoding,

    #[error("error decoding string as utf-8")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Tauri(#[from] tauri::Error),

    #[error("error when locking mutex for {0}")]
    TryLock(String),

    #[error("Application is locked")]
    Locked,

    #[error("cannot convert vector to array: {0}")]
    VectorArrayConversion(String),

    #[error(transparent)]
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

pub type Result<T> = core::result::Result<T, Error>;
