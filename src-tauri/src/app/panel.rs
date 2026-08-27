use tauri::Manager;
use tauri_nspanel::tauri_panel;

tauri_panel! {
    panel!(MainPanel {
        config: {
            can_become_key_window: true,
            can_become_main_window: false,
            is_floating_panel: true,
            hides_on_deactivate: false,
        }
    })
}
