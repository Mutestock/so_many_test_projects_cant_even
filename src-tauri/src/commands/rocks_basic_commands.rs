use crate::connection::rocks_connection::{connect, rocks_get, rocks_put};
use tauri::{AppHandle, InvokeError};

#[tauri::command]
pub async fn ping(handle: AppHandle) -> Result<bool, InvokeError> {
    Ok(match connect(handle) {
        Ok(_) => true,
        Err(_) => false,
    })
}

#[tauri::command]
pub async fn cmd_rocks_put(handle: AppHandle, key: &str, value: &str) -> Result<bool, InvokeError> {
    Ok(match rocks_put(handle, key, value) {
        Ok(_) => true,
        Err(_) => false,
    })
}

#[tauri::command]
pub async fn cmd_rocks_get(handle: AppHandle, key: &str) -> Result<String, InvokeError> {
    Ok(match rocks_get(handle, key) {
        Ok(value) => value,
        Err(e) => e.to_string(),
    })
}
