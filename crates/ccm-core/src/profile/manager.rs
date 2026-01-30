use crate::error::{CcmError, Result};
use crate::paths::CcmPaths;
use crate::profile::model::Profile;
use std::path::PathBuf;
use tokio::fs;

pub struct ProfileManager {
    paths: CcmPaths,
}

impl ProfileManager {
    /// Creates a new ProfileManager.
    /// This will create the necessary directories if they don't exist.
    pub fn new() -> Result<Self> {
        let paths = CcmPaths::new()?;
        Ok(Self { paths })
    }

    /// Gets the full path for a profile given its name.
    fn get_profile_path(&self, name: &str) -> PathBuf {
        self.paths.profiles_dir().join(format!("{}.toml", name))
    }

    /// Creates and stores a new profile.
    /// Fails if a profile with the same name already exists.
    pub async fn create(&self, profile: &Profile) -> Result<()> {
        let path = self.get_profile_path(&profile.name);
        if path.exists() {
            return Err(CcmError::ProfileAlreadyExists(profile.name.clone()));
        }

        let content = toml::to_string_pretty(profile).map_err(|e| CcmError::TomlSerialization {
            name: profile.name.clone(),
            source: e,
        })?;

        fs::write(&path, content)
            .await
            .map_err(|e| CcmError::IoError {
                path,
                source: e,
            })?;

        Ok(())
    }

    /// Retrieves a profile by name.
    pub async fn get(&self, name: &str) -> Result<Profile> {
        let path = self.get_profile_path(name);
        if !path.exists() {
            return Err(CcmError::ProfileNotFound(name.to_string()));
        }

        let content = fs::read_to_string(&path)
            .await
            .map_err(|e| CcmError::IoError {
                path: path.clone(),
                source: e,
            })?;
            
        toml::from_str(&content).map_err(|e| CcmError::TomlDeserialization {
            path,
            source: e,
        })
    }

    /// Deletes a profile by name.
    pub async fn delete(&self, name: &str) -> Result<()> {
        let path = self.get_profile_path(name);
        if !path.exists() {
            return Err(CcmError::ProfileNotFound(name.to_string()));
        }
        
        fs::remove_file(&path)
            .await
            .map_err(|e| CcmError::IoError { path, source: e })
    }

    /// Lists the names of all available profiles.
    pub async fn list(&self) -> Result<Vec<String>> {
        let mut profiles = Vec::new();
        let mut entries = fs::read_dir(self.paths.profiles_dir())
            .await
            .map_err(|e| CcmError::IoError {
                path: self.paths.profiles_dir().clone(),
                source: e,
            })?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                if let Some(stem) = path.file_stem() {
                     if let Some(name) = stem.to_str() {
                        profiles.push(name.to_string());
                    }
                }
            }
        }
        profiles.sort();
        Ok(profiles)
    }
}