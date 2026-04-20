use clap::Parser;
use tauri::{App, AppHandle, Manager, Runtime, State, Window, WindowEvent};
use tauri_plugin_global_shortcut::{Shortcut, ShortcutEvent, ShortcutState};

use crate::app::cli::{Cli, Command};
use crate::app::config::AppConfig;
use crate::app::state::{AppState, CryptoState, ItemState, RuntimeState};
use crate::app::tray::create_icon;
use crate::app::QuarkAppExt;
use crate::error::Result;

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

impl<R: Runtime> CommandContext<R> {
    pub fn handle(&self) -> &AppHandle<R> {
        match self {
            CommandContext::FirstLaunch { app, .. } => app.handle(),
            CommandContext::SingleInstance { app_handle } => app_handle,
        }
    }
}

pub fn launch_app<R: Runtime>(
    app: App<R>,
    app_config: AppConfig,
    runtime_state: RuntimeState,
) -> Result<()> {
    log::info!("Launching application with config: {:?}", app_config);

    let _tray_icon = create_icon(app.handle())?;

    // unwrap is safe since we return Some() from the callback of load_or
    let item_state_path = runtime_state.data_dir.join(ItemState::FILE_NAME);
    let item_state: ItemState = ItemState::load_or_new(item_state_path)?;

    let crypto_state_path = runtime_state.data_dir.join(CryptoState::FILE_NAME);
    let crypto_state: Option<CryptoState> = CryptoState::load_or(&crypto_state_path, |_| {
        log::info!("No crypto state is found. Setting first_launch = true");
        runtime_state.set_first_launch(true)?;

        Ok(None)
    })?;

    app.manage(app_config);
    app.manage(runtime_state);
    app.manage(item_state);
    if let Some(crypto_state) = crypto_state {
        app.manage(crypto_state);
    }

    log::info!("Runing application");
    app.run(|_, _| {});

    Ok(())
}

pub fn handle_cli_command<R: Runtime>(command: Option<Command>, context: CommandContext<R>) {
    match command.unwrap_or(Command::Show) {
        Command::Show => match context {
            CommandContext::FirstLaunch {
                app,
                app_config,
                runtime_state,
            } => launch_app(app, *app_config, runtime_state).expect("failed to launch application"),
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

pub fn on_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            log::debug!("Closing to system tray");

            window.hide().unwrap();
            api.prevent_close();
        }
        WindowEvent::Focused(false) => {
            log::debug!("Window lost focus, hiding");

            window.hide().unwrap();
        }
        _ => (),
    }
}

pub fn on_multiple_instance<R: Runtime>(app: &AppHandle<R>, args: Vec<String>, _cwd: String) {
    log::debug!("App re-launched with args {:?}", args);

    let args = Cli::parse_from(args);
    handle_cli_command(
        args.command,
        CommandContext::SingleInstance {
            app_handle: app.clone(),
        },
    );
}

pub fn global_shortcut_handler<R: Runtime>(
    app: &AppHandle<R>,
    shortcut: &Shortcut,
    event: ShortcutEvent,
) {
    let app_config: State<'_, AppConfig> = app.state();
    if event.state() != ShortcutState::Pressed {
        return;
    }

    log::debug!("Detected global shortcut: '{}'", shortcut);
    if let Some(action) = app_config.get_global_shortcut_action(shortcut) {
        log::debug!("Action for shortcut is '{:?}'", action);
        match action {
            crate::app::config::GlobalShortcutAction::Show => {
                app.show_window().expect("failed to show main window");
            }
        }
    }
}
