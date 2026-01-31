# ccm - MVP Implementation Guide

> **Goal**: Build a fast, secure, cross-platform Claude Code configuration manager in ~5 weeks
> 
> **Target**: A CLI tool that provides fnm/nvm-style profile switching, secure credential storage, and automatic directory-based configuration switching for Claude Code

---

## Table of Contents

1. [Project Structure](#project-structure)
2. [Week 1: Foundation & Core Types](#week-1-foundation--core-types)
3. [Week 2: Profile & Credential Management](#week-2-profile--credential-management)
4. [Week 3: Configuration Injection & Shell Integration](#week-3-configuration-injection--shell-integration)
5. [Week 4: Project Config & Doctor](#week-4-project-config--doctor)
6. [Week 5: Polish, Testing & Distribution](#week-5-polish-testing--distribution)

---

## Project Structure

### Directory Layout (MVP)

```
ccm/
├── Cargo.toml                      # Workspace configuration
├── Cargo.lock
├── LICENSE
├── README.md
├── CHANGELOG.md
├── MVP_ROADMAP.md
├── ROADMAP.md
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                  # CI pipeline
│   │   ├── release.yml             # Release automation
│   │   ├── docs.yml                # Docs validation
|   |   └── security.yml            # Security audits
|   |   
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── CONTRIBUTING.md
├── crates/
│   ├── ccm/                        # Main CLI binary
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs             # Entry point
│   │       ├── cli.rs              # Clap CLI definitions
│   │       └── commands/
│   │           ├── mod.rs
│   │           ├── add.rs          # Profile creation
│   │           ├── remove.rs       # Profile deletion
│   │           ├── list.rs         # List profiles
│   │           ├── use_cmd.rs      # Switch profiles (use is reserved)
│   │           ├── current.rs      # Show current profile
│   │           ├── show.rs         # Show profile details
│   │           ├── init.rs         # Initialize .ccmrc
│   │           ├── doctor.rs       # Diagnostics
│   │           └── env.rs          # Shell integration output
│   └── ccm-core/                   # Core library
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs              # Library root
│           ├── error.rs            # Error types
│           ├── profile/
│           │   ├── mod.rs
│           │   ├── types.rs        # Profile, Provider enums
│           │   ├── manager.rs      # ProfileManager
│           │   └── validation.rs   # Profile validation
│           ├── credential/
│           │   ├── mod.rs
│           │   ├── traits.rs       # CredentialStore trait
│           │   ├── keychain.rs     # System keychain backend
│           │   ├── encrypted.rs    # Encrypted file fallback
│           │   └── env.rs          # Environment variable backend
│           ├── injector/
│           │   ├── mod.rs
│           │   ├── claude.rs       # Claude Code settings.json injector
│           │   └── backup.rs       # Backup/restore logic
│           ├── project/
│           │   ├── mod.rs
│           │   ├── ccmrc.rs        # .ccmrc parser
│           │   └── resolver.rs     # Config resolution
│           ├── shell/
│           │   ├── mod.rs
│           │   ├── bash.rs         # Bash integration
│           │   ├── zsh.rs          # Zsh integration
│           │   ├── fish.rs         # Fish integration
│           │   └── powershell.rs   # PowerShell integration
│           ├── doctor/
│           │   ├── mod.rs
│           │   ├── checks.rs       # Individual checks
│           │   └── report.rs       # Report formatting
│           └── paths.rs            # Path constants and helpers
├── tests/
│   ├── integration/
│   │   ├── mod.rs
│   │   ├── profile_lifecycle.rs
│   │   ├── credential_backends.rs
│   │   ├── shell_integration.rs
│   │   └── project_config.rs
│   └── fixtures/
│       ├── profiles/
│       │   ├── valid_anthropic.toml
│       │   └── valid_openrouter.toml
│       └── ccmrc/
│           ├── simple.toml
│           └── with_overrides.toml
├── docs/
│   ├── getting-started.md
│   ├── guides/
│   │   ├── profiles.md
│   │   ├── shell-integration.md
│   │   ├── project-config.md
│   │   └── ci-cd.md
│   └── reference/
│       ├── cli.md
│       ├── ccmrc-format.md
│       ├── guide.md
│       └── initial-architecture-design.md
├── scripts/
│   ├── install.sh                  # Unix installer
│   ├── install.ps1                 # Windows installer
│   └── release.sh                  # Release helper
└── assets/
    └── demo.gif                    # Demo GIF (post-MVP)
```

### Directory Layout (Future - Post-MVP)

```
ccm/
├── ... (MVP structure)
├── crates/
│   ├── ccm/
│   │   └── src/
│   │       └── commands/
│   │           ├── ... (MVP commands)
│   │           ├── test.rs         # Connection testing
│   │           ├── export.rs       # Profile export
│   │           └── import.rs       # Profile import
│   ├── ccm-core/
│   │   └── src/
│   │       ├── ... (MVP modules)
│   │       ├── template/           # Profile templates/presets
│   │       │   ├── mod.rs
│   │       │   ├── builtin.rs
│   │       │   └── registry.rs
│   │       ├── mcp/                # MCP server management
│   │       │   ├── mod.rs
│   │       │   └── config.rs
│   │       └── analytics/          # Usage tracking (opt-in)
│   │           ├── mod.rs
│   │           └── cost.rs
│   └── ccm-vscode/                 # VS Code extension (future)
│       └── ...
├── docs/
│   ├── ... (MVP docs)
│   └── guides/
│       ├── ... (MVP guides)
│       ├── templates.md
│       ├── mcp-management.md
│       └── cost-tracking.md
└── contrib/
    ├── homebrew/
    │   └── ccm.rb                  # Homebrew formula
    └── scoop/
        └── ccm.json                # Scoop manifest
```

---

## Week 1: Foundation & Core Types

### Day 1-2: Project Setup & Dependencies

#### 1.1 Create Workspace `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = [
    "crates/ccm",
    "crates/ccm-core",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
repository = "https://github.com/yourusername/ccm"
homepage = "https://github.com/yourusername/ccm"
description = "Claude Code Manager - fnm-style configuration management for Claude Code CLI"
keywords = ["claude", "ai", "cli", "configuration", "developer-tools"]
categories = ["command-line-utilities", "development-tools"]

[workspace.dependencies]
# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# CLI
clap = { version = "4.4", features = ["derive", "env", "wrap_help"] }
dialoguer = "0.11"
console = "0.15"
indicatif = "0.17"

# Async runtime (for potential future use)
tokio = { version = "1.35", features = ["full"] }

# Credential storage
keyring = "2.3"
aes-gcm = "0.10"
argon2 = "0.5"
rand = "0.8"

# Filesystem & paths
dirs = "5.0"
home = "0.5"
walkdir = "2.4"
tempfile = "3.9"

# Error handling
thiserror = "1.0"
anyhow = "1.0"
color-eyre = "0.6"

# Logging & tracing
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Testing
assert_cmd = "2.0"
predicates = "3.0"
assert_fs = "1.1"

# Cross-platform
which = "6.0"
```

#### 1.2 Create `crates/ccm-core/Cargo.toml`

```toml
[package]
name = "ccm-core"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
authors.workspace = true
license.workspace = true
description = "Core library for Claude Code Manager"

[dependencies]
serde.workspace = true
serde_json.workspace = true
toml.workspace = true
thiserror.workspace = true
anyhow.workspace = true
keyring.workspace = true
aes-gcm.workspace = true
argon2.workspace = true
rand.workspace = true
dirs.workspace = true
home.workspace = true
walkdir.workspace = true
tracing.workspace = true
which.workspace = true

[dev-dependencies]
tempfile.workspace = true
assert_fs.workspace = true
```

#### 1.3 Create `crates/ccm/Cargo.toml`

```toml
[package]
name = "ccm"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
authors.workspace = true
license.workspace = true
description = "Claude Code Manager - fnm-style configuration management for Claude Code CLI"

[[bin]]
name = "ccm"
path = "src/main.rs"

[dependencies]
ccm-core = { path = "../ccm-core" }
clap.workspace = true
dialoguer.workspace = true
console.workspace = true
indicatif.workspace = true
serde.workspace = true
serde_json.workspace = true
toml.workspace = true
anyhow.workspace = true
color-eyre.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
dirs.workspace = true

[dev-dependencies]
assert_cmd.workspace = true
predicates.workspace = true
assert_fs.workspace = true
tempfile.workspace = true
```

#### 1.4 Set up development environment

```bash
# Create project
mkdir ccm && cd ccm
cargo init --name ccm

# Create workspace structure
mkdir -p crates/ccm/src/commands
mkdir -p crates/ccm-core/src/{profile,credential,injector,project,shell,doctor}
mkdir -p tests/{integration,fixtures}
mkdir -p docs/{guides,reference}
mkdir -p scripts

# Initialize git
git init
cat > .gitignore << 'EOF'
/target
Cargo.lock
*.swp
*.swo
.idea/
.vscode/
*.orig
.DS_Store
EOF

# Set up pre-commit (optional)
cat > .pre-commit-config.yaml << 'EOF'
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --all --
        language: system
        types: [rust]
        pass_filenames: false
      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy --all-targets --all-features -- -D warnings
        language: system
        types: [rust]
        pass_filenames: false
EOF
```

### Day 3-4: Error Types & Path Utilities

#### 1.5 Create `crates/ccm-core/src/error.rs`

```rust
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
```

#### 1.6 Create `crates/ccm-core/src/paths.rs`

```rust
//! Path constants and helpers for ccm

use crate::error::{CcmError, Result};
use std::path::PathBuf;

/// Get the ccm configuration directory
/// 
/// Location: `~/.config/ccm` on Unix, `%APPDATA%\ccm` on Windows
pub fn config_dir() -> Result<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join("ccm"))
        .ok_or_else(|| CcmError::Other("Could not determine config directory".into()))
}

/// Get the profiles directory
/// 
/// Location: `~/.config/ccm/profiles`
pub fn profiles_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("profiles"))
}

/// Get the path to a specific profile file
/// 
/// Location: `~/.config/ccm/profiles/<name>.toml`
pub fn profile_path(name: &str) -> Result<PathBuf> {
    Ok(profiles_dir()?.join(format!("{}.toml", name)))
}

/// Get the path to the default profile marker
/// 
/// Location: `~/.config/ccm/default`
pub fn default_profile_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("default"))
}

/// Get the path to the current (active) profile marker
/// 
/// Location: `~/.config/ccm/current`
pub fn current_profile_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("current"))
}

/// Get the encrypted credentials file path (fallback storage)
/// 
/// Location: `~/.config/ccm/credentials.enc`
pub fn encrypted_credentials_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("credentials.enc"))
}

/// Get the Claude Code settings directory
/// 
/// Location: `~/.claude` on all platforms
pub fn claude_config_dir() -> Result<PathBuf> {
    home::home_dir()
        .map(|p| p.join(".claude"))
        .ok_or_else(|| CcmError::Other("Could not determine home directory".into()))
}

/// Get the Claude Code settings.json path
/// 
/// Location: `~/.claude/settings.json`
pub fn claude_settings_path() -> Result<PathBuf> {
    Ok(claude_config_dir()?.join("settings.json"))
}

/// Get the backup directory for Claude Code settings
/// 
/// Location: `~/.config/ccm/backups`
pub fn backup_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("backups"))
}

