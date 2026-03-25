#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error when serializing/deserializing data")]
    Json(#[from] serde_json::Error),

    #[error("current platform may not be supported")]
    PlatformNotSupported,

    #[error("error when loading configuration")]
    Config(#[from] config::ConfigError),

    #[error("encryption error: {0}")]
    Encryption(#[from] aes_gcm::Error),

    #[error("hashing error")]
    Hash(#[from] argon2::Error),

    #[error("error when decoding decrypted data")]
    Decoding,

    #[error("error running shell command")]
    Shell(#[from] tauri_plugin_shell::Error),

    #[error("error decoding string as utf-8")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("io error")]
    Io(#[from] std::io::Error),

    #[error("tauri error")]
    Tauri(#[from] tauri::Error),

    #[error("error when locking mutex for {0}")]
    TryLock(String),

    #[error("data-encryption-key must be unlocked")]
    Locked,
}

pub type Result<T> = std::result::Result<T, Error>;

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
