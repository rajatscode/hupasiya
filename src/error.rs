//! Error types for hupasiya

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for hupasiya operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for hupasiya
#[derive(Error, Debug)]
pub enum Error {
    /// hannahanna not found
    #[error("hannahanna (hn) not found in PATH. Please install: cargo install hannahanna")]
    HnNotFound,

    /// hannahanna command failed
    #[error("hannahanna command failed: {0}")]
    HnCommandFailed(String),

    /// Session not found
    #[error("Session '{0}' not found")]
    SessionNotFound(String),

    /// Session already exists
    #[error("Session '{0}' already exists")]
    SessionAlreadyExists(String),

    /// Workbox not found
    #[error("Workbox '{0}' not found")]
    WorkboxNotFound(String),

    /// Invalid session name
    #[error("Invalid session name '{0}': {1}")]
    InvalidSessionName(String, String),

    /// Invalid agent type
    #[error("Invalid agent type '{0}'")]
    InvalidAgentType(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// File system error
    #[error("File system error: {0}")]
    FileSystemError(String),

    /// Parse error
    #[error("Parse error: {0}")]
    ParseError(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parse error
    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// YAML parse error
    #[error("YAML parse error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// Context directory not found
    #[error("Context directory not found: {0}")]
    ContextNotFound(PathBuf),

    /// Template not found
    #[error("Template '{0}' not found")]
    TemplateNotFound(String),

    /// Session is locked
    #[error("Session '{0}' is locked by {1}")]
    SessionLocked(String, String),

    /// No current session
    #[error(
        "No current session. Either specify session name or set HP_SESSION environment variable"
    )]
    NoCurrentSession,

    /// AI tool failed
    #[error("AI tool failed: {0}")]
    AiToolFailed(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Profile not found
    #[error("Profile '{0}' not found")]
    ProfileNotFound(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Other(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::SessionNotFound("test-session".to_string());
        assert_eq!(err.to_string(), "Session 'test-session' not found");
    }

    #[test]
    fn test_hn_not_found_error() {
        let err = Error::HnNotFound;
        assert!(err.to_string().contains("hannahanna"));
        assert!(err.to_string().contains("cargo install"));
    }

    #[test]
    fn test_session_locked_error() {
        let err = Error::SessionLocked("my-session".to_string(), "alice@laptop".to_string());
        assert!(err.to_string().contains("my-session"));
        assert!(err.to_string().contains("alice@laptop"));
    }
}