/// Ensure a directory exists, creating it if necessary
pub fn ensure_dir(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Keychain service name for credential storage
pub const KEYCHAIN_SERVICE: &str = "ccm-claude-code-manager";

/// Project config filename
pub const CCMRC_FILENAME: &str = ".ccmrc";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir_is_valid() {
        let dir = config_dir();
        assert!(dir.is_ok());
        assert!(dir.unwrap().ends_with("ccm"));
    }

    #[test]
    fn test_profile_path_format() {
        let path = profile_path("test-profile").unwrap();
        assert!(path.to_string_lossy().contains("test-profile.toml"));
    }
}
```

### Day 5-7: Profile Types & Manager

#### 1.7 Create `crates/ccm-core/src/profile/types.rs`

```rust
//! Profile type definitions

use serde::{Deserialize, Serialize};

/// Supported API providers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    /// Anthropic's official API
    Anthropic,
    /// OpenRouter (multi-model gateway)
    OpenRouter,
    /// AWS Bedrock
    Bedrock,
    /// Google Vertex AI
    VertexAi,
    /// Local Ollama server
    Ollama,
    /// Custom OpenAI-compatible endpoint
    Custom,
}

impl Provider {
    /// Get the default base URL for this provider
    pub fn default_base_url(&self) -> Option<&'static str> {
        match self {
            Provider::Anthropic => Some("https://api.anthropic.com"),
            Provider::OpenRouter => Some("https://openrouter.ai/api"),
            Provider::Ollama => Some("http://localhost:11434"),
            Provider::Bedrock => None, // Requires AWS region
            Provider::VertexAi => None, // Requires GCP project
            Provider::Custom => None,
        }
    }

    /// Get the default model for this provider
    pub fn default_model(&self) -> &'static str {
        match self {
            Provider::Anthropic => "claude-sonnet-4-5-20250929",
            Provider::OpenRouter => "anthropic/claude-sonnet-4-5",
            Provider::Bedrock => "anthropic.claude-sonnet-4-5-20250929-v1:0",
            Provider::VertexAi => "claude-sonnet-4-5@20250929",
            Provider::Ollama => "qwen2.5-coder:7b",
            Provider::Custom => "gpt-4",
        }
    }

    /// Whether this provider requires an API key
    pub fn requires_api_key(&self) -> bool {
        match self {
            Provider::Ollama => false,
            _ => true,
        }
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::Anthropic => write!(f, "anthropic"),
            Provider::OpenRouter => write!(f, "openrouter"),
            Provider::Bedrock => write!(f, "bedrock"),
            Provider::VertexAi => write!(f, "vertex-ai"),
            Provider::Ollama => write!(f, "ollama"),
            Provider::Custom => write!(f, "custom"),
        }
    }
}

impl std::str::FromStr for Provider {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "anthropic" => Ok(Provider::Anthropic),
            "openrouter" => Ok(Provider::OpenRouter),
            "bedrock" | "aws-bedrock" => Ok(Provider::Bedrock),
            "vertex-ai" | "vertexai" | "vertex" | "google" => Ok(Provider::VertexAi),
            "ollama" | "local" => Ok(Provider::Ollama),
            "custom" | "openai-compatible" => Ok(Provider::Custom),
            _ => Err(format!("Unknown provider: {}", s)),
        }
    }
}

/// Source of API credentials
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialSource {
    /// Stored in system keychain
    Keychain,
    /// Read from environment variable at runtime
    EnvVar { var_name: String },
    /// No credentials required (e.g., local Ollama)
    None,
}

impl Default for CredentialSource {
    fn default() -> Self {
        CredentialSource::Keychain
    }
}

/// A ccm profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Profile display name (also used as filename)
    pub name: String,

    /// API provider
    pub provider: Provider,

    /// API base URL (optional, uses provider default if not set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,

    /// Model identifier
    pub model: String,

    /// Where credentials are stored
    #[serde(default)]
    pub credential_source: CredentialSource,

    /// Request timeout in milliseconds
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Creation timestamp (RFC 3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Last modified timestamp (RFC 3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

fn default_timeout() -> u64 {
    60_000 // 60 seconds
}

impl Profile {
    /// Create a new profile with defaults for the given provider
    pub fn new(name: String, provider: Provider) -> Self {
        let now = chrono_lite_now();
        
        Self {
            name,
            model: provider.default_model().to_string(),
            base_url: provider.default_base_url().map(String::from),
            credential_source: if provider.requires_api_key() {
                CredentialSource::Keychain
            } else {
                CredentialSource::None
            },
            provider,
            timeout_ms: default_timeout(),
            description: None,
            created_at: Some(now.clone()),
            updated_at: Some(now),
        }
    }

    /// Get the effective base URL (profile override or provider default)
    pub fn effective_base_url(&self) -> Option<&str> {
        self.base_url
            .as_deref()
            .or_else(|| self.provider.default_base_url())
    }

    /// Validate the profile configuration
    pub fn validate(&self) -> std::result::Result<(), String> {
        // Name validation
        if self.name.is_empty() {
            return Err("Profile name cannot be empty".into());
        }
        if self.name.contains(std::path::MAIN_SEPARATOR) {
            return Err("Profile name cannot contain path separators".into());
        }
        if self.name.starts_with('.') {
            return Err("Profile name cannot start with a dot".into());
        }

        // Model validation
        if self.model.is_empty() {
            return Err("Model cannot be empty".into());
        }

        // Credential validation
        if self.provider.requires_api_key() {
            if matches!(self.credential_source, CredentialSource::None) {
                return Err(format!(
                    "Provider '{}' requires credentials, but credential_source is 'none'",
                    self.provider
                ));
            }
        }

        Ok(())
    }
}

/// Simple RFC 3339 timestamp (avoid chrono dependency for MVP)
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    
    // This is a simplified ISO 8601 timestamp
    // For production, consider using the `time` crate
    format!("{}", duration.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_from_str() {
        assert_eq!("anthropic".parse::<Provider>().unwrap(), Provider::Anthropic);
        assert_eq!("ollama".parse::<Provider>().unwrap(), Provider::Ollama);
        assert_eq!("local".parse::<Provider>().unwrap(), Provider::Ollama);
    }

    #[test]
    fn test_profile_validation() {
        let mut profile = Profile::new("test".into(), Provider::Anthropic);
        assert!(profile.validate().is_ok());

        profile.name = "".into();
        assert!(profile.validate().is_err());
    }

    #[test]
    fn test_profile_serialization() {
        let profile = Profile::new("test".into(), Provider::Anthropic);
        let toml = toml::to_string(&profile).unwrap();
        let parsed: Profile = toml::from_str(&toml).unwrap();
        assert_eq!(profile.name, parsed.name);
        assert_eq!(profile.provider, parsed.provider);
    }
}
```

#### 1.8 Create `crates/ccm-core/src/profile/validation.rs`

```rust
//! Profile validation utilities

use super::types::Profile;
use crate::error::{CcmError, Result};
use std::collections::HashSet;

/// Reserved profile names that cannot be used
const RESERVED_NAMES: &[&str] = &[
    "default",
    "current",
    "none",
    "all",
    "list",
    "help",
    "version",
];

/// Characters not allowed in profile names
const INVALID_CHARS: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|', ' ', '\t', '\n'];

/// Validate a profile name
pub fn validate_profile_name(name: &str) -> Result<()> {
    // Check empty
    if name.is_empty() {
        return Err(CcmError::InvalidProfileName {
            name: name.into(),
            reason: "Profile name cannot be empty".into(),
        });
    }

    // Check length
    if name.len() > 64 {
        return Err(CcmError::InvalidProfileName {
            name: name.into(),
            reason: "Profile name cannot exceed 64 characters".into(),
        });
    }

    // Check reserved names
    if RESERVED_NAMES.contains(&name.to_lowercase().as_str()) {
        return Err(CcmError::InvalidProfileName {
            name: name.into(),
            reason: format!("'{}' is a reserved name", name),
        });
    }

    // Check starting character
    if name.starts_with('.') || name.starts_with('-') {
        return Err(CcmError::InvalidProfileName {
            name: name.into(),
            reason: "Profile name cannot start with '.' or '-'".into(),
        });
    }

    // Check invalid characters
    for c in INVALID_CHARS {
        if name.contains(*c) {
            return Err(CcmError::InvalidProfileName {
                name: name.into(),
                reason: format!("Profile name contains invalid character: '{}'", c),
            });
        }
    }

    // Check it's valid UTF-8 filename on all platforms
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(CcmError::InvalidProfileName {
            name: name.into(),
            reason: "Profile name can only contain alphanumeric characters, hyphens, and underscores".into(),
        });
    }

    Ok(())
}

/// Validate URL format
pub fn validate_url(url: &str) -> Result<()> {
    if url.is_empty() {
        return Err(CcmError::InvalidProfileConfig("URL cannot be empty".into()));
    }

    // Simple URL validation (not comprehensive, but catches common mistakes)
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(CcmError::InvalidProfileConfig(
            "URL must start with http:// or https://".into(),
        ));
    }

    Ok(())
}

/// Validate a complete profile
pub fn validate_profile(profile: &Profile) -> Result<()> {
    validate_profile_name(&profile.name)?;

    if let Some(ref url) = profile.base_url {
        validate_url(url)?;
    }

    profile.validate().map_err(CcmError::InvalidProfileConfig)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_names() {
        assert!(validate_profile_name("my-profile").is_ok());
        assert!(validate_profile_name("profile_123").is_ok());
        assert!(validate_profile_name("Anthropic").is_ok());
    }

    #[test]
    fn test_invalid_names() {
        assert!(validate_profile_name("").is_err());
        assert!(validate_profile_name("default").is_err());
        assert!(validate_profile_name(".hidden").is_err());
        assert!(validate_profile_name("has space").is_err());
        assert!(validate_profile_name("has/slash").is_err());
    }

    #[test]
    fn test_url_validation() {
        assert!(validate_url("https://api.anthropic.com").is_ok());
        assert!(validate_url("http://localhost:11434").is_ok());
        assert!(validate_url("ftp://invalid.com").is_err());
        assert!(validate_url("not-a-url").is_err());
    }
}
```

#### 1.9 Create `crates/ccm-core/src/profile/manager.rs`

```rust
//! Profile management operations

use super::types::{CredentialSource, Profile, Provider};
use super::validation::{validate_profile, validate_profile_name};
use crate::error::{CcmError, Result};
use crate::paths::{
    current_profile_path, default_profile_path, ensure_dir, profile_path, profiles_dir,
};
use std::fs;
use std::path::Path;

/// Manages profile CRUD operations
pub struct ProfileManager;

impl ProfileManager {
    /// Create a new profile
    pub fn create(profile: &Profile) -> Result<()> {
        validate_profile(profile)?;

        let path = profile_path(&profile.name)?;

        // Check if already exists
        if path.exists() {
            return Err(CcmError::ProfileAlreadyExists {
                name: profile.name.clone(),
            });
        }

        // Ensure profiles directory exists
        ensure_dir(&profiles_dir()?)?;

        // Serialize and write
        let toml_content = toml::to_string_pretty(profile)?;
        fs::write(&path, toml_content)?;

        tracing::info!("Created profile '{}' at {:?}", profile.name, path);
        Ok(())
    }

    /// Read a profile by name
    pub fn read(name: &str) -> Result<Profile> {
        let path = profile_path(name)?;

        if !path.exists() {
            return Err(CcmError::ProfileNotFound { name: name.into() });
        }

        let content = fs::read_to_string(&path)?;
        let profile: Profile = toml::from_str(&content)?;

        Ok(profile)
    }

