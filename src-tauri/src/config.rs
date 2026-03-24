use config::{Config, File, FileFormat};
use serde::Deserialize;
use tauri::{AppHandle, Manager};

use crate::error::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pass_cli_path: String,
    state_file: String,
}

impl AppConfig {
    pub const DEFAULT_PASS_CLI_PATH: &str = "pass-cli";
    pub const DEFAULT_STATE_FILE: &str = "state.json";

    pub fn load(app_handle: &AppHandle) -> Result<Self> {
        let config_dir = app_handle
            .path()
            .app_config_dir()
            .map_err(|_| Error::PlatformNotSupported)?;

        let config_file = File::from(config_dir.join("config.toml"))
            .format(FileFormat::Toml)
            .required(false);

        let builder = Config::builder()
            .add_source(config_file)
            .set_default("pass_cli_path", Self::DEFAULT_PASS_CLI_PATH)?
            .set_default(
                "state_file",
                config_dir
                    .join(Self::DEFAULT_STATE_FILE)
                    .to_str()
                    .unwrap()
                    .to_string(),
            )?;

        let app_config: Self = builder.build()?.try_deserialize()?;

        Ok(app_config)
    }

    pub fn get_pass_cli_path(&self) -> &str {
        &self.pass_cli_path
    }

    pub fn get_state_file(&self) -> &str {
        &self.state_file
    }
}
