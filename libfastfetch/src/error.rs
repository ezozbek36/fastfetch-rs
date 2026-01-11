//! Error types for fastfetch operations

use thiserror::Error;

/// Result type that explicitly distinguishes between success, unavailable, and error states
///
/// This enum provides clear semantics for module detection:
/// - `Detected(T)`: Information was successfully detected
/// - `Unavailable`: Information cannot be detected (e.g., platform not supported, missing files)
/// - `Error(E)`: An actual error occurred during detection
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectionResult<T> {
    /// Successfully detected information
    Detected(T),
    /// Information is not available (not an error)
    Unavailable,
    /// An error occurred during detection
    Error(Error),
}

impl<T> DetectionResult<T> {
    /// Returns `true` if the result is `Detected`
    pub const fn is_detected(&self) -> bool {
        matches!(self, Self::Detected(_))
    }

    /// Returns `true` if the result is `Unavailable`
    pub const fn is_unavailable(&self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns `true` if the result is `Error`
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    /// Converts from `DetectionResult<T>` to `Option<T>`
    pub fn ok(self) -> Option<T> {
        match self {
            Self::Detected(val) => Some(val),
            _ => None,
        }
    }

    /// Maps a `DetectionResult<T>` to `DetectionResult<U>` by applying a function to a contained `Detected` value
    pub fn map<U, F>(self, f: F) -> DetectionResult<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Detected(val) => DetectionResult::Detected(f(val)),
            Self::Unavailable => DetectionResult::Unavailable,
            Self::Error(err) => DetectionResult::Error(err),
        }
    }
}

/// Common error types for fastfetch operations
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    /// Platform not supported for this module
    #[error("Platform not supported")]
    UnsupportedPlatform,
    
    /// Detection failed with a message
    #[error("Detection failed: {0}")]
    DetectionFailed(String),
    
    /// I/O error occurred
    #[error("I/O error: {0}")]
    Io(String),
    
    /// Parse error occurred
    #[error("Parse error: {0}")]
    Parse(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

/// Legacy Result type for compatibility during migration
/// This will be removed once all code is migrated to DetectionResult
#[deprecated(since = "0.1.0", note = "Use DetectionResult instead")]
pub type Result<T> = std::result::Result<Option<T>, Error>;