    /// Update an existing profile
    pub fn update(profile: &Profile) -> Result<()> {
        validate_profile(profile)?;

        let path = profile_path(&profile.name)?;

        if !path.exists() {
            return Err(CcmError::ProfileNotFound {
                name: profile.name.clone(),
            });
        }

        // Update timestamp
        let mut updated = profile.clone();
        updated.updated_at = Some(format!("{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()));

        let toml_content = toml::to_string_pretty(&updated)?;
        fs::write(&path, toml_content)?;

        tracing::info!("Updated profile '{}'", profile.name);
        Ok(())
    }

    /// Delete a profile
    pub fn delete(name: &str) -> Result<()> {
        let path = profile_path(name)?;

        if !path.exists() {
            return Err(CcmError::ProfileNotFound { name: name.into() });
        }

        fs::remove_file(&path)?;

        // If this was the default, clear default marker
        if let Ok(default) = Self::get_default() {
            if default == name {
                let _ = fs::remove_file(default_profile_path()?);
            }
        }

        // If this was current, clear current marker
        if let Ok(current) = Self::get_current() {
            if current == name {
                let _ = fs::remove_file(current_profile_path()?);
            }
        }

        tracing::info!("Deleted profile '{}'", name);
        Ok(())
    }

    /// List all profile names
    pub fn list() -> Result<Vec<String>> {
        let dir = profiles_dir()?;

        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut profiles = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "toml") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    profiles.push(name.to_string());
                }
            }
        }

        profiles.sort();
        Ok(profiles)
    }

    /// List all profiles with full details
    pub fn list_full() -> Result<Vec<Profile>> {
        let names = Self::list()?;
        let mut profiles = Vec::new();

        for name in names {
            match Self::read(&name) {
                Ok(profile) => profiles.push(profile),
                Err(e) => {
                    tracing::warn!("Failed to read profile '{}': {}", name, e);
                }
            }
        }

        Ok(profiles)
    }

    /// Check if a profile exists
    pub fn exists(name: &str) -> Result<bool> {
        let path = profile_path(name)?;
        Ok(path.exists())
    }

    /// Get the default profile name
    pub fn get_default() -> Result<String> {
        let path = default_profile_path()?;

        if !path.exists() {
            return Err(CcmError::NoDefaultProfile);
        }

        let name = fs::read_to_string(&path)?.trim().to_string();

        // Verify the profile still exists
        if !Self::exists(&name)? {
            return Err(CcmError::ProfileNotFound { name });
        }

        Ok(name)
    }

    /// Set the default profile
    pub fn set_default(name: &str) -> Result<()> {
        // Verify profile exists
        if !Self::exists(name)? {
            return Err(CcmError::ProfileNotFound { name: name.into() });
        }

        let path = default_profile_path()?;
        ensure_dir(&path.parent().unwrap().to_path_buf())?;
        fs::write(&path, name)?;

        tracing::info!("Set default profile to '{}'", name);
        Ok(())
    }

    /// Get the currently active profile name
    pub fn get_current() -> Result<String> {
        let path = current_profile_path()?;

        if !path.exists() {
            // Fall back to default
            return Self::get_default();
        }

        let name = fs::read_to_string(&path)?.trim().to_string();

        // Verify the profile still exists
        if !Self::exists(&name)? {
            // Fall back to default
            return Self::get_default();
        }

        Ok(name)
    }

    /// Set the currently active profile
    pub fn set_current(name: &str) -> Result<()> {
        // Verify profile exists
        if !Self::exists(name)? {
            return Err(CcmError::ProfileNotFound { name: name.into() });
        }

        let path = current_profile_path()?;
        ensure_dir(&path.parent().unwrap().to_path_buf())?;
        fs::write(&path, name)?;

        tracing::info!("Set current profile to '{}'", name);
        Ok(())
    }

    /// Clear the current profile (revert to default)
    pub fn clear_current() -> Result<()> {
        let path = current_profile_path()?;
        
        if path.exists() {
            fs::remove_file(&path)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    // Note: These tests would need to mock the paths module
    // or use a test-specific configuration directory
}
```

#### 1.10 Create `crates/ccm-core/src/profile/mod.rs`

```rust
//! Profile management module

pub mod manager;
pub mod types;
pub mod validation;

pub use manager::ProfileManager;
pub use types::{CredentialSource, Profile, Provider};
pub use validation::{validate_profile, validate_profile_name};
```

---

## Week 2: Profile & Credential Management

### Day 8-10: Credential Storage System

#### 2.1 Create `crates/ccm-core/src/credential/traits.rs`

```rust
//! Credential storage traits

use crate::error::Result;

/// Trait for credential storage backends
pub trait CredentialStore: Send + Sync {
    /// Store a credential for a profile
    fn store(&self, profile_name: &str, credential: &str) -> Result<()>;

    /// Retrieve a credential for a profile
    fn retrieve(&self, profile_name: &str) -> Result<String>;

    /// Delete a credential for a profile
    fn delete(&self, profile_name: &str) -> Result<()>;

    /// Check if a credential exists for a profile
    fn exists(&self, profile_name: &str) -> Result<bool>;

    /// Get the backend name (for diagnostics)
    fn backend_name(&self) -> &'static str;
}
```

#### 2.2 Create `crates/ccm-core/src/credential/keychain.rs`

```rust
//! System keychain credential storage

use super::traits::CredentialStore;
use crate::error::{CcmError, Result};
use crate::paths::KEYCHAIN_SERVICE;

/// System keychain credential storage
/// 
/// Uses:
/// - macOS: Keychain
/// - Linux: libsecret (GNOME Keyring, KWallet via Secret Service API)
/// - Windows: Credential Manager
pub struct KeychainStore;

impl KeychainStore {
    pub fn new() -> Self {
        Self
    }

    fn entry(&self, profile_name: &str) -> keyring::Entry {
        keyring::Entry::new(KEYCHAIN_SERVICE, profile_name)
            .expect("Failed to create keyring entry")
    }
}

impl Default for KeychainStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialStore for KeychainStore {
    fn store(&self, profile_name: &str, credential: &str) -> Result<()> {
        let entry = self.entry(profile_name);
        
        entry.set_password(credential).map_err(|e| {
            CcmError::KeychainError(format!("Failed to store credential: {}", e))
        })?;

        tracing::debug!("Stored credential for profile '{}' in keychain", profile_name);
        Ok(())
    }

    fn retrieve(&self, profile_name: &str) -> Result<String> {
        let entry = self.entry(profile_name);

        entry.get_password().map_err(|e| match e {
            keyring::Error::NoEntry => CcmError::CredentialNotFound {
                profile: profile_name.to_string(),
            },
            _ => CcmError::KeychainError(format!("Failed to retrieve credential: {}", e)),
        })
    }

    fn delete(&self, profile_name: &str) -> Result<()> {
        let entry = self.entry(profile_name);

        entry.delete_credential().map_err(|e| match e {
            keyring::Error::NoEntry => CcmError::CredentialNotFound {
                profile: profile_name.to_string(),
            },
            _ => CcmError::KeychainError(format!("Failed to delete credential: {}", e)),
        })?;

        tracing::debug!("Deleted credential for profile '{}' from keychain", profile_name);
        Ok(())
    }

    fn exists(&self, profile_name: &str) -> Result<bool> {
        match self.retrieve(profile_name) {
            Ok(_) => Ok(true),
            Err(CcmError::CredentialNotFound { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    fn backend_name(&self) -> &'static str {
        #[cfg(target_os = "macos")]
        return "macOS Keychain";
        
        #[cfg(target_os = "linux")]
        return "Secret Service (libsecret)";
        
        #[cfg(target_os = "windows")]
        return "Windows Credential Manager";
        
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        return "System Keychain";
    }
}

/// Check if the system keychain is available
pub fn is_keychain_available() -> bool {
    let store = KeychainStore::new();
    
    // Try to create and delete a test entry
    let test_profile = "__ccm_keychain_test__";
    let test_value = "test";

    match store.store(test_profile, test_value) {
        Ok(_) => {
            let _ = store.delete(test_profile);
            true
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires actual keychain access
    fn test_keychain_roundtrip() {
        let store = KeychainStore::new();
        let profile = "test-profile";
        let credential = "test-api-key";

        // Store
        store.store(profile, credential).unwrap();

        // Retrieve
        let retrieved = store.retrieve(profile).unwrap();
        assert_eq!(retrieved, credential);

        // Delete
        store.delete(profile).unwrap();

        // Verify deleted
        assert!(!store.exists(profile).unwrap());
    }
}
```

#### 2.3 Create `crates/ccm-core/src/credential/encrypted.rs`

```rust
//! Encrypted file credential storage (fallback when keychain unavailable)

use super::traits::CredentialStore;
use crate::error::{CcmError, Result};
use crate::paths::encrypted_credentials_path;

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::Argon2;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

/// Encrypted file credential storage
/// 
/// Uses AES-256-GCM encryption with Argon2id key derivation.
/// Falls back to this when system keychain is unavailable.
pub struct EncryptedFileStore {
    /// Master password (derived from machine ID or user input)
    password: String,
}

#[derive(Serialize, Deserialize)]
struct EncryptedCredentials {
    /// Salt for key derivation
    salt: Vec<u8>,
    /// Nonce for AES-GCM
    nonce: Vec<u8>,
    /// Encrypted credential data
    ciphertext: Vec<u8>,
}

#[derive(Serialize, Deserialize, Default)]
struct CredentialData {
    /// Map of profile name to API key
    credentials: HashMap<String, String>,
}

impl EncryptedFileStore {
    /// Create a new encrypted file store with the given master password
    pub fn new(password: String) -> Self {
        Self { password }
    }

    /// Create with auto-derived password from machine ID
    pub fn with_machine_id() -> Result<Self> {
        let machine_id = get_machine_id()?;
        Ok(Self::new(machine_id))
    }

    fn derive_key(&self, salt: &[u8]) -> Result<[u8; 32]> {
        let mut key = [0u8; 32];
        
        Argon2::default()
            .hash_password_into(self.password.as_bytes(), salt, &mut key)
            .map_err(|e| CcmError::EncryptionError(format!("Key derivation failed: {}", e)))?;

        Ok(key)
    }

    fn load_credentials(&self) -> Result<CredentialData> {
        let path = encrypted_credentials_path()?;

        if !path.exists() {
            return Ok(CredentialData::default());
        }

        let content = fs::read(&path)?;
        let encrypted: EncryptedCredentials = serde_json::from_slice(&content)?;

        let key = self.derive_key(&encrypted.salt)?;
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| CcmError::EncryptionError(format!("Cipher init failed: {}", e)))?;

        let nonce = Nonce::from_slice(&encrypted.nonce);
        let plaintext = cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|e| CcmError::EncryptionError(format!("Decryption failed: {}", e)))?;

        let data: CredentialData = serde_json::from_slice(&plaintext)?;
        Ok(data)
    }

    fn save_credentials(&self, data: &CredentialData) -> Result<()> {
        let path = encrypted_credentials_path()?;

        // Generate random salt and nonce
        let mut salt = [0u8; 16];
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut nonce_bytes);

        let key = self.derive_key(&salt)?;
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| CcmError::EncryptionError(format!("Cipher init failed: {}", e)))?;

        let nonce = Nonce::from_slice(&nonce_bytes);
        let plaintext = serde_json::to_vec(data)?;
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| CcmError::EncryptionError(format!("Encryption failed: {}", e)))?;

        let encrypted = EncryptedCredentials {
            salt: salt.to_vec(),
            nonce: nonce_bytes.to_vec(),
            ciphertext,
        };

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_vec(&encrypted)?;
        fs::write(&path, content)?;

        Ok(())
    }
}

impl CredentialStore for EncryptedFileStore {
    fn store(&self, profile_name: &str, credential: &str) -> Result<()> {
        let mut data = self.load_credentials()?;
        data.credentials.insert(profile_name.to_string(), credential.to_string());
        self.save_credentials(&data)?;

        tracing::debug!("Stored credential for profile '{}' in encrypted file", profile_name);
        Ok(())
    }

    fn retrieve(&self, profile_name: &str) -> Result<String> {
        let data = self.load_credentials()?;
        
        data.credentials
            .get(profile_name)
            .cloned()
            .ok_or_else(|| CcmError::CredentialNotFound {
                profile: profile_name.to_string(),
            })
    }

    fn delete(&self, profile_name: &str) -> Result<()> {
        let mut data = self.load_credentials()?;
        
        if data.credentials.remove(profile_name).is_none() {
            return Err(CcmError::CredentialNotFound {
                profile: profile_name.to_string(),
            });
        }

        self.save_credentials(&data)?;

        tracing::debug!("Deleted credential for profile '{}' from encrypted file", profile_name);
        Ok(())
    }

    fn exists(&self, profile_name: &str) -> Result<bool> {
        let data = self.load_credentials()?;
        Ok(data.credentials.contains_key(profile_name))
    }

    fn backend_name(&self) -> &'static str {
        "Encrypted File"
    }
}

