use anyhow::Result;
use tauri::menu::{Menu, MenuEvent, MenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Runtime};

pub fn create_icon<M: Manager<R>, R: Runtime>(manager: &M) -> Result<TrayIcon<R>> {
    let menu = create_menu(manager)?;

    let tray = TrayIconBuilder::<R>::new()
        .menu(&menu)
        .on_menu_event(on_menu_event)
        .on_tray_icon_event(on_tray_icon_event)
        .show_menu_on_left_click(false)
        .build(manager)?;

    Ok(tray)
}

pub fn create_menu<M: Manager<R>, R: Runtime>(manager: &M) -> Result<Menu<R>> {
    let quick_access_tray_item =
        MenuItem::with_id(manager, "quick-access", "Quick Access", true, None::<&str>)?;
    let quit_tray_item = MenuItem::with_id(manager, "quit", "Quit", true, None::<&str>)?;

    let tray_menu = Menu::with_items(manager, &[&quick_access_tray_item, &quit_tray_item])?;

    Ok(tray_menu)
}

fn on_menu_event<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    match event.id.as_ref() {
        "quick-access" => {
            log::info!("Showing quick access through tray");
            match app.get_webview_window("main") {
                Some(window) => {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                None => log::warn!("No main window to show"),
            }
        }
        "quit" => {
            log::info!("Quitting through tray");
            app.exit(0);
        }
        _ => {
            log::debug!("Tray menu item {:?} is not handled", event.id);
        }
    }
}

fn on_tray_icon_event<R: Runtime>(tray_icon: &TrayIcon<R>, event: TrayIconEvent) {
    // Doesn't seem to work on Linux and still shows the menu on left click
    match event {
        TrayIconEvent::Click { .. } => {
            let app = tray_icon.app_handle();
            match app.get_webview_window("main") {
                Some(window) => {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                None => log::warn!("No main window to show"),
            }
        }
        _ => {}
    }
}
