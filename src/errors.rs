use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum LokiError {
    Internal(String),
}

impl Display for LokiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LokiError::Internal(message) => write!(f, "LOKI ERROR: {}", message),
        }
    }
}

impl Error for LokiError {}

impl From<std::io::Error> for LokiError {
    fn from(value: std::io::Error) -> Self {
        LokiError::Internal(format!("IO Error: {value:?}"))
    }
}