/// Get a machine-specific identifier for automatic password derivation
fn get_machine_id() -> Result<String> {
    // Try various sources of machine identity
    
    #[cfg(target_os = "linux")]
    {
        if let Ok(id) = fs::read_to_string("/etc/machine-id") {
            return Ok(id.trim().to_string());
        }
        if let Ok(id) = fs::read_to_string("/var/lib/dbus/machine-id") {
            return Ok(id.trim().to_string());
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("IOPlatformUUID") {
                    if let Some(uuid) = line.split('"').nth(3) {
                        return Ok(uuid.to_string());
                    }
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("wmic")
            .args(["csproduct", "get", "uuid"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().nth(1) {
                let uuid = line.trim();
                if !uuid.is_empty() {
                    return Ok(uuid.to_string());
                }
            }
        }
    }

    // Fallback: use hostname + username
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    Ok(format!("{}@{}", username, hostname))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_encrypted_store_roundtrip() {
        // This test needs to mock the paths module
        // For now, just test the encryption logic conceptually
    }
}
```

#### 2.4 Create `crates/ccm-core/src/credential/env.rs`

```rust
//! Environment variable credential source

use super::traits::CredentialStore;
use crate::error::{CcmError, Result};
use std::env;

/// Environment variable credential source
/// 
/// Reads credentials from environment variables at runtime.
/// Useful for CI/CD environments.
pub struct EnvVarStore {
    /// The environment variable name to read
    var_name: String,
}

impl EnvVarStore {
    pub fn new(var_name: String) -> Self {
        Self { var_name }
    }
}

impl CredentialStore for EnvVarStore {
    fn store(&self, _profile_name: &str, _credential: &str) -> Result<()> {
        // Environment variables are read-only from the application's perspective
        Err(CcmError::Other(
            "Cannot store credentials in environment variables. Set the variable externally.".into()
        ))
    }

    fn retrieve(&self, _profile_name: &str) -> Result<String> {
        env::var(&self.var_name).map_err(|_| CcmError::EnvVarNotSet {
            var: self.var_name.clone(),
        })
    }

    fn delete(&self, _profile_name: &str) -> Result<()> {
        Err(CcmError::Other(
            "Cannot delete credentials from environment variables.".into()
        ))
    }

    fn exists(&self, _profile_name: &str) -> Result<bool> {
        Ok(env::var(&self.var_name).is_ok())
    }

    fn backend_name(&self) -> &'static str {
        "Environment Variable"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_var_store() {
        env::set_var("TEST_CCM_API_KEY", "test-key");
        
        let store = EnvVarStore::new("TEST_CCM_API_KEY".into());
        
        assert!(store.exists("any-profile").unwrap());
        assert_eq!(store.retrieve("any-profile").unwrap(), "test-key");
        
        env::remove_var("TEST_CCM_API_KEY");
    }
}
```

#### 2.5 Create `crates/ccm-core/src/credential/mod.rs`

```rust
//! Credential storage module

pub mod encrypted;
pub mod env;
pub mod keychain;
pub mod traits;

pub use encrypted::EncryptedFileStore;
pub use env::EnvVarStore;
pub use keychain::{is_keychain_available, KeychainStore};
pub use traits::CredentialStore;

use crate::error::Result;
use crate::profile::{CredentialSource, Profile};
use std::sync::Arc;

/// Get the appropriate credential store for a profile
pub fn get_store_for_profile(profile: &Profile) -> Result<Arc<dyn CredentialStore>> {
    match &profile.credential_source {
        CredentialSource::Keychain => {
            if is_keychain_available() {
                Ok(Arc::new(KeychainStore::new()))
            } else {
                // Fall back to encrypted file
                tracing::warn!(
                    "System keychain unavailable, falling back to encrypted file storage"
                );
                Ok(Arc::new(EncryptedFileStore::with_machine_id()?))
            }
        }
        CredentialSource::EnvVar { var_name } => {
            Ok(Arc::new(EnvVarStore::new(var_name.clone())))
        }
        CredentialSource::None => {
            // Return a dummy store that always fails
            Ok(Arc::new(NoOpStore))
        }
    }
}

/// Get the default credential store (prefers keychain)
pub fn get_default_store() -> Result<Arc<dyn CredentialStore>> {
    if is_keychain_available() {
        Ok(Arc::new(KeychainStore::new()))
    } else {
        Ok(Arc::new(EncryptedFileStore::with_machine_id()?))
    }
}

/// No-op credential store for profiles that don't need credentials
struct NoOpStore;

impl CredentialStore for NoOpStore {
    fn store(&self, _profile_name: &str, _credential: &str) -> Result<()> {
        Ok(())
    }

    fn retrieve(&self, profile_name: &str) -> Result<String> {
        Err(crate::error::CcmError::CredentialNotFound {
            profile: profile_name.to_string(),
        })
    }

    fn delete(&self, _profile_name: &str) -> Result<()> {
        Ok(())
    }

    fn exists(&self, _profile_name: &str) -> Result<bool> {
        Ok(false)
    }

    fn backend_name(&self) -> &'static str {
        "None"
    }
}
```

---

## Week 3: Configuration Injection & Shell Integration

### Day 11-14: Claude Code Settings Injection

#### 3.1 Create `crates/ccm-core/src/injector/claude.rs`

```rust
//! Claude Code settings.json injection

use crate::credential::{get_store_for_profile, CredentialStore};
use crate::error::{CcmError, Result};
use crate::paths::{backup_dir, claude_settings_path, ensure_dir};
use crate::profile::{CredentialSource, Profile, Provider};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Claude Code settings.json structure (partial)
/// 
/// We only manage the `env` block, preserving all other settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClaudeSettings {
    /// Environment variables block
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub env: HashMap<String, String>,

    /// Preserve all other fields
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

/// Environment variable names for Claude Code configuration
pub mod env_vars {
    pub const ANTHROPIC_API_KEY: &str = "ANTHROPIC_API_KEY";
    pub const ANTHROPIC_BASE_URL: &str = "ANTHROPIC_BASE_URL";
    pub const CLAUDE_MODEL: &str = "CLAUDE_MODEL";
    pub const ANTHROPIC_TIMEOUT: &str = "ANTHROPIC_TIMEOUT";
    
    /// All ccm-managed environment variables
    pub const CCM_MANAGED: &[&str] = &[
        ANTHROPIC_API_KEY,
        ANTHROPIC_BASE_URL,
        CLAUDE_MODEL,
        ANTHROPIC_TIMEOUT,
    ];
}

/// Injects profile configuration into Claude Code settings
pub struct ClaudeCodeInjector;

impl ClaudeCodeInjector {
    /// Apply a profile to Claude Code settings
    pub fn apply(profile: &Profile) -> Result<()> {
        let settings_path = claude_settings_path()?;

        // Load existing settings or create new
        let mut settings = Self::load_settings(&settings_path)?;

        // Create backup before modification
        Self::create_backup(&settings_path)?;

        // Clear previous ccm-managed env vars
        Self::clear_ccm_env_vars(&mut settings);

        // Set new env vars from profile
        Self::set_profile_env_vars(&mut settings, profile)?;

        // Write atomically
        Self::write_settings(&settings_path, &settings)?;

        tracing::info!("Applied profile '{}' to Claude Code settings", profile.name);
        Ok(())
    }

    /// Remove ccm configuration from Claude Code settings
    pub fn clear() -> Result<()> {
        let settings_path = claude_settings_path()?;

        if !settings_path.exists() {
            return Ok(());
        }

        let mut settings = Self::load_settings(&settings_path)?;
        
        // Create backup
        Self::create_backup(&settings_path)?;

        // Clear ccm-managed env vars
        Self::clear_ccm_env_vars(&mut settings);

        // Write back
        Self::write_settings(&settings_path, &settings)?;

        tracing::info!("Cleared ccm configuration from Claude Code settings");
        Ok(())
    }

    /// Load settings from file
    fn load_settings(path: &PathBuf) -> Result<ClaudeSettings> {
        if !path.exists() {
            // If settings.json doesn't exist, check if .claude directory exists
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    return Err(CcmError::ClaudeSettingsNotFound {
                        path: path.clone(),
                    });
                }
            }
            return Ok(ClaudeSettings::default());
        }

        let content = fs::read_to_string(path)?;
        let settings: ClaudeSettings = serde_json::from_str(&content).map_err(|e| {
            CcmError::InvalidClaudeSettings(format!("Failed to parse settings.json: {}", e))
        })?;

        Ok(settings)
    }

    /// Write settings atomically (write to temp, then rename)
    fn write_settings(path: &PathBuf, settings: &ClaudeSettings) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            ensure_dir(&parent.to_path_buf())?;
        }

        // Serialize with pretty formatting
        let content = serde_json::to_string_pretty(settings)?;

        // Write to temp file first
        let temp_path = path.with_extension("json.tmp");
        fs::write(&temp_path, &content)?;

        // Rename atomically
        fs::rename(&temp_path, path).map_err(|e| {
            // Clean up temp file on failure
            let _ = fs::remove_file(&temp_path);
            CcmError::ClaudeSettingsWriteError(format!("Failed to write settings: {}", e))
        })?;

        Ok(())
    }

    /// Create a backup of current settings
    fn create_backup(path: &PathBuf) -> Result<()> {
        if !path.exists() {
            return Ok(());
        }

        let backup_path = backup_dir()?;
        ensure_dir(&backup_path)?;

        // Use timestamp in filename
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let backup_file = backup_path.join(format!("settings.{}.json", timestamp));
        fs::copy(path, &backup_file)?;

        // Keep only last 5 backups
        Self::cleanup_old_backups(&backup_path)?;

        tracing::debug!("Created backup at {:?}", backup_file);
        Ok(())
    }

    /// Remove old backup files, keeping only the most recent N
    fn cleanup_old_backups(backup_path: &PathBuf) -> Result<()> {
        const MAX_BACKUPS: usize = 5;

        let mut backups: Vec<_> = fs::read_dir(backup_path)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with("settings.") && n.ends_with(".json"))
                    .unwrap_or(false)
            })
            .collect();

        if backups.len() <= MAX_BACKUPS {
            return Ok(());
        }

        // Sort by modification time (oldest first)
        backups.sort_by_key(|e| {
            e.metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });

        // Remove oldest backups
        for entry in backups.iter().take(backups.len() - MAX_BACKUPS) {
            let _ = fs::remove_file(entry.path());
        }

        Ok(())
    }

    /// Clear all ccm-managed environment variables
    fn clear_ccm_env_vars(settings: &mut ClaudeSettings) {
        for var in env_vars::CCM_MANAGED {
            settings.env.remove(*var);
        }
    }

    /// Set environment variables from profile
    fn set_profile_env_vars(settings: &mut ClaudeSettings, profile: &Profile) -> Result<()> {
        // Set API key if required
        if profile.provider.requires_api_key() {
            let store = get_store_for_profile(profile)?;
            let api_key = store.retrieve(&profile.name)?;
            settings.env.insert(env_vars::ANTHROPIC_API_KEY.to_string(), api_key);
        }

        // Set base URL
        if let Some(base_url) = profile.effective_base_url() {
            settings.env.insert(env_vars::ANTHROPIC_BASE_URL.to_string(), base_url.to_string());
        }

        // Set model
        settings.env.insert(env_vars::CLAUDE_MODEL.to_string(), profile.model.clone());

        // Set timeout
        let timeout_secs = profile.timeout_ms / 1000;
        settings.env.insert(
            env_vars::ANTHROPIC_TIMEOUT.to_string(),
            timeout_secs.to_string(),
        );

        Ok(())
    }

    /// Get the currently applied profile name from settings (if any)
    pub fn get_applied_profile() -> Result<Option<String>> {
        // We don't store profile name in settings.json
        // This would require reading the 'current' file
        crate::profile::ProfileManager::get_current().map(Some).or(Ok(None))
    }

    /// Restore settings from the most recent backup
    pub fn restore_backup() -> Result<()> {
        let backup_path = backup_dir()?;
        let settings_path = claude_settings_path()?;

        // Find most recent backup
        let mut backups: Vec<_> = fs::read_dir(&backup_path)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with("settings.") && n.ends_with(".json"))
                    .unwrap_or(false)
            })
            .collect();

        if backups.is_empty() {
            return Err(CcmError::Other("No backups available".into()));
        }

        // Sort by modification time (newest first)
        backups.sort_by_key(|e| {
            std::cmp::Reverse(
                e.metadata()
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
            )
        });

        let latest_backup = &backups[0];
        fs::copy(latest_backup.path(), &settings_path)?;

        tracing::info!("Restored settings from backup {:?}", latest_backup.path());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_settings_serialization() {
        let mut settings = ClaudeSettings::default();
        settings.env.insert("TEST_KEY".to_string(), "test_value".to_string());

        let json = serde_json::to_string_pretty(&settings).unwrap();
        let parsed: ClaudeSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.env.get("TEST_KEY"), Some(&"test_value".to_string()));
    }

    #[test]
    fn test_clear_ccm_env_vars() {
        let mut settings = ClaudeSettings::default();
        settings.env.insert(env_vars::ANTHROPIC_API_KEY.to_string(), "key".into());
        settings.env.insert(env_vars::CLAUDE_MODEL.to_string(), "model".into());
        settings.env.insert("CUSTOM_VAR".to_string(), "value".into());

        ClaudeCodeInjector::clear_ccm_env_vars(&mut settings);

        assert!(!settings.env.contains_key(env_vars::ANTHROPIC_API_KEY));
        assert!(!settings.env.contains_key(env_vars::CLAUDE_MODEL));
        assert!(settings.env.contains_key("CUSTOM_VAR")); // Preserved
    }
}
```

#### 3.2 Create `crates/ccm-core/src/injector/mod.rs`

```rust
//! Configuration injection module

