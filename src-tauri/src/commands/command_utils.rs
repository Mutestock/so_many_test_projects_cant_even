use serde::{Deserialize, Serialize};
use tauri::InvokeError;

use crate::misc::logging::{LogLayer, log, LogLevel};

#[derive(Serialize, Deserialize, Clone)]
pub struct SqliteCommandResponse<T: Serialize> {
    pub payload: Option<T>,
    pub error_code: Option<String>,
    pub msg: Option<String>,
}


#[tauri::command]
pub async fn cmd_log(level: String, message: String) -> Result<(), InvokeError> {
    log(LogLayer::Frontend, LogLevel::from(level.as_str()), &message);
    Ok(())
}





pub trait CommandResponseComposable<T: Serialize> {
    fn to_command_response(result: Result<T, rusqlite::Error>) -> SqliteCommandResponse<T> {
        match result {
            Ok(v) => SqliteCommandResponse {
                payload: Some(v),
                error_code: None,
                msg: None,
            },
            Err(e) => SqliteCommandResponse {
                payload: None,
                error_code: Some(e.to_string()),
                msg: None,
            },
        }
    }
    fn to_command_response_with_message(result: Result<T, rusqlite::Error>, message: String) -> SqliteCommandResponse<T> {
        match result {
            Ok(v) => SqliteCommandResponse {
                payload: Some(v),
                error_code: None,
                msg: Some(message),
            },
            Err(e) => SqliteCommandResponse {
                payload: None,
                error_code: Some(e.to_string()),
                msg: Some(message),
            },
        }
    }
}

impl CommandResponseComposable<usize> for SqliteCommandResponse<usize> {}
impl CommandResponseComposable<bool> for SqliteCommandResponse<bool> {}
