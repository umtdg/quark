use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::LevelFilter;
use tauri::{App, AppHandle, Runtime};

use crate::app::config::AppConfig;
use crate::app::state::RuntimeState;
use crate::app::{launch_app, QuarkAppExt};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand, help = "Subcommand")]
    pub command: Option<Command>,

    #[arg(short, long, global = true, help = "Path to config file")]
    pub config: Option<PathBuf>,

    #[arg(short, long, global = true, help = "Log level")]
    pub log_level: Option<LevelFilter>,

    #[arg(short, long, global = true, help = "Path to pass-cli executable")]
    pub pass_cli: Option<String>,
}

#[derive(Subcommand, PartialEq, Eq)]
pub enum Command {
    #[command(
        name = "version",
        visible_alias = "v",
        about = "Print application version"
    )]
    Version,

    #[command(
        name = "info",
        visible_alias = "i",
        about = "Print information about the application"
    )]
    Info,

    #[command(
        name = "show",
        visible_alias = "s",
        about = "Show window of running application or start a new application [default]"
    )]
    Show,

    #[command(name = "lock", visible_alias = "l", about = "Lock running application")]
    Lock,

    #[command(name = "quit", visible_alias = "q", about = "Quit running application")]
    Quit,
}

pub enum CommandContext<R: Runtime> {
    FirstLaunch {
        app: App<R>,
        app_config: Box<AppConfig>,
        runtime_state: RuntimeState,
    },
    SingleInstance {
        app_handle: AppHandle<R>,
    },
}

impl Cli {
    pub fn run<R: Runtime>(&self, context: CommandContext<R>) {
        self.command.as_ref().unwrap_or(&Command::Show).run(context)
    }
}

impl Command {
    pub fn run<R: Runtime>(&self, context: CommandContext<R>) {
        match self {
            Command::Show => match context {
                CommandContext::FirstLaunch {
                    app,
                    app_config,
                    runtime_state,
                } => launch_app(app, *app_config, runtime_state)
                    .expect("failed to launch application"),
                CommandContext::SingleInstance { app_handle } => app_handle
                    .show_window()
                    .expect("failed to show main window"),
            },
            Command::Lock => match context {
                CommandContext::SingleInstance { app_handle } => {
                    app_handle.lock().expect("failed to lock application")
                }
                _ => eprintln!("There is no instance of the application running"),
            },
            Command::Quit => match context {
                CommandContext::SingleInstance { app_handle } => app_handle.exit(0),
                _ => eprintln!("There is no instance of the application running"),
            },
            Command::Version => context.handle().print_version(),
            Command::Info => context.handle().print_info(),
        }
    }
}

impl<R: Runtime> CommandContext<R> {
    pub fn handle(&self) -> &AppHandle<R> {
        match self {
            CommandContext::FirstLaunch { app, .. } => app.handle(),
            CommandContext::SingleInstance { app_handle } => app_handle,
        }
    }
}
