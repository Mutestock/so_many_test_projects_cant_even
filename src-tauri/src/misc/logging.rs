use std::fmt::Display;
use std::fs::{OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use super::directories::BASE_LOG_PATH;


#[derive(Debug)]
pub enum LogLevel {
    Info,
    Error,
    Warn,
    Trace,
    Debug,
    Fatal
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}



pub fn log(log_level: LogLevel, log_message:&str){
    let msg = format!("\nBackend: Level: {} - Message: {}", log_level.to_string(), log_message);
    let mut file = OpenOptions::new()
      .write(true)
      .append(true) // This is needed to append to file
      .open(BASE_LOG_PATH.to_owned())
      .unwrap();
    file.write_all(msg.as_bytes()).unwrap();
}