use tauri::State;

use crate::app::state::RuntimeState;
use crate::error::Result;

#[tauri::command]
pub fn is_first_launch(runtime_state: State<'_, RuntimeState>) -> Result<bool> {
    runtime_state.is_first_launch()
}
