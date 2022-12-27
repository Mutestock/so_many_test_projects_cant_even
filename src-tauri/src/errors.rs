use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct TryFromError(pub String);
impl Error for TryFromError {}
impl Display for TryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Attempt to use TryFrom and failed: Msg: {}", self.0)
    }
}
