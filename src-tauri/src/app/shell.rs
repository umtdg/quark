use std::collections::HashSet;
use std::ffi::OsStr;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use tauri::{AppHandle, Runtime};
use tauri_plugin_shell::ShellExt;

use crate::error::{Error, Result};
use crate::item::{Item, Vault};

#[derive(Debug, Deserialize)]
struct VaultListOutput {
    vaults: HashSet<Vault>,
}

#[derive(Debug, Deserialize)]
struct ItemsOutput {
    items: HashSet<Item>,
}

async fn run_pass_cli<O, R, I, S>(app: &AppHandle<R>, path: &str, args: I) -> Result<O>
where
    O: DeserializeOwned,
    R: Runtime,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = app
        .shell()
        .command(path)
        .args(args)
        .output()
        .await
        .map_err(|shell_error| match &shell_error {
            tauri_plugin_shell::Error::Io(error) => match error.kind() {
                std::io::ErrorKind::NotFound => Error::PassCliNotFound {
                    path: path.to_string(),
                },
                _ => shell_error.into(),
            },
            _ => shell_error.into(),
        })?;

    if !output.status.success() {
        return Err(Error::PassCliAuth);
    }

    let json: O = serde_json::from_slice(&output.stdout)?;

    Ok(json)
}

pub async fn get_vaults<R: Runtime>(
    app: &AppHandle<R>,
    pass_cli_path: &str,
) -> Result<HashSet<Vault>> {
    log::debug!("Getting vaults from pass-cli");

    let output: VaultListOutput =
        run_pass_cli(app, pass_cli_path, ["vault", "list", "--output", "json"]).await?;

    Ok(output.vaults)
}

pub async fn get_vault_items<R: Runtime>(
    app: &AppHandle<R>,
    pass_cli_path: &str,
    share_id: &str,
) -> Result<HashSet<Item>> {
    log::debug!("Getting items from pass-cli for share {}", share_id);

    let output: ItemsOutput = run_pass_cli(
        app,
        pass_cli_path,
        [
            "item",
            "list",
            "--share-id",
            share_id,
            "--output",
            "json",
            "--filter-type",
            "login",
        ],
    )
    .await?;

    Ok(output.items)
}
