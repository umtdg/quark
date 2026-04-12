use tauri::menu::{Menu, MenuEvent, MenuItem};
use tauri::tray::{MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Runtime};

use crate::error::Result;
use crate::handlers::{lock_app, show_window};

pub fn create_icon<M: Manager<R>, R: Runtime>(manager: &M) -> Result<TrayIcon<R>> {
    let menu = create_menu(manager)?;

    let tray = TrayIconBuilder::<R>::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(manager.app_handle().default_window_icon().unwrap().clone())
        .on_menu_event(on_menu_event)
        .on_tray_icon_event(on_tray_icon_event)
        .build(manager)?;

    Ok(tray)
}

pub fn create_menu<M: Manager<R>, R: Runtime>(manager: &M) -> Result<Menu<R>> {
    let quick_access_tray_item = MenuItem::with_id(manager, "show", "Show", true, None::<&str>)?;
    let lock_tray_item = MenuItem::with_id(manager, "lock", "Lock", true, None::<&str>)?;
    let quit_tray_item = MenuItem::with_id(manager, "quit", "Quit", true, None::<&str>)?;

    let tray_menu = Menu::with_items(
        manager,
        &[&quick_access_tray_item, &lock_tray_item, &quit_tray_item],
    )?;

    Ok(tray_menu)
}

fn on_menu_event<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    match event.id.as_ref() {
        "show" => {
            show_window(app).expect("failed to show main window");
        }
        "lock" => {
            lock_app(app);
        }
        "quit" => {
            log::info!("Quitting application");

            app.exit(0);
        }
        _ => {
            log::debug!("Tray menu item {:?} is not handled", event.id);
        }
    }
}

fn on_tray_icon_event<R: Runtime>(tray_icon: &TrayIcon<R>, event: TrayIconEvent) {
    // Doesn't seem to work on Linux and still shows the menu on left click
    if let TrayIconEvent::Click {
        button,
        button_state,
        ..
    } = event
    {
        match button {
            tauri::tray::MouseButton::Left if button_state == MouseButtonState::Down => {
                let app = tray_icon.app_handle();
                show_window(&app).expect("failed to show main window");
            }
            _ => {}
        }
    }
}
