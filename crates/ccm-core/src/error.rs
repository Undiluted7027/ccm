use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CcmError {
    #[error("Configuration directory not found.")]
    ConfigDirNotFound,

    #[error("I/O error for path: {path}")]
    IoError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to deserialize TOML file: {path}")]
    TomlDeserialization {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("Failed to serialize TOML for profile: {name}")]
    TomlSerialization {
        name: String,
        #[source]
        source: toml::ser::Error,
    },
    
    #[error("Profile '{0}' not found.")]
    ProfileNotFound(String),

    #[error("Profile '{0}' already exists.")]
    ProfileAlreadyExists(String),
}

// A convenient type alias for our results.
pub type Result<T> = std::result::Result<T, CcmError>;