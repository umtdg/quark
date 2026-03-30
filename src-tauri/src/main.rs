// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use quark_lib::error::Result;

fn main() -> Result<()> {
    quark_lib::run()
}
