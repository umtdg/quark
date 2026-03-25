use config::{Config, File, FileFormat};
use serde::Deserialize;
use tauri::{Manager, Runtime};

use crate::error::Result;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pass_cli_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            pass_cli_path: AppConfig::DEFAULT_PASS_CLI_PATH.into(),
        }
    }
}

impl AppConfig {
    pub const DEFAULT_PASS_CLI_PATH: &str = "pass-cli";

    pub fn load<M: Manager<R>, R: Runtime>(manager: M) -> Result<Self> {
        let config_dir = AppState::config_dir(manager)?;

        let config_file = File::from(config_dir.join("config.toml"))
            .format(FileFormat::Toml)
            .required(false);

        let builder = Config::builder()
            .add_source(config_file)
            .set_default("pass_cli_path", Self::DEFAULT_PASS_CLI_PATH)?;

        let config = builder.build()?;
        let app_config: Self = config.try_deserialize()?;

        Ok(app_config)
    }

    pub fn get_pass_cli_path(&self) -> &str {
        &self.pass_cli_path
    }
}
