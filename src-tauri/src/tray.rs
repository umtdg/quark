use anyhow::Result;
use tauri::menu::{Menu, MenuEvent, MenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder};
use tauri::{AppHandle, Manager, Runtime};

pub fn create_icon<M: Manager<R>, R: Runtime>(manager: &M) -> Result<TrayIcon<R>> {
    let menu = create_menu(manager)?;

    let tray = TrayIconBuilder::<R>::new()
        .menu(&menu)
        .on_menu_event(on_menu_event)
        .build(manager)?;

    Ok(tray)
}

pub fn create_menu<M: Manager<R>, R: Runtime>(manager: &M) -> Result<Menu<R>> {
    let quit_tray_item = MenuItem::with_id(manager, "quit", "Quit", true, None::<&str>)?;

    let tray_menu = Menu::with_items(manager, &[&quit_tray_item])?;

    Ok(tray_menu)
}

fn on_menu_event<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            log::info!("Quitting through tray");
            app.exit(0);
        }
        _ => {
            log::debug!("Tray menu item {:?} is not handled", event.id);
        }
    }
}

