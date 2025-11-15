//! Error types for hupasiya

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for hupasiya operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for hupasiya
#[derive(Error, Debug)]
#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    /// hannahanna not found
    #[error("hannahanna (hn) not found in PATH\n\nTo fix this issue:\n  1. Install hannahanna: cargo install hannahanna\n  2. Ensure it's in your PATH\n  3. Verify installation: hn --version")]
    HnNotFound,

    /// hannahanna command failed
    #[error("hannahanna command failed: {0}\n\nTroubleshooting:\n  • Check that the workbox exists: hn list\n  • Verify your current directory is a valid repository\n  • Try running the command with verbose output")]
    HnCommandFailed(String),

    /// Session not found
    #[error("Session '{0}' not found\n\nAvailable options:\n  • List all sessions: hp list\n  • Create a new session: hp new {0}\n  • Check session name spelling")]
    SessionNotFound(String),

    /// Session already exists
    #[error("Session '{0}' already exists\n\nYou can:\n  • Switch to it: hp switch {0}\n  • View its info: hp info {0}\n  • Use a different name for your new session")]
    SessionAlreadyExists(String),

    /// Workbox not found
    #[error("Workbox '{0}' not found\n\nPossible causes:\n  • Workbox was deleted outside of hupasiya\n  • Session metadata is out of sync\n  \nTo fix:\n  • List workboxes: hn list\n  • Close and recreate the session")]
    WorkboxNotFound(String),

    /// Invalid session name
    #[error("Invalid session name '{0}': {1}\n\nSession name requirements:\n  • Must contain only alphanumeric characters, hyphens, and underscores\n  • Cannot start with a hyphen\n  • Should be descriptive and unique")]
    InvalidSessionName(String, String),

    /// Invalid agent type
    #[error("Invalid agent type '{0}'\n\nValid agent types:\n  • feature - New feature development\n  • bugfix - Bug fixes\n  • test - Test writing\n  • docs - Documentation\n  • refactor - Code refactoring\n  • research - Investigation/spike\n  • review - Code review")]
    InvalidAgentType(String),

    /// Configuration error
    #[error("Configuration error: {0}\n\nConfiguration file locations:\n  • System: /etc/hupasiya/config.yml\n  • User: ~/.config/hupasiya/config.yml\n  • Repo: ./.hupasiya.yml\n  • Local: ./.hupasiya.local.yml\n\nCheck these files for syntax errors or invalid values.")]
    ConfigError(String),

    /// File system error
    #[error("File system error: {0}\n\nCommon causes:\n  • Permission denied - check file/directory permissions\n  • Disk full - ensure sufficient disk space\n  • File locked - close any programs using the file")]
    FileSystemError(String),

    /// Parse error
    #[error("Parse error: {0}\n\nSuggestions:\n  • Check for valid YAML/JSON syntax\n  • Ensure file is properly formatted\n  • Look for special characters that need escaping")]
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
    #[error("Context directory not found: {0}\n\nTo fix:\n  • Create a session first: hp new <name>\n  • Check the session exists: hp list\n  • Verify the context directory wasn't deleted manually")]
    ContextNotFound(PathBuf),

    /// Template not found
    #[error("Template '{0}' not found\n\nOptions:\n  • List available templates: hp template list\n  • Search for templates: hp template search <query>\n  • Install from marketplace: hp template install {0}\n  • Check spelling and try again")]
    TemplateNotFound(String),

    /// Session is locked
    #[error("Session '{0}' is locked by {1}\n\nThis session is currently in use by another developer or process.\n\nYou can:\n  • Wait for the lock to be released\n  • Contact {1} to coordinate\n  • Clone the session instead: hp collab clone {0} <new-name>")]
    SessionLocked(String, String),

    /// No current session
    #[error("No current session specified\n\nYou need to either:\n  • Provide session name: hp <command> <session-name>\n  • Set HP_SESSION environment variable: export HP_SESSION=<name>\n  • Switch to a session: hp switch <name>\n\nList available sessions: hp list")]
    NoCurrentSession,

    /// AI tool failed
    #[error("AI tool failed: {0}\n\nTroubleshooting:\n  • Verify AI tool is installed and in PATH\n  • Check configuration: hp profile show <profile>\n  • Try running the AI tool manually to diagnose\n  • Check AI tool logs for more details")]
    AiToolFailed(String),

    /// Invalid input
    #[error("Invalid input: {0}\n\nPlease check:\n  • Required arguments are provided\n  • Values are in the correct format\n  • Use --help for usage information")]
    InvalidInput(String),

    /// Profile not found
    #[error("Profile '{0}' not found\n\nAvailable profiles:\n  • List all profiles: hp profile list\n  • Use default profile (omit --profile flag)\n  • Check profile name spelling")]
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

impl From<octocrab::Error> for Error {
    fn from(err: octocrab::Error) -> Self {
        Error::Other(format!("GitHub API error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::SessionNotFound("test-session".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Session 'test-session' not found"));
        assert!(msg.contains("hp list"));
        assert!(msg.contains("hp new"));
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
