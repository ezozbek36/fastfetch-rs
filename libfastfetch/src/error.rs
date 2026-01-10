//! Error types for fastfetch operations

use std::fmt;

/// Result type alias that returns Option<T> on success
///
/// This allows modules to return None when information is unavailable
/// versus an actual error when detection fails
pub type Result<T> = std::result::Result<Option<T>, anyhow::Error>;

/// Common error types for fastfetch operations
#[derive(Debug)]
pub enum Error {
    /// Platform not supported for this module
    UnsupportedPlatform,
    /// Detection failed with a message
    DetectionFailed(String),
    /// I/O error occurred
    Io(std::io::Error),
    /// Parse error occurred
    Parse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlatform => write!(f, "Platform not supported"),
            Self::DetectionFailed(msg) => write!(f, "Detection failed: {msg}"),
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Parse(msg) => write!(f, "Parse error: {msg}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
