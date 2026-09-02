// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use quark_lib::error::Result;

fn main() -> Result<()> {
    // fix the PATH environment variable
    let _ = fix_path_env::fix();

    quark_lib::run()
}
