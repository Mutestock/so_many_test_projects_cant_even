use crate::connection::{sqlite_connection::SQLITE_CONNECTOR, connection_common::MindmapConnector};
use tauri::{InvokeError};

#[tauri::command]
pub async fn cmd_sqlite_ping() -> Result<bool, InvokeError> {        
    Ok(match SQLITE_CONNECTOR.connect() {
        Ok(_) => true,
        Err(_) => false,
    })
}


