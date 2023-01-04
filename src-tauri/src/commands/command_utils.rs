use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SqliteCommandMessage<T: Serialize> {
    pub payload: Option<T>,
    pub error_code: Option<String>,
    pub msg: Option<String>,
}

pub trait CommandMessageComposable<T: Serialize> {
    fn to_command_message(result: Result<T, rusqlite::Error>) -> SqliteCommandMessage<T> {
        match result {
            Ok(v) => SqliteCommandMessage {
                payload: Some(v),
                error_code: None,
                msg: None,
            },
            Err(e) => SqliteCommandMessage {
                payload: None,
                error_code: Some(e.to_string()),
                msg: None,
            },
        }
    }
}

impl CommandMessageComposable<usize> for SqliteCommandMessage<usize> {}
impl CommandMessageComposable<bool> for SqliteCommandMessage<bool> {}
