use crate::error::{CcmError, Result};
use std::path::PathBuf;

/// Manages the paths for ccm's configuration and data directories.
#[derive(Debug, Clone)]
pub struct CcmPaths {
    config_dir: PathBuf,
    profiles_dir: PathBuf,
}

impl CcmPaths {
    /// Discovers the ccm paths and creates them if they don't exist.
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or(CcmError::ConfigDirNotFound)?
            .join("ccm");

        let profiles_dir = config_dir.join("profiles");

        let paths = Self {
            config_dir,
            profiles_dir,
        };
        
        // Ensure the directories exist
        std::fs::create_dir_all(paths.profiles_dir())?;

        Ok(paths)
    }

    /// Returns the path to the main configuration directory.
    /// e.g., ~/.config/ccm
    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    /// Returns the path to the profiles storage directory.
    /// e.g., ~/.config/ccm/profiles
    pub fn profiles_dir(&self) -> &PathBuf {
        &self.profiles_dir
    }
}

// Implement a custom From<std::io::Error> to simplify our `?` usage,
// though we won't need it for this specific file, it's a good pattern.
// Note: This is a simplified implementation. A real one would need a path.
// For now, we'll map errors manually where they occur.