//! Error types for ccm-core

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for ccm operations
#[derive(Error, Debug)]
pub enum CcmError {
    // Profile errors
    #[error("Profile not found: {name}")]
    ProfileNotFound { name: String },

    #[error("Profile already exists: {name}")]
    ProfileAlreadyExists { name: String },

    #[error("Invalid profile name '{name}': {reason}")]
    InvalidProfileName { name: String, reason: String },

    #[error("Invalid profile configuration: {0}")]
    InvalidProfileConfig(String),

    #[error("No default profile configured")]
    NoDefaultProfile,

    // Credential errors
    #[error("Credential not found for profile: {profile}")]
    CredentialNotFound { profile: String },

    #[error("Credential storage error: {0}")]
    CredentialStorageError(String),

    #[error("Failed to access system keychain: {0}")]
    KeychainError(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Environment variable not set: {var}")]
    EnvVarNotSet { var: String },

    // Injection errors
    #[error("Claude Code settings not found at: {path}")]
    ClaudeSettingsNotFound { path: PathBuf },

    #[error("Failed to write Claude Code settings: {0}")]
    ClaudeSettingsWriteError(String),

    #[error("Invalid Claude Code settings format: {0}")]
    InvalidClaudeSettings(String),

    // Project config errors
    #[error("Invalid .ccmrc file at {path}: {reason}")]
    InvalidCcmrc { path: PathBuf, reason: String },

    // Shell errors
    #[error("Unsupported shell: {shell}")]
    UnsupportedShell { shell: String },

    // IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    // Generic
    #[error("{0}")]
    Other(String),
}

/// Result type alias for ccm operations
pub type Result<T> = std::result::Result<T, CcmError>;

impl CcmError {
    /// Get a user-friendly suggestion for fixing this error
    pub fn suggestion(&self) -> Option<&'static str> {
        match self {
            CcmError::ProfileNotFound { .. } => {
                Some("Run 'ccm list' to see available profiles, or 'ccm add <name>' to create one.")
            }
            CcmError::NoDefaultProfile => {
                Some("Run 'ccm use <profile>' to set a default profile.")
            }
            CcmError::CredentialNotFound { .. } => {
                Some("Run 'ccm add <profile>' to set up credentials for this profile.")
            }
            CcmError::KeychainError(_) => {
                Some("Check that your system keychain is accessible. On Linux, ensure libsecret is installed.")
            }
            CcmError::ClaudeSettingsNotFound { .. } => {
                Some("Ensure Claude Code CLI is installed. Run 'claude --version' to verify.")
            }
            CcmError::EnvVarNotSet { var } => {
                Some("Set the environment variable before running ccm, or use --auth-token directly.")
            }
            _ => None,
        }
    }
}