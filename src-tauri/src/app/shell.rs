use std::collections::HashSet;
use std::ffi::OsStr;

use serde::Deserialize;
use tauri::{Manager, Runtime};
use tauri_plugin_shell::ShellExt;

use crate::error::Result;
use crate::item::{Item, Vault};

#[derive(Debug, Deserialize)]
struct VaultListOutput {
    vaults: HashSet<Vault>,
}

#[derive(Debug, Deserialize)]
struct ItemsOutput {
    items: HashSet<Item>,
}

pub async fn get_vaults<T: Manager<R>, R: Runtime, P: AsRef<OsStr>>(
    app_handle: T,
    pass_cli_path: P,
) -> Result<HashSet<Vault>> {
    log::debug!("Getting vaults from pass-cli");

    let shell = app_handle.shell();
    let output = shell
        .command(pass_cli_path)
        .args(["vault", "list", "--output", "json"])
        .output()
        .await?;

    log::trace!("Decoding pass-cli stdout");
    let stdout = String::from_utf8(output.stdout)?;

    log::trace!("Parsing pass-cli output as JSON");
    let json: VaultListOutput = serde_json::from_str(&stdout)?;

    Ok(json.vaults)
}

pub async fn get_vault_items<T: Manager<R>, R: Runtime, P: AsRef<OsStr>>(
    app_handle: T,
    pass_cli_path: P,
    share_id: &str,
) -> Result<HashSet<Item>> {
    log::debug!("Getting items from pass-cli for share {}", share_id);

    let shell = app_handle.shell();
    let output = shell
        .command(pass_cli_path)
        .args([
            "item",
            "list",
            "--share-id",
            share_id,
            "--output",
            "json",
            "--filter-type",
            "login",
        ])
        .output()
        .await?;

    log::trace!("Decoding pass-cli stdout");
    let stdout = String::from_utf8(output.stdout)?;

    log::trace!("Parsing pass-cli output as JSON");
    let json: ItemsOutput = serde_json::from_str(&stdout)?;

    Ok(json.items)
}
