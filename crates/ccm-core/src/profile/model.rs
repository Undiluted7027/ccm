use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a complete ccm profile, combining provider configuration
/// and ccm-specific metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub provider: ProviderConfig,
    // We will add more metadata here later
}

/// Configuration specific to an AI provider endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub base_url: String,
    pub model: String,
    pub small_fast_model: Option<String>,
    pub auth_token_source: CredentialSource,
    #[serde(default)]
    pub extra_env: HashMap<String, String>,
}

/// Defines the source for retrieving an authentication token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialSource {
    Keychain { service: String },
    Environment { var_name: String },
    Encrypted { path: PathBuf },
}