pub mod claude;

pub use claude::{env_vars, ClaudeCodeInjector, ClaudeSettings};
```

### Day 15-17: Shell Integration

#### 3.3 Create `crates/ccm-core/src/shell/mod.rs`

```rust
//! Shell integration generators

pub mod bash;
pub mod fish;
pub mod powershell;
pub mod zsh;

use crate::error::{CcmError, Result};

/// Supported shells
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

impl Shell {
    /// Detect the current shell from environment
    pub fn detect() -> Option<Self> {
        // Check SHELL environment variable
        if let Ok(shell) = std::env::var("SHELL") {
            if shell.contains("bash") {
                return Some(Shell::Bash);
            }
            if shell.contains("zsh") {
                return Some(Shell::Zsh);
            }
            if shell.contains("fish") {
                return Some(Shell::Fish);
            }
        }

        // Check for PowerShell
        if std::env::var("PSModulePath").is_ok() {
            return Some(Shell::PowerShell);
        }

        // Check BASH_VERSION or ZSH_VERSION
        if std::env::var("BASH_VERSION").is_ok() {
            return Some(Shell::Bash);
        }
        if std::env::var("ZSH_VERSION").is_ok() {
            return Some(Shell::Zsh);
        }

        None
    }

    /// Get the shell name
    pub fn name(&self) -> &'static str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Fish => "fish",
            Shell::PowerShell => "powershell",
        }
    }

    /// Get the configuration file path
    pub fn config_file(&self) -> &'static str {
        match self {
            Shell::Bash => "~/.bashrc",
            Shell::Zsh => "~/.zshrc",
            Shell::Fish => "~/.config/fish/config.fish",
            Shell::PowerShell => "$PROFILE",
        }
    }
}

impl std::str::FromStr for Shell {
    type Err = CcmError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            "fish" => Ok(Shell::Fish),
            "powershell" | "pwsh" | "ps" => Ok(Shell::PowerShell),
            _ => Err(CcmError::UnsupportedShell { shell: s.into() }),
        }
    }
}

/// Generate shell integration script
pub fn generate_script(shell: Shell, use_on_cd: bool) -> String {
    match shell {
        Shell::Bash => bash::generate(use_on_cd),
        Shell::Zsh => zsh::generate(use_on_cd),
        Shell::Fish => fish::generate(use_on_cd),
        Shell::PowerShell => powershell::generate(use_on_cd),
    }
}

/// Generate completions for a shell
pub fn generate_completions(shell: Shell) -> String {
    match shell {
        Shell::Bash => bash::completions(),
        Shell::Zsh => zsh::completions(),
        Shell::Fish => fish::completions(),
        Shell::PowerShell => powershell::completions(),
    }
}
```

#### 3.4 Create `crates/ccm-core/src/shell/zsh.rs`

```rust
//! Zsh shell integration

