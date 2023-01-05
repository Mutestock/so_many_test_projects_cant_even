use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;

use super::directories::BASE_LOG_PATH;

#[derive(Debug)]
pub enum LogLevel {
    Info,
    Error,
    Warn,
    Trace,
    Debug,
    Fatal,
    None,
}

#[derive(Debug)]
pub enum LogLayer {
    Frontend,
    Backend,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for LogLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for LogLevel {
    fn from(value: &str) -> Self {
        match value {
            "Info" => LogLevel::Info,
            "Error" => LogLevel::Error,
            "Warn" => LogLevel::Warn,
            "Trace" => LogLevel::Trace,
            "Debug" => LogLevel::Debug,
            "Fatal" => LogLevel::Fatal,
            _ => {
                log(
                    LogLayer::Backend,
                    LogLevel::Error,
                    "\nError log with invalid value. This is a dev issue.",
                );
                LogLevel::None
            }
        }
    }
}

pub fn log(log_layer: LogLayer, log_level: LogLevel, log_message: &str) {
    let msg = format!(
        "\n{}: Level: {} - Message: {}",
        log_layer.to_string(),
        log_level.to_string(),
        log_message
    );
    let mut file = OpenOptions::new()
        .write(true)
        .append(true) // This is needed to append to file
        .open(BASE_LOG_PATH.to_owned())
        .unwrap();
    file.write_all(msg.as_bytes()).unwrap();
}
