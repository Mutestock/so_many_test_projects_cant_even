use crate::connection::{connection_common::MindmapConnector, sqlite_connection::SQLITE_CONNECTOR};
use tauri::InvokeError;

#[tauri::command]
pub async fn cmd_sqlite_ping() -> Result<bool, InvokeError> {
    Ok(match SQLITE_CONNECTOR.connect() {
        Ok(_) => true,
        Err(_) => false,
    })
}