/// Generate zsh integration script
pub fn generate(use_on_cd: bool) -> String {
    let mut script = String::from(r#"
# ccm shell integration for zsh
# Add to ~/.zshrc: eval "$(ccm env --use-on-cd)"

__ccm_use() {
    local profile="$1"
    if [[ -n "$profile" ]]; then
        ccm use "$profile" --quiet
    fi
}

# Function to resolve profile from .ccmrc
__ccm_resolve_profile() {
    local dir="$PWD"
    while [[ "$dir" != "/" ]]; do
        if [[ -f "$dir/.ccmrc" ]]; then
            # Extract profile name from .ccmrc (TOML format)
            local profile=$(grep -E '^profile\s*=' "$dir/.ccmrc" 2>/dev/null | sed 's/.*=\s*["'"'"']\?\([^"'"'"']*\)["'"'"']\?/\1/')
            if [[ -n "$profile" ]]; then
                echo "$profile"
                return 0
            fi
        fi
        dir=$(dirname "$dir")
    done
    return 1
}
"#);

    if use_on_cd {
        script.push_str(r#"
# Auto-switch on directory change
__ccm_auto_switch() {
    local profile=$(__ccm_resolve_profile)
    if [[ -n "$profile" ]]; then
        local current=$(ccm current --quiet 2>/dev/null)
        if [[ "$profile" != "$current" ]]; then
            __ccm_use "$profile"
            echo "[ccm] Switched to '$profile'"
        fi
    fi
}

# Hook into chpwd (zsh-specific)
autoload -U add-zsh-hook
add-zsh-hook chpwd __ccm_auto_switch

# Run on shell startup
__ccm_auto_switch
"#);
    }

    script
}

/// Generate zsh completions
pub fn completions() -> String {
    String::from(r#"
#compdef ccm

_ccm() {
    local -a commands
    commands=(
        'add:Create a new profile'
        'remove:Delete a profile'
        'list:List all profiles'
        'use:Switch to a profile'
        'current:Show current profile'
        'show:Show profile details'
        'init:Initialize .ccmrc in current directory'
        'doctor:Run diagnostics'
        'env:Output shell integration script'
    )

    local -a profiles
    if (( CURRENT == 2 )); then
        _describe 'command' commands
    else
        case "$words[2]" in
            use|remove|show)
                profiles=($(ccm list --quiet 2>/dev/null))
                _describe 'profile' profiles
                ;;
        esac
    fi
}

compdef _ccm ccm
"#)
}
```

#### 3.5 Create `crates/ccm-core/src/shell/bash.rs`

```rust
//! Bash shell integration

/// Generate bash integration script
pub fn generate(use_on_cd: bool) -> String {
    let mut script = String::from(r#"
# ccm shell integration for bash
# Add to ~/.bashrc: eval "$(ccm env --use-on-cd)"

__ccm_use() {
    local profile="$1"
    if [[ -n "$profile" ]]; then
        ccm use "$profile" --quiet
    fi
}

# Function to resolve profile from .ccmrc
__ccm_resolve_profile() {
    local dir="$PWD"
    while [[ "$dir" != "/" ]]; do
        if [[ -f "$dir/.ccmrc" ]]; then
            local profile=$(grep -E '^profile\s*=' "$dir/.ccmrc" 2>/dev/null | sed 's/.*=\s*["'"'"']\?\([^"'"'"']*\)["'"'"']\?/\1/')
            if [[ -n "$profile" ]]; then
                echo "$profile"
                return 0
            fi
        fi
        dir=$(dirname "$dir")
    done
    return 1
}
"#);

    if use_on_cd {
        script.push_str(r#"
# Auto-switch on directory change
__ccm_auto_switch() {
    local profile=$(__ccm_resolve_profile)
    if [[ -n "$profile" ]]; then
        local current=$(ccm current --quiet 2>/dev/null)
        if [[ "$profile" != "$current" ]]; then
            __ccm_use "$profile"
            echo "[ccm] Switched to '$profile'"
        fi
    fi
}

# Override cd to trigger auto-switch
__ccm_cd() {
    builtin cd "$@" && __ccm_auto_switch
}

alias cd=__ccm_cd

# Run on shell startup
__ccm_auto_switch
"#);
    }

    script
}

/// Generate bash completions
pub fn completions() -> String {
    String::from(r#"
# bash completion for ccm

_ccm_completions() {
    local cur prev commands profiles
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    commands="add remove list use current show init doctor env"

    case "$prev" in
        ccm)
            COMPREPLY=($(compgen -W "$commands" -- "$cur"))
            ;;
        use|remove|show)
            profiles=$(ccm list --quiet 2>/dev/null)
            COMPREPLY=($(compgen -W "$profiles" -- "$cur"))
            ;;
    esac
}

complete -F _ccm_completions ccm
"#)
}
```

#### 3.6 Create `crates/ccm-core/src/shell/fish.rs`

```rust
//! Fish shell integration

/// Generate fish integration script
pub fn generate(use_on_cd: bool) -> String {
    let mut script = String::from(r#"
# ccm shell integration for fish
# Add to ~/.config/fish/config.fish: ccm env --use-on-cd | source

function __ccm_use
    set -l profile $argv[1]
    if test -n "$profile"
        ccm use "$profile" --quiet
    end
end

function __ccm_resolve_profile
    set -l dir $PWD
    while test "$dir" != "/"
        if test -f "$dir/.ccmrc"
            set -l profile (grep -E '^profile\s*=' "$dir/.ccmrc" 2>/dev/null | sed 's/.*=\s*["'"'"']\?\([^"'"'"']*\)["'"'"']\?/\1/')
            if test -n "$profile"
                echo $profile
                return 0
            end
        end
        set dir (dirname "$dir")
    end
    return 1
end
"#);

    if use_on_cd {
        script.push_str(r#"
function __ccm_auto_switch --on-variable PWD
    set -l profile (__ccm_resolve_profile)
    if test -n "$profile"
        set -l current (ccm current --quiet 2>/dev/null)
        if test "$profile" != "$current"
            __ccm_use "$profile"
            echo "[ccm] Switched to '$profile'"
        end
    end
end

# Run on shell startup
__ccm_auto_switch
"#);
    }

    script
}

/// Generate fish completions
pub fn completions() -> String {
    String::from(r#"
# fish completion for ccm

complete -c ccm -f

# Commands
complete -c ccm -n "__fish_use_subcommand" -a add -d "Create a new profile"
complete -c ccm -n "__fish_use_subcommand" -a remove -d "Delete a profile"
complete -c ccm -n "__fish_use_subcommand" -a list -d "List all profiles"
complete -c ccm -n "__fish_use_subcommand" -a use -d "Switch to a profile"
complete -c ccm -n "__fish_use_subcommand" -a current -d "Show current profile"
complete -c ccm -n "__fish_use_subcommand" -a show -d "Show profile details"
complete -c ccm -n "__fish_use_subcommand" -a init -d "Initialize .ccmrc"
complete -c ccm -n "__fish_use_subcommand" -a doctor -d "Run diagnostics"
complete -c ccm -n "__fish_use_subcommand" -a env -d "Output shell integration"

# Profile completions for relevant commands
function __fish_ccm_profiles
    ccm list --quiet 2>/dev/null
end

complete -c ccm -n "__fish_seen_subcommand_from use remove show" -a "(__fish_ccm_profiles)"
"#)
}
```

#### 3.7 Create `crates/ccm-core/src/shell/powershell.rs`

```rust
//! PowerShell integration

/// Generate PowerShell integration script
pub fn generate(use_on_cd: bool) -> String {
    let mut script = String::from(r#"
# ccm shell integration for PowerShell
# Add to $PROFILE: Invoke-Expression (& ccm env --use-on-cd)

function __ccm_use {
    param([string]$profile)
    if ($profile) {
        & ccm use $profile --quiet
    }
}

function __ccm_resolve_profile {
    $dir = Get-Location
    while ($dir.Path -ne [System.IO.Path]::GetPathRoot($dir.Path)) {
        $ccmrc = Join-Path $dir.Path ".ccmrc"
        if (Test-Path $ccmrc) {
            $content = Get-Content $ccmrc -Raw
            if ($content -match 'profile\s*=\s*[''"]?([^''"]+)[''"]?') {
                return $Matches[1]
            }
        }
        $dir = Split-Path $dir.Path -Parent
        if (-not $dir) { break }
        $dir = Get-Item $dir
    }
    return $null
}
"#);

    if use_on_cd {
        script.push_str(r#"
function __ccm_auto_switch {
    $profile = __ccm_resolve_profile
    if ($profile) {
        $current = & ccm current --quiet 2>$null
        if ($profile -ne $current) {
            __ccm_use $profile
            Write-Host "[ccm] Switched to '$profile'"
        }
    }
}

# Override Set-Location (cd)
$ExecutionContext.InvokeCommand.PostCommandLookupAction = {
    param($CommandName, $CommandLookupEventArgs)
    if ($CommandName -eq 'Set-Location' -or $CommandName -eq 'cd') {
        $CommandLookupEventArgs.CommandScriptBlock = {
            param($Path)
            Microsoft.PowerShell.Management\Set-Location @PSBoundParameters
            __ccm_auto_switch
        }.GetNewClosure()
    }
}

# Run on shell startup
__ccm_auto_switch
"#);
    }

    script
}

/// Generate PowerShell completions
pub fn completions() -> String {
    String::from(r#"
# PowerShell completion for ccm

Register-ArgumentCompleter -Native -CommandName ccm -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    
    $commands = @('add', 'remove', 'list', 'use', 'current', 'show', 'init', 'doctor', 'env')
    $elements = $commandAst.CommandElements
    
    if ($elements.Count -eq 1) {
        $commands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
            [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
        }
    }
    elseif ($elements.Count -eq 2) {
        $subCommand = $elements[1].Value
        if ($subCommand -in @('use', 'remove', 'show')) {
            $profiles = & ccm list --quiet 2>$null
            $profiles | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
                [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
            }
        }
    }
}
"#)
}
```

---

## Week 4: Project Config & Doctor

### Day 18-21: Project Configuration (.ccmrc)

#### 4.1 Create `crates/ccm-core/src/project/ccmrc.rs`

```rust
//! .ccmrc file parser

use crate::error::{CcmError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// .ccmrc file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ccmrc {
    /// Profile name to use
    pub profile: String,

    /// Optional overrides (applied on top of profile)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#override: Option<CcmrcOverride>,
}

/// Optional overrides in .ccmrc
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CcmrcOverride {
    /// Override model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Override base URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,

    /// Override timeout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

impl Ccmrc {
    /// Parse a .ccmrc file from string content
    pub fn parse(content: &str) -> Result<Self> {
        toml::from_str(content).map_err(|e| CcmError::InvalidCcmrc {
            path: PathBuf::from(".ccmrc"),
            reason: e.to_string(),
        })
    }

    /// Parse a .ccmrc file from a path
    pub fn from_path(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content).map_err(|e| CcmError::InvalidCcmrc {
            path: path.clone(),
            reason: e.to_string(),
        })
    }

    /// Serialize to TOML string
    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self).map_err(|e| e.into())
    }

    /// Create a minimal .ccmrc with just a profile name
    pub fn minimal(profile: String) -> Self {
        Self {
            profile,
            r#override: None,
        }
    }

    /// Check if there are any overrides
    pub fn has_overrides(&self) -> bool {
        self.r#override.as_ref().map_or(false, |o| {
            o.model.is_some() || o.base_url.is_some() || o.timeout_ms.is_some()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_ccmrc() {
        let content = r#"
profile = "anthropic"
"#;
        let ccmrc = Ccmrc::parse(content).unwrap();
        assert_eq!(ccmrc.profile, "anthropic");
        assert!(ccmrc.r#override.is_none());
    }

    #[test]
    fn test_ccmrc_with_overrides() {
        let content = r#"
profile = "anthropic"

[override]
model = "claude-opus-4-5-20251101"
timeout_ms = 120000
"#;
        let ccmrc = Ccmrc::parse(content).unwrap();
        assert_eq!(ccmrc.profile, "anthropic");
        
        let overrides = ccmrc.r#override.unwrap();
        assert_eq!(overrides.model, Some("claude-opus-4-5-20251101".to_string()));
        assert_eq!(overrides.timeout_ms, Some(120000));
    }

    #[test]
    fn test_ccmrc_serialization() {
        let ccmrc = Ccmrc {
            profile: "local".to_string(),
            r#override: Some(CcmrcOverride {
                model: Some("qwen2.5-coder:32b".to_string()),
                base_url: None,
                timeout_ms: Some(180000),
            }),
        };

        let toml = ccmrc.to_toml().unwrap();
        assert!(toml.contains("profile = \"local\""));
        assert!(toml.contains("[override]"));
    }
}
```

#### 4.2 Create `crates/ccm-core/src/project/resolver.rs`

```rust
//! Project configuration resolver

use super::ccmrc::Ccmrc;
use crate::error::Result;
use crate::paths::CCMRC_FILENAME;
use crate::profile::{Profile, ProfileManager};
use std::path::{Path, PathBuf};

/// Result of resolving project configuration
#[derive(Debug)]
pub struct ResolvedConfig {
    /// The .ccmrc file path (if found)
    pub ccmrc_path: Option<PathBuf>,
    
    /// The parsed .ccmrc content (if found)
    pub ccmrc: Option<Ccmrc>,
    
    /// The resolved profile (with overrides applied)
    pub profile: Profile,
}

/// Resolves project configuration by searching for .ccmrc files
pub struct ProjectResolver;

impl ProjectResolver {
    /// Find .ccmrc file starting from the given directory, searching up
    pub fn find_ccmrc(start_dir: &Path) -> Option<PathBuf> {
        let mut current = start_dir.to_path_buf();

        loop {
            let ccmrc_path = current.join(CCMRC_FILENAME);
            if ccmrc_path.exists() {
                return Some(ccmrc_path);
            }

            if !current.pop() {
                break;
            }
        }

        None
    }

    /// Find .ccmrc file starting from the current directory
    pub fn find_ccmrc_from_cwd() -> Option<PathBuf> {
        let cwd = std::env::current_dir().ok()?;
        Self::find_ccmrc(&cwd)
    }

    /// Resolve configuration for the current directory
    pub fn resolve() -> Result<ResolvedConfig> {
        Self::resolve_from(&std::env::current_dir()?)
    }

    /// Resolve configuration starting from a specific directory
    pub fn resolve_from(start_dir: &Path) -> Result<ResolvedConfig> {
        // Try to find .ccmrc
        let ccmrc_path = Self::find_ccmrc(start_dir);

        if let Some(ref path) = ccmrc_path {
            // Parse .ccmrc
            let ccmrc = Ccmrc::from_path(path)?;

            // Load the base profile
            let mut profile = ProfileManager::read(&ccmrc.profile)?;

            // Apply overrides
            if let Some(ref overrides) = ccmrc.r#override {
                if let Some(ref model) = overrides.model {
                    profile.model = model.clone();
                }
                if let Some(ref base_url) = overrides.base_url {
                    profile.base_url = Some(base_url.clone());
                }
                if let Some(timeout) = overrides.timeout_ms {
                    profile.timeout_ms = timeout;
                }
            }

            Ok(ResolvedConfig {
                ccmrc_path,
                ccmrc: Some(ccmrc),
                profile,
            })
        } else {
            // No .ccmrc found, use current/default profile
            let profile_name = ProfileManager::get_current()?;
            let profile = ProfileManager::read(&profile_name)?;

            Ok(ResolvedConfig {
                ccmrc_path: None,
                ccmrc: None,
                profile,
            })
        }
    }

    /// Check if a .ccmrc exists in the current directory (not parents)
    pub fn has_local_ccmrc() -> bool {
        std::env::current_dir()
            .map(|cwd| cwd.join(CCMRC_FILENAME).exists())
            .unwrap_or(false)
    }

    /// Initialize a .ccmrc file in the specified directory
    pub fn init(dir: &Path, profile: &str, overrides: Option<Ccmrc>) -> Result<PathBuf> {
        let ccmrc_path = dir.join(CCMRC_FILENAME);

        // Check if already exists
        if ccmrc_path.exists() {
            return Err(crate::error::CcmError::Other(format!(
                ".ccmrc already exists at {:?}",
                ccmrc_path
            )));
        }

        // Verify profile exists
        if !ProfileManager::exists(profile)? {
            return Err(crate::error::CcmError::ProfileNotFound {
                name: profile.to_string(),
            });
        }

        // Create .ccmrc
        let ccmrc = overrides.unwrap_or_else(|| Ccmrc::minimal(profile.to_string()));
        let content = ccmrc.to_toml()?;
        std::fs::write(&ccmrc_path, content)?;

        tracing::info!("Created .ccmrc at {:?}", ccmrc_path);
        Ok(ccmrc_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_find_ccmrc() {
        let temp = tempdir().unwrap();
        let sub_dir = temp.path().join("a/b/c");
        std::fs::create_dir_all(&sub_dir).unwrap();

        // Create .ccmrc in 'a' directory
        let ccmrc_path = temp.path().join("a/.ccmrc");
        std::fs::write(&ccmrc_path, "profile = \"test\"").unwrap();

        // Should find it from 'a/b/c'
        let found = ProjectResolver::find_ccmrc(&sub_dir);
        assert!(found.is_some());
        assert_eq!(found.unwrap(), ccmrc_path);
    }

    #[test]
    fn test_no_ccmrc() {
        let temp = tempdir().unwrap();
        let found = ProjectResolver::find_ccmrc(temp.path());
        assert!(found.is_none());
    }
}
```

#### 4.3 Create `crates/ccm-core/src/project/mod.rs`

```rust
//! Project configuration module

pub mod ccmrc;
pub mod resolver;

pub use ccmrc::{Ccmrc, CcmrcOverride};
pub use resolver::{ProjectResolver, ResolvedConfig};
```

### Day 22-24: Doctor Command

#### 4.4 Create `crates/ccm-core/src/doctor/checks.rs`

```rust
//! Diagnostic check implementations

use crate::credential::is_keychain_available;
use crate::error::Result;
use crate::paths::{claude_config_dir, claude_settings_path, config_dir, profiles_dir};
use crate::profile::ProfileManager;

/// Result of a diagnostic check
#[derive(Debug)]
pub struct CheckResult {
    /// Check name
    pub name: &'static str,
    /// Whether the check passed
    pub passed: bool,
    /// Status message
    pub message: String,
    /// Suggestion if failed
    pub suggestion: Option<String>,
}

impl CheckResult {
    pub fn pass(name: &'static str, message: impl Into<String>) -> Self {
        Self {
            name,
            passed: true,
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn fail(
        name: &'static str,
        message: impl Into<String>,
        suggestion: impl Into<String>,
    ) -> Self {
        Self {
            name,
            passed: false,
            message: message.into(),
            suggestion: Some(suggestion.into()),
        }
    }

    pub fn warn(name: &'static str, message: impl Into<String>) -> Self {
        Self {
            name,
            passed: true, // Warnings don't fail
            message: message.into(),
            suggestion: None,
        }
    }
}

/// Check if ccm configuration directory exists
pub fn check_ccm_installation() -> CheckResult {
    match config_dir() {
        Ok(path) => {
            if path.exists() {
                CheckResult::pass("ccm installation", format!("Config directory: {:?}", path))
            } else {
                CheckResult::fail(
                    "ccm installation",
                    "Config directory not found",
                    "Run 'ccm add <profile>' to create your first profile",
                )
            }
        }
        Err(e) => CheckResult::fail(
            "ccm installation",
            format!("Could not determine config path: {}", e),
            "Ensure HOME or XDG_CONFIG_HOME is set",
        ),
    }
}

/// Check if Claude Code CLI is installed
pub fn check_claude_code_cli() -> CheckResult {
    match which::which("claude") {
        Ok(path) => CheckResult::pass(
            "Claude Code CLI",
            format!("Found at: {}", path.display()),
        ),
        Err(_) => CheckResult::fail(
            "Claude Code CLI",
            "Claude Code CLI not found in PATH",
            "Install Claude Code: https://docs.anthropic.com/claude-code",
        ),
    }
}

/// Check if Claude Code config directory exists
pub fn check_claude_config() -> CheckResult {
    match claude_config_dir() {
        Ok(path) => {
            if path.exists() {
                CheckResult::pass("Claude Code config", format!("Config directory: {:?}", path))
            } else {
                CheckResult::fail(
                    "Claude Code config",
                    "~/.claude directory not found",
                    "Run 'claude' once to initialize the config directory",
                )
            }
        }
        Err(e) => CheckResult::fail(
            "Claude Code config",
            format!("Could not determine config path: {}", e),
            "Ensure HOME is set",
        ),
    }
}

/// Check if any profiles exist
pub fn check_profiles() -> CheckResult {
    match ProfileManager::list() {
        Ok(profiles) => {
            if profiles.is_empty() {
                CheckResult::fail(
                    "Profiles",
                    "No profiles configured",
                    "Create a profile with 'ccm add <name>'",
                )
            } else {
                CheckResult::pass("Profiles", format!("{} profile(s) found", profiles.len()))
            }
        }
        Err(e) => CheckResult::fail(
            "Profiles",
            format!("Could not list profiles: {}", e),
            "Check file permissions in config directory",
        ),
    }
}

/// Check if a default profile is set
pub fn check_default_profile() -> CheckResult {
    match ProfileManager::get_default() {
        Ok(name) => {
            // Verify the profile exists
            if ProfileManager::exists(&name).unwrap_or(false) {
                CheckResult::pass("Default profile", format!("Set to '{}'", name))
            } else {
                CheckResult::fail(
                    "Default profile",
                    format!("Default profile '{}' does not exist", name),
                    format!("Run 'ccm add {}' or 'ccm use <other-profile>'", name),
                )
            }
        }
        Err(_) => CheckResult::fail(
            "Default profile",
            "No default profile set",
            "Run 'ccm use <profile>' to set a default",
        ),
    }
}

/// Check if system keychain is available
pub fn check_keychain() -> CheckResult {
    if is_keychain_available() {
        #[cfg(target_os = "macos")]
        let backend = "macOS Keychain";
        #[cfg(target_os = "linux")]
        let backend = "Secret Service (libsecret)";
        #[cfg(target_os = "windows")]
        let backend = "Windows Credential Manager";
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        let backend = "System Keychain";

        CheckResult::pass("Credential storage", format!("Using {}", backend))
    } else {
        CheckResult::warn(
            "Credential storage",
            "System keychain unavailable, using encrypted file fallback",
        )
    }
}

/// Check if shell integration is set up
pub fn check_shell_integration() -> CheckResult {
    // Check common shell config files for ccm integration
    let home = match home::home_dir() {
        Some(h) => h,
        None => {
            return CheckResult::fail(
                "Shell integration",
                "Could not determine home directory",
                "Ensure HOME is set",
            )
        }
    };

    let shell_configs = [
        (home.join(".zshrc"), "zsh"),
        (home.join(".bashrc"), "bash"),
        (home.join(".config/fish/config.fish"), "fish"),
    ];

    for (config_path, shell_name) in shell_configs {
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if content.contains("ccm env") {
                    return CheckResult::pass(
                        "Shell integration",
                        format!("Found in {} config", shell_name),
                    );
                }
            }
        }
    }

    CheckResult::fail(
        "Shell integration",
        "ccm shell integration not detected",
        "Add 'eval \"$(ccm env --use-on-cd)\"' to your shell config",
    )
}

/// Check credentials for a specific profile
pub fn check_profile_credentials(profile_name: &str) -> CheckResult {
    use crate::credential::get_store_for_profile;
    use crate::profile::ProfileManager;

    let profile = match ProfileManager::read(profile_name) {
        Ok(p) => p,
        Err(e) => {
            return CheckResult::fail(
                "Profile credentials",
                format!("Could not read profile '{}': {}", profile_name, e),
                format!("Run 'ccm add {}' to recreate the profile", profile_name),
            )
        }
    };

    if !profile.provider.requires_api_key() {
        return CheckResult::pass(
            "Profile credentials",
            format!("'{}' does not require credentials", profile_name),
        );
    }

    match get_store_for_profile(&profile) {
        Ok(store) => match store.exists(profile_name) {
            Ok(true) => {
                CheckResult::pass("Profile credentials", format!("'{}' has credentials", profile_name))
            }
            Ok(false) => CheckResult::fail(
                "Profile credentials",
                format!("No credentials found for '{}'", profile_name),
                format!("Run 'ccm add {}' to set credentials", profile_name),
            ),
            Err(e) => CheckResult::fail(
                "Profile credentials",
                format!("Could not check credentials: {}", e),
                "Check credential storage access",
            ),
        },
        Err(e) => CheckResult::fail(
            "Profile credentials",
            format!("Could not access credential store: {}", e),
            "Check system keychain or file permissions",
        ),
    }
}

/// Run all basic checks
pub fn run_all_checks() -> Vec<CheckResult> {
    vec![
        check_ccm_installation(),
        check_claude_code_cli(),
        check_claude_config(),
        check_profiles(),
        check_default_profile(),
        check_keychain(),
        check_shell_integration(),
    ]
}
```

#### 4.5 Create `crates/ccm-core/src/doctor/report.rs`

```rust
//! Doctor report formatting

use super::checks::CheckResult;

/// Format check results for terminal output
pub fn format_report(results: &[CheckResult]) -> String {
    let mut output = String::new();
    output.push_str("\nccm Doctor\n");
    output.push_str("──────────\n\n");

    let passed = results.iter().filter(|r| r.passed).count();
    let total = results.len();

    for result in results {
        let icon = if result.passed { "✓" } else { "✗" };
        let status_color = if result.passed { "green" } else { "red" };

        output.push_str(&format!("{} {}\n", icon, result.name));
        output.push_str(&format!("  {}\n", result.message));

        if let Some(ref suggestion) = result.suggestion {
            output.push_str(&format!("  → {}\n", suggestion));
        }
        output.push('\n');
    }

    output.push_str("──────────\n");
    output.push_str(&format!("Result: {}/{} checks passed\n", passed, total));

    if passed == total {
        output.push_str("\n✓ All checks passed!\n");
    } else {
        output.push_str("\n⚠ Some checks failed. See suggestions above.\n");
    }

    output
}

/// Check if all results passed
pub fn all_passed(results: &[CheckResult]) -> bool {
    results.iter().all(|r| r.passed)
}
```

#### 4.6 Create `crates/ccm-core/src/doctor/mod.rs`

```rust
//! Diagnostic checks module

pub mod checks;
pub mod report;

pub use checks::{run_all_checks, CheckResult};
pub use report::{all_passed, format_report};
```

---

## Week 5: Polish, Testing & Distribution

### Day 25-28: CLI Implementation

#### 5.1 Create `crates/ccm/src/main.rs`

```rust
//! ccm - Claude Code Manager CLI

use clap::Parser;
use color_eyre::eyre::Result;
use tracing_subscriber::EnvFilter;

mod cli;
mod commands;

use cli::Cli;

fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("ccm=info".parse()?)
                .add_directive("ccm_core=info".parse()?),
        )
        .with_target(false)
        .without_time()
        .init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Execute command
    cli.execute()
}
```

#### 5.2 Create `crates/ccm/src/cli.rs`

```rust
//! CLI argument definitions

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

use crate::commands;

/// ccm - Claude Code Manager
/// 
/// Fast, secure configuration management for Claude Code CLI.
/// Like fnm/nvm, but for Claude Code profiles.
#[derive(Parser)]
#[command(name = "ccm")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new profile
    Add(commands::AddArgs),

    /// Delete a profile
    Remove(commands::RemoveArgs),

    /// List all profiles
    List(commands::ListArgs),

    /// Switch to a profile
    Use(commands::UseArgs),

    /// Show the current active profile
    Current(commands::CurrentArgs),

    /// Show details of a profile
    Show(commands::ShowArgs),

    /// Initialize a .ccmrc file in the current directory
    Init(commands::InitArgs),

    /// Run diagnostic checks
    Doctor(commands::DoctorArgs),

    /// Output shell integration script
    Env(commands::EnvArgs),
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        match self.command {
            Commands::Add(args) => commands::add::execute(args),
            Commands::Remove(args) => commands::remove::execute(args),
            Commands::List(args) => commands::list::execute(args),
            Commands::Use(args) => commands::use_cmd::execute(args),
            Commands::Current(args) => commands::current::execute(args),
            Commands::Show(args) => commands::show::execute(args),
            Commands::Init(args) => commands::init::execute(args),
            Commands::Doctor(args) => commands::doctor::execute(args),
            Commands::Env(args) => commands::env::execute(args),
        }
    }
}
```

#### 5.3 Create `crates/ccm/src/commands/mod.rs`

```rust
//! CLI command implementations

pub mod add;
pub mod current;
pub mod doctor;
pub mod env;
pub mod init;
pub mod list;
pub mod remove;
pub mod show;
pub mod use_cmd;

pub use add::AddArgs;
pub use current::CurrentArgs;
pub use doctor::DoctorArgs;
pub use env::EnvArgs;
pub use init::InitArgs;
pub use list::ListArgs;
pub use remove::RemoveArgs;
pub use show::ShowArgs;
pub use use_cmd::UseArgs;
```

#### 5.4 Create `crates/ccm/src/commands/add.rs`

```rust
//! Add profile command

use clap::Args;
use color_eyre::eyre::{eyre, Result};
use console::style;
use dialoguer::{Input, Password, Select};

use ccm_core::credential::get_default_store;
use ccm_core::profile::{CredentialSource, Profile, ProfileManager, Provider};

#[derive(Args)]
pub struct AddArgs {
    /// Profile name
    #[arg()]
    pub name: Option<String>,

    /// Provider (anthropic, openrouter, ollama, etc.)
    #[arg(long)]
    pub provider: Option<String>,

    /// Model name
    #[arg(long)]
    pub model: Option<String>,

    /// Base URL for API
    #[arg(long)]
    pub base_url: Option<String>,

    /// API token (not recommended, use interactive mode)
    #[arg(long, env, hide_env = true)]
    pub auth_token: Option<String>,

    /// Read API token from environment variable
    #[arg(long)]
    pub auth_token_env: Option<String>,

    /// Skip interactive prompts
    #[arg(long)]
    pub non_interactive: bool,

    /// Set as default profile
    #[arg(long)]
    pub default: bool,
}

pub fn execute(args: AddArgs) -> Result<()> {
    let (name, provider, model, base_url, credential_source, api_key) = if args.non_interactive {
        // Non-interactive: require name and provider
        let name = args.name.ok_or_else(|| eyre!("Profile name required in non-interactive mode"))?;
        let provider: Provider = args
            .provider
            .ok_or_else(|| eyre!("Provider required in non-interactive mode"))?
            .parse()
            .map_err(|e: String| eyre!(e))?;

        let model = args.model.unwrap_or_else(|| provider.default_model().to_string());
        let base_url = args.base_url.or_else(|| provider.default_base_url().map(String::from));

        let (credential_source, api_key) = if let Some(env_var) = args.auth_token_env {
            (CredentialSource::EnvVar { var_name: env_var }, None)
        } else if provider.requires_api_key() {
            let key = args.auth_token.ok_or_else(|| {
                eyre!("API token required for {}. Use --auth-token or --auth-token-env", provider)
            })?;
            (CredentialSource::Keychain, Some(key))
        } else {
            (CredentialSource::None, None)
        };

        (name, provider, model, base_url, credential_source, api_key)
    } else {
        // Interactive mode
        interactive_add(args)?
    };

    // Create profile
    let profile = Profile {
        name: name.clone(),
        provider,
        model,
        base_url,
        credential_source,
        timeout_ms: 60_000,
        description: None,
        created_at: None,
        updated_at: None,
    };

    // Validate and create
    ProfileManager::create(&profile)?;

    // Store credential if provided
    if let Some(key) = api_key {
        let store = get_default_store()?;
        store.store(&name, &key)?;
    }

    // Set as default if requested or if first profile
    if args.default || ProfileManager::list()?.len() == 1 {
        ProfileManager::set_default(&name)?;
    }

    println!("{} Profile '{}' created.", style("✓").green(), style(&name).cyan());

    if args.default {
        println!("{} Set as default profile.", style("✓").green());
    }

    Ok(())
}

fn interactive_add(args: AddArgs) -> Result<(String, Provider, String, Option<String>, CredentialSource, Option<String>)> {
    println!("{}", style("Create a new ccm profile").bold());
    println!();

    // Profile name
    let name: String = if let Some(n) = args.name {
        n
    } else {
        Input::new()
            .with_prompt("Profile name")
            .interact_text()?
    };

    // Provider selection
    let providers = ["anthropic", "openrouter", "ollama", "bedrock", "vertex-ai", "custom"];
    let provider_idx = if let Some(ref p) = args.provider {
        providers.iter().position(|&x| x == p).unwrap_or(0)
    } else {
        Select::new()
            .with_prompt("Select provider")
            .items(&providers)
            .default(0)
            .interact()?
    };
    let provider: Provider = providers[provider_idx].parse().unwrap();

    // Model
    let default_model = provider.default_model();
    let model: String = if let Some(m) = args.model {
        m
    } else {
        Input::new()
            .with_prompt("Model")
            .default(default_model.to_string())
            .interact_text()?
    };

    // Base URL
    let base_url = if let Some(url) = args.base_url {
        Some(url)
    } else if provider == Provider::Custom {
        Some(
            Input::new()
                .with_prompt("Base URL")
                .interact_text()?,
        )
    } else {
        provider.default_base_url().map(String::from)
    };

    // Credentials
    let (credential_source, api_key) = if let Some(env_var) = args.auth_token_env {
        (CredentialSource::EnvVar { var_name: env_var }, None)
    } else if provider.requires_api_key() {
        let key: String = if let Some(k) = args.auth_token {
            k
        } else {
            Password::new()
                .with_prompt("API Key")
                .interact()?
        };
        (CredentialSource::Keychain, Some(key))
    } else {
        (CredentialSource::None, None)
    };

    Ok((name, provider, model, base_url, credential_source, api_key))
}
```

#### 5.5 Create `crates/ccm/src/commands/use_cmd.rs`

```rust
//! Use (switch) profile command

use clap::Args;
use color_eyre::eyre::Result;
use console::style;

use ccm_core::injector::ClaudeCodeInjector;
use ccm_core::profile::ProfileManager;

#[derive(Args)]
pub struct UseArgs {
    /// Profile name to switch to
    #[arg()]
    pub name: String,

    /// Don't output confirmation message
    #[arg(long)]
    pub quiet: bool,
}

pub fn execute(args: UseArgs) -> Result<()> {
    // Load the profile
    let profile = ProfileManager::read(&args.name)?;

    // Set as current
    ProfileManager::set_current(&args.name)?;

    // Also set as default for persistence
    ProfileManager::set_default(&args.name)?;

    // Inject into Claude Code settings
    ClaudeCodeInjector::apply(&profile)?;

    if !args.quiet {
        println!(
            "{} Switched to '{}' profile.",
            style("✓").green(),
            style(&args.name).cyan()
        );
    }

    Ok(())
}
```

#### 5.6 Create remaining command files

Create similar implementations for:
- `commands/remove.rs`
- `commands/list.rs`
- `commands/current.rs`
- `commands/show.rs`
- `commands/init.rs`
- `commands/doctor.rs`
- `commands/env.rs`

### Day 29-31: Testing & Documentation

#### 5.7 Create `tests/integration/profile_lifecycle.rs`

```rust
//! Integration tests for profile lifecycle

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn ccm() -> Command {
    Command::cargo_bin("ccm").unwrap()
}

#[test]
fn test_add_list_remove_cycle() {
    // This test requires a clean environment
    // In practice, you'd mock the config directory
}

#[test]
fn test_help_output() {
    ccm()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Claude Code Manager"));
}

#[test]
fn test_version_output() {
    ccm()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}
```

### Day 32-35: Distribution & Polish

#### 5.8 Create `scripts/install.sh`

```bash
#!/bin/bash
set -e

# ccm installer script

VERSION="${CCM_VERSION:-latest}"
INSTALL_DIR="${CCM_INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)  OS="linux" ;;
    Darwin) OS="darwin" ;;
    *)      echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
    x86_64)  ARCH="x86_64" ;;
    aarch64) ARCH="aarch64" ;;
    arm64)   ARCH="aarch64" ;;
    *)       echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Download URL
