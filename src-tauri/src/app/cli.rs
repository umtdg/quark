use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::LevelFilter;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    #[arg(short, long, global = true)]
    pub log_level: Option<LevelFilter>,

    #[arg(short, long, global = true)]
    pub pass_cli: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    Show,
    Lock,
    Quit,
}
