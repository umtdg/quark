use anyhow::Result;
use config::{Config, File, FileFormat};
use serde::Deserialize;
use tauri::{AppHandle, Manager};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pass_cli_path: String,
    store_file: String,
}

impl AppConfig {
    pub fn load(app_handle: &AppHandle) -> Result<Self> {
        let config_dir = match app_handle.path().app_config_dir() {
            Ok(config_dir) => config_dir,
            Err(_) => anyhow::bail!("Cannot get config path. This platform may not be supported"),
        };

        let config_file = File::from(config_dir.join("config.toml"))
            .format(FileFormat::Toml)
            .required(false);

        let builder = Config::builder()
            .add_source(config_file)
            .set_default("pass_cli_path", "pass-cli")?
            .set_default(
                "store_file",
                config_dir.join("items.json").to_str().unwrap().to_string(),
            )?;

        let app_config: Self = builder.build()?.try_deserialize()?;

        Ok(app_config)
    }

    pub fn get_pass_cli_path(&self) -> &str {
        &self.pass_cli_path
    }

    pub fn get_store_file(&self) -> &str {
        &self.store_file
    }
}
