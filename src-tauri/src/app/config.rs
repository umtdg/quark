use std::path::Path;

use config::{Config, File, FileFormat};
use log::LevelFilter;
use serde::Deserialize;

use crate::app::cli::Cli;
use crate::error::Result;
use crate::serde::log_level;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pass_cli_path: String,

    #[serde(with = "log_level")]
    log_level: LevelFilter,

    clear_interval: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            pass_cli_path: AppConfig::DEFAULT_PASS_CLI_PATH.into(),
            log_level: LevelFilter::Info,
            clear_interval: 120,
        }
    }
}

impl AppConfig {
    pub const FILE_NAME: &str = "config.toml";
    pub const DEFAULT_PASS_CLI_PATH: &str = "pass-cli";

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let config_file = File::from(path).format(FileFormat::Toml).required(false);

        let builder = Config::builder().add_source(config_file);

        let config = builder.build()?;
        let app_config: Self = config.try_deserialize()?;

        Ok(app_config)
    }

    pub fn merge(&mut self, cli: &Cli) {
        if let Some(log_level) = cli.log_level {
            self.log_level = log_level;
        }

        if let Some(pass_cli_path) = &cli.pass_cli {
            self.pass_cli_path = pass_cli_path.clone();
        }
    }

    pub fn get_pass_cli_path(&self) -> &str {
        &self.pass_cli_path
    }

    pub fn get_level_filter(&self) -> LevelFilter {
        self.log_level
    }

    pub fn get_clear_interval(&self) -> u32 {
        self.clear_interval
    }
}