if [ "$VERSION" = "latest" ]; then
    DOWNLOAD_URL="https://github.com/yourusername/ccm/releases/latest/download/ccm-${OS}-${ARCH}"
else
    DOWNLOAD_URL="https://github.com/yourusername/ccm/releases/download/v${VERSION}/ccm-${OS}-${ARCH}"
fi

echo "Installing ccm..."
echo "  Version: $VERSION"
echo "  Target: $OS-$ARCH"
echo "  Install directory: $INSTALL_DIR"
echo

# Create install directory
mkdir -p "$INSTALL_DIR"

# Download binary
curl -fsSL "$DOWNLOAD_URL" -o "$INSTALL_DIR/ccm"
chmod +x "$INSTALL_DIR/ccm"

echo
echo "✓ ccm installed successfully!"
echo
echo "To complete setup, add this to your shell config:"
echo
echo '  eval "$(ccm env --use-on-cd)"'
echo
echo "Then restart your shell or run: source ~/.bashrc (or ~/.zshrc)"
```

#### 5.9 Create `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --all-features

  lint:
    name: Lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets --all-features -- -D warnings

  security:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
```

---

## Success Metrics

### Week 1-2
- [ ] Project compiles
- [ ] Profile CRUD works
- [ ] Credential storage works (at least one backend)

### Week 3-4
- [ ] Claude Code injection works
- [ ] Shell integration works for zsh/bash
- [ ] .ccmrc parsing works
- [ ] Doctor command works

### Week 5
- [ ] All CLI commands implemented
- [ ] Tests pass on Linux, macOS, Windows
- [ ] Documentation complete
- [ ] Binary releases working

---

## Tips for Implementation

1. **Start Simple**: Get profile create/list/delete working first
2. **Test on Multiple Platforms**: Use GitHub Actions for cross-platform CI
3. **Security First**: Never log credentials, use secure storage from day 1
4. **User Experience**: Clear error messages with suggestions
5. **Documentation**: Write docs as you implement

---

## Next Steps After MVP

See [ROADMAP.md](./ROADMAP.md) for post-MVP features:
- Profile templates/presets
- Connection testing
- Profile export/import
- MCP server management
- VS Code extension