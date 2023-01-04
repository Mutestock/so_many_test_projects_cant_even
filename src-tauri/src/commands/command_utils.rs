use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SqliteCommandResponse<T: Serialize> {
    pub payload: Option<T>,
    pub error_code: Option<String>,
    pub msg: Option<String>,
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
