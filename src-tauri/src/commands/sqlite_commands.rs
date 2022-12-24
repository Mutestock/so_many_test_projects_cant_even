use crate::connection::{sqlite_connection::SQLITE_CONNECTION, connection_common::MindmapConnector};
use tauri::{InvokeError, AppHandle};

#[tauri::command]
pub async fn cmd_sqlite_ping() -> Result<bool, InvokeError> {        
    Ok(match SQLITE_CONNECTION.connect() {
        Ok(_) => true,
        Err(_) => false,
    })
}


