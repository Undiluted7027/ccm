# ccm - MVP Roadmap

> **Goal**: Ship a working, production-ready MVP in 5 weeks  
> **Target**: Cross-platform Claude Code configuration manager with secure credential storage

---

## 🎯 MVP Vision

Build a functional configuration manager that demonstrates:
1. ✅ **Profile Management**: Create, switch, and manage multiple Claude Code configurations
2. ✅ **Secure Credentials**: System keychain integration with encrypted fallback
3. ✅ **Shell Integration**: Auto-switching profiles on directory change
4. ✅ **Project Configuration**: Per-project `.ccmrc` files with overrides
5. ✅ **Production Ready**: Cross-platform support, error handling, diagnostics

**MVP Success Metric**: A developer can install ccm, run `ccm add work`, configure their API key, and seamlessly switch between work and personal Claude Code configurations based on which directory they're in.

---

## 📊 Progress Tracker

| Phase | Status | Completion |
|-------|--------|------------|
| **Phase 0: Foundation** | ✅ Completed | 100% |
| **Phase 1: Core Types & Profile Management** | 🏗️ In Progress | 0% |
| **Phase 2: Credential Management** | 🔜 Not Started | 0% |
| **Phase 3: Configuration Injection & Shell** | 🔜 Not Started | 0% |
| **Phase 4: Project Config & Doctor** | 🔜 Not Started | 0% |
| **Phase 5: CLI, Testing & Distribution** | 🔜 Not Started | 0% |

**Overall Progress**: 16.67% (1/6 phases complete)

---

## Phase 0: Foundation

**Duration**: Day 0 (Setup)  
**Status**: ✅ Completed

### Goals
- Set up repository structure
- Configure Cargo workspace
- Establish development environment
- Create initial documentation

### Tasks

#### Repository Setup
- [x] Initialize Git repository
- [x] Create `.gitignore` for Rust projects
- [x] Set up branch protection rules
- [x] Configure GitHub Actions workflows
- [x] Create issue and PR templates

#### Cargo Workspace Configuration
- [x] **`Cargo.toml`** (workspace root)
  ```toml
  [workspace]
  resolver = "2"
  members = ["crates/*"]
  
  [workspace.package]
  version = "0.1.0"
  edition = "2024"
  license = "MIT"
  repository = "https://github.com/username/ccm"
  
  [workspace.dependencies]
  # Serialization
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  toml = "0.8"
  
  # CLI
  clap = { version = "4.5", features = ["derive", "env"] }
  dialoguer = { version = "0.11", features = ["fuzzy-select"] }
  console = "0.15"
  indicatif = "0.17"
  
  # Security
  keyring = "3"
  aes-gcm = "0.10"
  argon2 = "0.5"
  rand = "0.8"
  
  # Async (for future)
  tokio = { version = "1", features = ["full"] }
  
  # Error handling
  thiserror = "2"
  anyhow = "1"
  
  # Utilities
  directories = "5"
  which = "7"
  url = "2"
  home = "0.5"
  
  # Testing
  tempfile = "3"
  assert_cmd = "2"
  predicates = "3"
  ```

#### Directory Structure
- [x] Create `crates/ccm/` (CLI binary)
- [x] Create `crates/ccm-core/` (library)
- [x] Create `tests/integration/`
- [x] Create `docs/`
- [x] Create `scripts/`

#### Development Environment
- [x] Configure `rustfmt.toml`
- ~~Configure `clippy.toml`~~
- [x] Set up pre-commit hooks
- [x] Create `.env.example`
- [x] Add `rust-toolchain.toml`

### Success Criteria
✅ `cargo build` succeeds  
✅ `cargo test` runs (even with no tests)  
✅ `cargo clippy` passes  
✅ `cargo fmt --check` passes  
✅ GitHub Actions CI runs  

### Deliverables
- Working Cargo workspace
- CI/CD pipeline configured
- Development environment ready
- Initial documentation structure

---

## Phase 1: Core Types & Profile Management

**Duration**: Week 1 (Days 1-7)  
**Status**: 🏗️ In Progress  
**Focus**: Error types, path utilities, profile types, profile manager

### Goals
- Implement error handling infrastructure
- Define core profile types
- Build profile CRUD operations
- Establish path conventions

### Tasks

#### Day 1-2: Error Types & Path Utilities
- [ ] **`crates/ccm-core/src/error.rs`**
  - `CcmError` enum with variants:
    - `ProfileNotFound { name: String }`
    - `ProfileAlreadyExists { name: String }`
    - `InvalidProfileName { name: String, reason: String }`
    - `CredentialNotFound { profile: String }`
    - `CredentialStorageError { message: String }`
    - `KeychainError { message: String }`
    - `EncryptionError { message: String }`
    - `ConfigParseError { path: PathBuf, message: String }`
    - `IoError { path: PathBuf, source: std::io::Error }`
    - `ClaudeSettingsError { message: String }`
    - `ShellDetectionError`
    - `ValidationError { message: String }`
  - User-friendly `Display` implementation with suggestions
  - `thiserror` derive macros
- [ ] **`crates/ccm-core/src/paths.rs`**
  - `config_dir()` → `~/.config/ccm/`
  - `profiles_dir()` → `~/.config/ccm/profiles/`
  - `profile_path(name)` → `~/.config/ccm/profiles/{name}.toml`
  - `default_marker_path()` → `~/.config/ccm/default`
  - `current_marker_path()` → `~/.config/ccm/current`
  - `credentials_path()` → `~/.config/ccm/credentials.enc`
  - `backups_dir()` → `~/.config/ccm/backups/`
  - `claude_settings_path()` → `~/.claude/settings.json`
  - `ensure_dirs()` → Create all required directories
- [ ] **Write tests**: `tests/unit/test_paths.rs`
  - Test all path functions
  - Test directory creation
  - Test cross-platform paths

#### Day 3-4: Profile Types
- [ ] **`crates/ccm-core/src/profile/types.rs`**
  - `Provider` enum:
    ```rust
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "lowercase")]
    pub enum Provider {
        Anthropic,
        OpenRouter,
        Bedrock,
        VertexAi,
        Ollama,
        #[serde(untagged)]
        Custom(String),
    }
    
    impl Provider {
        pub fn default_base_url(&self) -> Option<&'static str>;
        pub fn default_model(&self) -> Option<&'static str>;
        pub fn requires_api_key(&self) -> bool;
        pub fn env_var_name(&self) -> &'static str;
    }
    ```
  - `CredentialSource` enum:
    ```rust
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum CredentialSource {
        #[default]
        Keychain,
        EnvVar(String),
        None,
    }
    ```
  - `Profile` struct:
    ```rust
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Profile {
        pub name: String,
        pub provider: Provider,
        pub model: Option<String>,
        pub base_url: Option<String>,
        pub credential_source: CredentialSource,
        pub timeout_ms: Option<u64>,
        #[serde(default)]
        pub metadata: ProfileMetadata,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct ProfileMetadata {
        pub created_at: Option<String>,
        pub updated_at: Option<String>,
        pub description: Option<String>,
    }
    ```
- [ ] **Write tests**: `tests/unit/test_profile_types.rs`
  - Test serialization/deserialization
  - Test provider methods
  - Test default values

#### Day 5-6: Profile Validation
- [ ] **`crates/ccm-core/src/profile/validation.rs`**
  - `validate_profile_name(name)`:
    - Not empty
    - No reserved names (`default`, `current`, `none`, `_`)
    - Valid characters (alphanumeric, dash, underscore)
    - Max length (64 chars)
  - `validate_profile(profile)`:
    - Valid name
    - Valid URL if provided
    - Valid model format
  - Reserved names constant:
    ```rust
    pub const RESERVED_NAMES: &[&str] = &[
        "default", "current", "none", "all", "list", 
        "help", "version", "_", "-"
    ];
    ```
- [ ] **Write tests**: `tests/unit/test_validation.rs`
  - Test valid names
  - Test invalid names
  - Test reserved names
  - Test URL validation

#### Day 7: Profile Manager
- [ ] **`crates/ccm-core/src/profile/manager.rs`**
  - `ProfileManager` struct:
    ```rust
    pub struct ProfileManager {
        profiles_dir: PathBuf,
    }
    
    impl ProfileManager {
        pub fn new() -> Result<Self, CcmError>;
        
        // CRUD operations
        pub fn create(&self, profile: &Profile) -> Result<(), CcmError>;
        pub fn read(&self, name: &str) -> Result<Profile, CcmError>;
        pub fn update(&self, profile: &Profile) -> Result<(), CcmError>;
        pub fn delete(&self, name: &str) -> Result<(), CcmError>;
        pub fn list(&self) -> Result<Vec<Profile>, CcmError>;
        pub fn exists(&self, name: &str) -> bool;
        
        // Default/current management
        pub fn get_default(&self) -> Result<Option<String>, CcmError>;
        pub fn set_default(&self, name: &str) -> Result<(), CcmError>;
        pub fn clear_default(&self) -> Result<(), CcmError>;
        pub fn get_current(&self) -> Result<Option<String>, CcmError>;
        pub fn set_current(&self, name: &str) -> Result<(), CcmError>;
        pub fn clear_current(&self) -> Result<(), CcmError>;
    }
    ```
  - TOML file format for profiles
  - Atomic writes (temp file + rename)
- [ ] **Write tests**: `tests/unit/test_profile_manager.rs`
  - Test all CRUD operations
  - Test default/current management
  - Test error cases
  - Test concurrent access safety

### Success Criteria
✅ Error types compile with helpful messages  
✅ Path functions work on macOS, Linux, Windows  
✅ Profile types serialize/deserialize correctly  
✅ Validation catches all invalid inputs  
✅ ProfileManager CRUD operations work  
✅ All unit tests pass  

### Deliverables
- Complete error handling system
- Path utilities for all platforms
- Profile types with serialization
- Validation logic
- ProfileManager with CRUD operations
- Unit tests for all components

### Example Usage After This Phase
```rust
use ccm_core::profile::{Profile, Provider, ProfileManager};

let manager = ProfileManager::new()?;

// Create a profile
let profile = Profile {
    name: "work".to_string(),
    provider: Provider::Anthropic,
    model: Some("claude-sonnet-4-5-20250514".to_string()),
    base_url: None,
    credential_source: CredentialSource::Keychain,
    timeout_ms: None,
    metadata: ProfileMetadata::default(),
};

manager.create(&profile)?;
manager.set_default("work")?;

// List all profiles
for profile in manager.list()? {
    println!("{}: {:?}", profile.name, profile.provider);
}
```

---

## Phase 2: Credential Management

**Duration**: Week 2 (Days 8-14)  
**Status**: 🔜 Not Started  
**Focus**: Credential storage backends, secure storage, fallback mechanisms

### Goals
- Build credential store abstraction
- Implement system keychain backend
- Implement encrypted file fallback
- Implement environment variable backend

### Tasks

#### Day 8-9: Credential Store Trait
- [ ] **`crates/ccm-core/src/credential/traits.rs`**
  - `CredentialStore` trait:
    ```rust
    pub trait CredentialStore: Send + Sync {
        /// Store a credential for a profile
        fn store(&self, profile: &str, credential: &str) -> Result<(), CcmError>;
        
        /// Retrieve a credential for a profile
        fn retrieve(&self, profile: &str) -> Result<String, CcmError>;
        
        /// Delete a credential for a profile
        fn delete(&self, profile: &str) -> Result<(), CcmError>;
        
        /// Check if a credential exists
        fn exists(&self, profile: &str) -> Result<bool, CcmError>;
        
        /// Get the backend name for display
        fn backend_name(&self) -> &'static str;
    }
    ```
  - `MaskedCredential` wrapper for safe display:
    ```rust
    pub struct MaskedCredential(String);
    
    impl MaskedCredential {
        pub fn new(credential: String) -> Self;
        pub fn reveal(&self) -> &str;
        pub fn masked(&self) -> String; // "sk-...abc"
    }
    
    impl std::fmt::Display for MaskedCredential {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.masked())
        }
    }
    
    impl std::fmt::Debug for MaskedCredential {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MaskedCredential({})", self.masked())
        }
    }
    ```

#### Day 10-11: Keychain Backend
- [ ] **`crates/ccm-core/src/credential/keychain.rs`**
  - `KeychainStore` struct:
    ```rust
    pub struct KeychainStore {
        service_name: String,
    }
    
    impl KeychainStore {
        pub fn new() -> Self {
            Self {
                service_name: "ccm".to_string(),
            }
        }
        
        pub fn is_available() -> bool;
        
        fn entry(&self, profile: &str) -> keyring::Entry {
            keyring::Entry::new(&self.service_name, profile)
                .expect("Failed to create keyring entry")
        }
    }
    
    impl CredentialStore for KeychainStore {
        fn store(&self, profile: &str, credential: &str) -> Result<(), CcmError>;
        fn retrieve(&self, profile: &str) -> Result<String, CcmError>;
        fn delete(&self, profile: &str) -> Result<(), CcmError>;
        fn exists(&self, profile: &str) -> Result<bool, CcmError>;
        fn backend_name(&self) -> &'static str { "system-keychain" }
    }
    ```
  - Platform-specific handling:
    - macOS: Keychain Services
    - Linux: Secret Service (libsecret/GNOME Keyring)
    - Windows: Credential Manager
- [ ] **Write tests**: `tests/unit/test_keychain.rs`
  - Test store/retrieve/delete
  - Test error handling
  - Skip tests if keychain unavailable

#### Day 12-13: Encrypted File Backend
- [ ] **`crates/ccm-core/src/credential/encrypted.rs`**
  - `EncryptedFileStore` struct:
    ```rust
    pub struct EncryptedFileStore {
        file_path: PathBuf,
    }
    
    impl EncryptedFileStore {
        pub fn new() -> Result<Self, CcmError>;
        
        // Encryption utilities
        fn derive_key(&self) -> Result<[u8; 32], CcmError>;
        fn get_machine_id(&self) -> Result<String, CcmError>;
        fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>, CcmError>;
        fn decrypt(&self, ciphertext: &[u8]) -> Result<String, CcmError>;
        
        // File operations
        fn load_store(&self) -> Result<HashMap<String, Vec<u8>>, CcmError>;
        fn save_store(&self, store: &HashMap<String, Vec<u8>>) -> Result<(), CcmError>;
    }
    ```
  - Encryption details:
    - AES-256-GCM for encryption
    - Argon2id for key derivation
    - Machine ID + user-specific salt
    - Random nonce per encryption
  - File format:
    ```rust
    #[derive(Serialize, Deserialize)]
    struct EncryptedStore {
        version: u32,
        salt: String,  // Base64
        credentials: HashMap<String, EncryptedCredential>,
    }
    
    #[derive(Serialize, Deserialize)]
    struct EncryptedCredential {
        nonce: String,      // Base64
        ciphertext: String, // Base64
    }
    ```
- [ ] **Write tests**: `tests/unit/test_encrypted.rs`
  - Test encryption/decryption roundtrip
  - Test key derivation consistency
  - Test file persistence
  - Test error handling

#### Day 14: Environment Variable Backend & Store Factory
- [ ] **`crates/ccm-core/src/credential/env.rs`**
  - `EnvVarStore` struct (read-only):
    ```rust
    pub struct EnvVarStore;
    
    impl EnvVarStore {
        pub fn new() -> Self { Self }
        
        fn get_env_var_name(profile: &str) -> String {
            format!("CCM_CREDENTIAL_{}", profile.to_uppercase().replace('-', "_"))
        }
    }
    
    impl CredentialStore for EnvVarStore {
        fn store(&self, _profile: &str, _credential: &str) -> Result<(), CcmError> {
            Err(CcmError::CredentialStorageError {
                message: "Environment variable store is read-only".to_string(),
            })
        }
        
        fn retrieve(&self, profile: &str) -> Result<String, CcmError>;
        fn delete(&self, _profile: &str) -> Result<(), CcmError>;
        fn exists(&self, profile: &str) -> Result<bool, CcmError>;
        fn backend_name(&self) -> &'static str { "environment-variable" }
    }
    ```
- [ ] **`crates/ccm-core/src/credential/mod.rs`**
  - Store factory function:
    ```rust
    pub fn get_store_for_profile(profile: &Profile) -> Box<dyn CredentialStore> {
        match &profile.credential_source {
            CredentialSource::Keychain => {
                // Try keychain first, fall back to encrypted file
                if KeychainStore::is_available() {
                    Box::new(KeychainStore::new())
                } else {
                    Box::new(EncryptedFileStore::new().expect("Failed to create encrypted store"))
                }
            }
            CredentialSource::EnvVar(_) => {
                Box::new(EnvVarStore::new())
            }
            CredentialSource::None => {
                Box::new(NoOpStore::new())
            }
        }
    }
    ```
- [ ] **`crates/ccm-core/src/credential/manager.rs`**
  - `CredentialManager` high-level API:
    ```rust
    pub struct CredentialManager {
        profile_manager: ProfileManager,
    }
    
    impl CredentialManager {
        pub fn new(profile_manager: ProfileManager) -> Self;
        
        pub fn store_credential(&self, profile_name: &str, credential: &str) -> Result<(), CcmError>;
        pub fn get_credential(&self, profile_name: &str) -> Result<MaskedCredential, CcmError>;
        pub fn delete_credential(&self, profile_name: &str) -> Result<(), CcmError>;
        pub fn has_credential(&self, profile_name: &str) -> Result<bool, CcmError>;
        pub fn get_backend_name(&self, profile_name: &str) -> Result<&'static str, CcmError>;
    }
    ```
- [ ] **Write tests**: `tests/unit/test_credential_manager.rs`

### Success Criteria
✅ Keychain backend works on all platforms  
✅ Encrypted file fallback works when keychain unavailable  
✅ Environment variable backend works for CI/CD  
✅ Credentials never appear in logs or error messages  
✅ Store factory selects correct backend  
✅ All unit tests pass  

### Deliverables
- CredentialStore trait
- KeychainStore implementation
- EncryptedFileStore implementation
- EnvVarStore implementation
- CredentialManager high-level API
- Comprehensive tests

### Example Usage After This Phase
```rust
use ccm_core::credential::{CredentialManager, MaskedCredential};

let cred_manager = CredentialManager::new(profile_manager);

// Store credential (automatically selects backend based on profile)
cred_manager.store_credential("work", "sk-ant-api03-...")?;

// Retrieve credential (masked for safety)
let cred: MaskedCredential = cred_manager.get_credential("work")?;
println!("Credential: {}", cred); // Prints: "sk-ant-...03-"

// Access raw credential when needed
let api_key = cred.reveal();
```

---

## Phase 3: Configuration Injection & Shell Integration

**Duration**: Week 3 (Days 15-21)  
**Status**: 🔜 Not Started  
**Focus**: Claude Code settings.json injection, shell integration scripts

### Goals
- Build Claude Code settings.json injector
- Implement atomic file operations with backups
- Create shell integration for bash, zsh, fish, PowerShell
- Implement auto-switch on directory change

### Tasks

#### Day 15-16: Claude Code Settings Injector
- [ ] **`crates/ccm-core/src/injector/claude.rs`**
  - `ClaudeSettings` struct (preserve unknown fields):
    ```rust
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct ClaudeSettings {
        #[serde(default)]
        pub env: HashMap<String, String>,
        
        #[serde(flatten)]
        pub other: serde_json::Value,
    }
    ```
  - Environment variable constants:
    ```rust
    pub mod env_vars {
        pub const ANTHROPIC_API_KEY: &str = "ANTHROPIC_API_KEY";
        pub const ANTHROPIC_BASE_URL: &str = "ANTHROPIC_BASE_URL";
        pub const CLAUDE_MODEL: &str = "CLAUDE_MODEL";
        pub const ANTHROPIC_TIMEOUT: &str = "ANTHROPIC_TIMEOUT";
        
        // Markers for ccm-managed variables
        pub const CCM_MANAGED_PREFIX: &str = "CCM_";
        pub const CCM_PROFILE: &str = "CCM_PROFILE";
    }
    ```
  - `ClaudeCodeInjector` struct:
    ```rust
    pub struct ClaudeCodeInjector {
        settings_path: PathBuf,
        backup_dir: PathBuf,
    }
    
    impl ClaudeCodeInjector {
        pub fn new() -> Result<Self, CcmError>;
        
        // Core operations
        pub fn apply(&self, profile: &Profile, credential: Option<&str>) -> Result<(), CcmError>;
        pub fn clear(&self) -> Result<(), CcmError>;
        pub fn get_active_profile(&self) -> Result<Option<String>, CcmError>;
        
        // Settings management
        fn load_settings(&self) -> Result<ClaudeSettings, CcmError>;
        fn save_settings(&self, settings: &ClaudeSettings) -> Result<(), CcmError>;
        
        // Backup management
        fn create_backup(&self) -> Result<PathBuf, CcmError>;
        fn cleanup_old_backups(&self, keep: usize) -> Result<(), CcmError>;
        pub fn restore_backup(&self, backup_path: &Path) -> Result<(), CcmError>;
        pub fn list_backups(&self) -> Result<Vec<PathBuf>, CcmError>;
    }
    ```
- [ ] **Write tests**: `tests/unit/test_injector.rs`
  - Test apply/clear
  - Test preserves existing settings
  - Test backup creation
  - Test backup restoration

#### Day 17-18: Atomic File Operations & Backup
- [ ] **`crates/ccm-core/src/injector/backup.rs`**
  - `BackupManager` struct:
    ```rust
    pub struct BackupManager {
        backup_dir: PathBuf,
        max_backups: usize,
    }
    
    impl BackupManager {
        pub fn new(backup_dir: PathBuf) -> Self;
        
        pub fn create_backup(&self, source: &Path) -> Result<PathBuf, CcmError>;
        pub fn restore_backup(&self, backup: &Path, target: &Path) -> Result<(), CcmError>;
        pub fn list_backups(&self, prefix: &str) -> Result<Vec<BackupInfo>, CcmError>;
        pub fn cleanup(&self, prefix: &str) -> Result<usize, CcmError>;
    }
    
    #[derive(Debug)]
    pub struct BackupInfo {
        pub path: PathBuf,
        pub created_at: SystemTime,
        pub size: u64,
    }
    ```
  - Atomic write helper:
    ```rust
    pub fn atomic_write(path: &Path, content: &[u8]) -> Result<(), CcmError> {
        let temp_path = path.with_extension("tmp");
        std::fs::write(&temp_path, content)?;
        std::fs::rename(&temp_path, path)?;
        Ok(())
    }
    ```
- [ ] **Write tests**: `tests/unit/test_backup.rs`

#### Day 19-20: Shell Integration Scripts
- [ ] **`crates/ccm-core/src/shell/mod.rs`**
  - `Shell` enum:
    ```rust
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Shell {
        Bash,
        Zsh,
        Fish,
        PowerShell,
    }
    
    impl Shell {
        pub fn detect() -> Option<Self>;
        pub fn from_str(s: &str) -> Option<Self>;
        pub fn config_file(&self) -> PathBuf;
        pub fn name(&self) -> &'static str;
    }
    ```
  - `ShellIntegration` trait:
    ```rust
    pub trait ShellIntegration {
        fn generate_script(&self, use_on_cd: bool) -> String;
        fn generate_completions(&self) -> String;
        fn init_command(&self) -> String;
    }
    ```
- [ ] **`crates/ccm-core/src/shell/bash.rs`**
  ```bash
  # ccm shell integration (bash)
  __ccm_resolve_profile() {
      local dir="$PWD"
      while [[ "$dir" != "/" ]]; do
          if [[ -f "$dir/.ccmrc" ]]; then
              ccm use --quiet --from-ccmrc "$dir/.ccmrc"
              return
          fi
          dir="$(dirname "$dir")"
      done
      # No .ccmrc found, use default
      ccm use --quiet --default
  }
  
  __ccm_auto_switch() {
      if [[ "$__CCM_LAST_DIR" != "$PWD" ]]; then
          __CCM_LAST_DIR="$PWD"
          __ccm_resolve_profile
      fi
  }
  
  if [[ -n "$CCM_USE_ON_CD" ]]; then
      PROMPT_COMMAND="__ccm_auto_switch${PROMPT_COMMAND:+;$PROMPT_COMMAND}"
  fi
  ```
- [ ] **`crates/ccm-core/src/shell/zsh.rs`**
- [ ] **`crates/ccm-core/src/shell/fish.rs`**
- [ ] **`crates/ccm-core/src/shell/powershell.rs`**
- [ ] **Write tests**: `tests/unit/test_shell.rs`
  - Test script generation
  - Test shell detection
  - Test completions

#### Day 21: Integration Testing
- [ ] **`tests/integration/test_injection.rs`**
  - Test full injection flow
  - Test backup and restore
  - Test concurrent access
- [ ] **`tests/integration/test_shell.rs`**
  - Test script execution
  - Test auto-switch behavior

### Success Criteria
✅ Claude Code settings.json updated correctly  
✅ Existing settings preserved  
✅ Backups created automatically  
✅ Shell integration works for all 4 shells  
✅ Auto-switch triggers on directory change  
✅ All tests pass  

### Deliverables
- ClaudeCodeInjector with atomic writes
- BackupManager for settings
- Shell integration for bash, zsh, fish, PowerShell
- Auto-switch on cd functionality
- Integration tests

### Example Usage After This Phase
```bash
# Initialize shell integration
eval "$(ccm env --shell bash --use-on-cd)"

# Now whenever you cd into a project with .ccmrc,
# ccm automatically switches profiles
cd ~/projects/work-project  # Switches to "work" profile
cd ~/projects/personal      # Switches to "personal" profile
```

---

## Phase 4: Project Config & Doctor

**Duration**: Week 4 (Days 22-28)  
**Status**: 🔜 Not Started  
**Focus**: .ccmrc project configuration, diagnostic system

### Goals
- Implement .ccmrc parser with override support
- Build recursive config resolution
- Create comprehensive diagnostic system
- Add report formatting

### Tasks

#### Day 22-23: .ccmrc Parser
- [ ] **`crates/ccm-core/src/project/ccmrc.rs`**
  - `Ccmrc` struct:
    ```rust
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Ccmrc {
        pub profile: String,
        
        #[serde(default)]
        pub overrides: Option<CcmrcOverride>,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct CcmrcOverride {
        pub model: Option<String>,
        pub base_url: Option<String>,
        pub timeout_ms: Option<u64>,
    }
    
    impl Ccmrc {
        pub fn load(path: &Path) -> Result<Self, CcmError>;
        pub fn save(&self, path: &Path) -> Result<(), CcmError>;
        pub fn validate(&self) -> Result<(), CcmError>;
    }
    ```
  - Example `.ccmrc`:
    ```toml
    profile = "work"
    
    [overrides]
    model = "claude-sonnet-4-5-20250514"
    timeout_ms = 60000
    ```
- [ ] **Write tests**: `tests/unit/test_ccmrc.rs`

#### Day 24-25: Project Resolver
- [ ] **`crates/ccm-core/src/project/resolver.rs`**
  - `ProjectResolver` struct:
    ```rust
    pub struct ProjectResolver {
        profile_manager: ProfileManager,
    }
    
    impl ProjectResolver {
        pub fn new(profile_manager: ProfileManager) -> Self;
        
        /// Find .ccmrc by walking up from current directory
        pub fn find_ccmrc(&self, start_dir: &Path) -> Option<PathBuf>;
        
        /// Resolve configuration for a directory
        pub fn resolve(&self, dir: &Path) -> Result<ResolvedConfig, CcmError>;
        
        /// Resolve with explicit .ccmrc path
        pub fn resolve_from_ccmrc(&self, ccmrc_path: &Path) -> Result<ResolvedConfig, CcmError>;
    }
    
    #[derive(Debug, Clone)]
    pub struct ResolvedConfig {
        pub profile: Profile,
        pub ccmrc_path: Option<PathBuf>,
        pub applied_overrides: Option<CcmrcOverride>,
    }
    ```
  - Resolution priority:
    1. CLI flags (handled in CLI layer)
    2. .ccmrc overrides
    3. Profile defaults
    4. Provider defaults
- [ ] **Write tests**: `tests/unit/test_resolver.rs`
  - Test recursive search
  - Test override merging
  - Test missing profile handling

#### Day 26-27: Doctor (Diagnostics)
- [ ] **`crates/ccm-core/src/doctor/mod.rs`**
  - `CheckResult` struct:
    ```rust
    #[derive(Debug)]
    pub struct CheckResult {
        pub name: String,
        pub passed: bool,
        pub message: String,
        pub suggestion: Option<String>,
    }
    
    impl CheckResult {
        pub fn pass(name: impl Into<String>, message: impl Into<String>) -> Self;
        pub fn fail(name: impl Into<String>, message: impl Into<String>) -> Self;
        pub fn with_suggestion(self, suggestion: impl Into<String>) -> Self;
    }
    ```
  - Individual checks:
    ```rust
    pub fn check_ccm_installation() -> CheckResult;
    pub fn check_claude_code_cli() -> CheckResult;
    pub fn check_profiles() -> CheckResult;
    pub fn check_keychain() -> CheckResult;
    pub fn check_credentials() -> CheckResult;
    pub fn check_shell_integration() -> CheckResult;
    pub fn check_current_profile() -> CheckResult;
    pub fn check_claude_settings() -> CheckResult;
    ```
  - `Doctor` struct:
    ```rust
    pub struct Doctor {
        profile_manager: ProfileManager,
        credential_manager: CredentialManager,
    }
    
    impl Doctor {
        pub fn new(profile_manager: ProfileManager, credential_manager: CredentialManager) -> Self;
        pub fn run_all_checks(&self) -> Vec<CheckResult>;
        pub fn run_check(&self, name: &str) -> Option<CheckResult>;
        pub fn available_checks(&self) -> Vec<&'static str>;
    }
    ```
- [ ] **`crates/ccm-core/src/doctor/report.rs`**
  - `DoctorReport` struct:
    ```rust
    pub struct DoctorReport {
        pub results: Vec<CheckResult>,
        pub passed: usize,
        pub failed: usize,
    }
    
    impl DoctorReport {
        pub fn from_results(results: Vec<CheckResult>) -> Self;
        pub fn format_console(&self) -> String;
        pub fn format_json(&self) -> String;
        pub fn all_passed(&self) -> bool;
    }
    ```
- [ ] **Write tests**: `tests/unit/test_doctor.rs`

#### Day 28: Integration & Polish
- [ ] **`tests/integration/test_project_config.rs`**
  - Test full config resolution flow
  - Test with nested directories
  - Test missing files handling
- [ ] Polish error messages
- [ ] Add verbose output option

### Success Criteria
✅ .ccmrc parsing works correctly  
✅ Recursive search finds config files  
✅ Override merging works as expected  
✅ Doctor runs all checks  
✅ Clear, actionable diagnostic messages  
✅ All tests pass  

### Deliverables
- Ccmrc parser and validator
- ProjectResolver with recursive search
- Doctor diagnostic system
- Formatted reports
- Integration tests

### Example Usage After This Phase
```bash
# Initialize project config
ccm init
# Creates .ccmrc with current profile

# Check everything is working
ccm doctor

✓ ccm installation
✓ Claude Code CLI found
✓ 3 profiles configured
✓ Keychain accessible
✓ Credentials stored for 2 profiles
✓ Shell integration installed
✓ Current profile: work
✓ Claude settings.json valid

All checks passed (8/8)
```

---

## Phase 5: CLI, Testing & Distribution

**Duration**: Week 5 (Days 29-35)  
**Status**: 🔜 Not Started  
**Focus**: Full CLI implementation, comprehensive testing, distribution

### Goals
- Implement complete CLI with all commands
- Achieve high test coverage
- Create installation scripts
- Set up release automation

### Tasks

#### Day 29-30: CLI Implementation
- [ ] **`crates/ccm/src/cli.rs`**
  - Clap CLI definitions:
    ```rust
    use clap::{Parser, Subcommand};
    
    #[derive(Parser)]
    #[command(name = "ccm")]
    #[command(about = "Claude Code configuration manager")]
    #[command(version)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Commands,
        
        #[arg(short, long, global = true)]
        pub verbose: bool,
    }
    
    #[derive(Subcommand)]
    pub enum Commands {
        /// Add a new profile
        Add(AddArgs),
        /// Remove a profile
        Remove(RemoveArgs),
        /// List all profiles
        List(ListArgs),
        /// Switch to a profile
        Use(UseArgs),
        /// Show current profile
        Current,
        /// Show profile details
        Show(ShowArgs),
        /// Initialize .ccmrc in current directory
        Init(InitArgs),
        /// Run diagnostic checks
        Doctor(DoctorArgs),
        /// Output shell integration script
        Env(EnvArgs),
        /// Manage credentials
        Credential(CredentialArgs),
    }
    ```
  - Command argument structs
- [ ] **`crates/ccm/src/commands/add.rs`**
  - Interactive mode with dialoguer
  - Non-interactive mode with flags
  - Credential input (masked)
- [ ] **`crates/ccm/src/commands/remove.rs`**
  - Confirmation prompt
  - Force flag
- [ ] **`crates/ccm/src/commands/list.rs`**
  - Table format
  - JSON format option
  - Show current/default markers
- [ ] **`crates/ccm/src/commands/use_cmd.rs`**
  - Switch profile
  - Apply to Claude settings
  - Quiet mode for shell scripts
- [ ] **`crates/ccm/src/commands/current.rs`**
- [ ] **`crates/ccm/src/commands/show.rs`**
  - Profile details
  - Credential status (masked)
- [ ] **`crates/ccm/src/commands/init.rs`**
  - Create .ccmrc
  - Use current profile or prompt
- [ ] **`crates/ccm/src/commands/doctor.rs`**
  - Run checks
  - Format output
- [ ] **`crates/ccm/src/commands/env.rs`**
  - Shell detection
  - Script output
  - use-on-cd flag
- [ ] **`crates/ccm/src/commands/credential.rs`**
  - `credential set <profile>`
  - `credential delete <profile>`
  - `credential show <profile>` (masked)

#### Day 31-32: Interactive Features
- [ ] **`crates/ccm/src/interactive.rs`**
  - Interactive profile creation:
    ```rust
    pub fn interactive_add() -> Result<Profile, CcmError> {
        let name: String = Input::new()
            .with_prompt("Profile name")
            .validate_with(validate_profile_name)
            .interact_text()?;
        
        let provider: Provider = Select::new()
            .with_prompt("Provider")
            .items(&[
                "Anthropic (Claude)",
                "OpenRouter",
                "AWS Bedrock",
                "Google Vertex AI",
                "Ollama (local)",
                "Custom",
            ])
            .interact()?
            .into();
        
        // ... continue with other fields
    }
    ```
  - Credential input with masking
  - Confirmation prompts
  - Progress indicators

#### Day 33: Testing
- [ ] **`tests/integration/test_cli.rs`**
  - Test all CLI commands
  - Test argument parsing
  - Test error handling
  - Test interactive mode (mock stdin)
- [ ] **`tests/e2e/test_workflows.rs`**
  - Test: Add profile, store credential, switch, verify
  - Test: Init .ccmrc, auto-switch
  - Test: Doctor with various states
- [ ] Run coverage: `cargo llvm-cov --html`
- [ ] Target: >80% coverage

#### Day 34: Distribution Scripts
- [ ] **`scripts/install.sh`**
  ```bash
  #!/bin/bash
  set -e
  
  VERSION="${CCM_VERSION:-latest}"
  INSTALL_DIR="${CCM_INSTALL_DIR:-$HOME/.local/bin}"
  
  # Detect OS and architecture
  OS=$(uname -s | tr '[:upper:]' '[:lower:]')
  ARCH=$(uname -m)
  
  case "$ARCH" in
      x86_64) ARCH="x86_64" ;;
      aarch64|arm64) ARCH="aarch64" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
  esac
  
  # Download binary
  URL="https://github.com/username/ccm/releases/download/${VERSION}/ccm-${OS}-${ARCH}"
  curl -fsSL "$URL" -o "$INSTALL_DIR/ccm"
  chmod +x "$INSTALL_DIR/ccm"
  
  echo "ccm installed to $INSTALL_DIR/ccm"
  echo "Run 'ccm doctor' to verify installation"
  ```
- [ ] **`scripts/install.ps1`**
  ```powershell
  $Version = $env:CCM_VERSION ?? "latest"
  $InstallDir = $env:CCM_INSTALL_DIR ?? "$env:LOCALAPPDATA\ccm"
  
  # Create install directory
  New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
  
  # Download binary
  $Url = "https://github.com/username/ccm/releases/download/$Version/ccm-windows-x86_64.exe"
  Invoke-WebRequest -Uri $Url -OutFile "$InstallDir\ccm.exe"
  
  # Add to PATH
  $Path = [Environment]::GetEnvironmentVariable("Path", "User")
  if ($Path -notlike "*$InstallDir*") {
      [Environment]::SetEnvironmentVariable("Path", "$Path;$InstallDir", "User")
  }
  
  Write-Host "ccm installed to $InstallDir\ccm.exe"
  Write-Host "Run 'ccm doctor' to verify installation"
  ```

#### Day 35: CI/CD & Release
- [ ] **`.github/workflows/ci.yml`**
  ```yaml
  name: CI
  
  on:
    push:
      branches: [main]
    pull_request:
      branches: [main]
  
  jobs:
    test:
      strategy:
        matrix:
          os: [ubuntu-latest, macos-latest, windows-latest]
      runs-on: ${{ matrix.os }}
      steps:
        - uses: actions/checkout@v4
        - uses: dtolnay/rust-toolchain@stable
        - uses: Swatinem/rust-cache@v2
        - run: cargo test --all-features
        - run: cargo clippy -- -D warnings
        - run: cargo fmt --check
    
    coverage:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4
        - uses: dtolnay/rust-toolchain@stable
        - run: cargo install cargo-llvm-cov
        - run: cargo llvm-cov --lcov --output-path lcov.info
        - uses: codecov/codecov-action@v4
          with:
            files: lcov.info
  ```
- [ ] **`.github/workflows/release.yml`**
  - Build binaries for all platforms
  - Create GitHub release
  - Upload artifacts
  - Publish to crates.io (optional)
- [ ] **`.github/workflows/security.yml`**
  - Run `cargo audit`
  - Dependabot configuration

### Success Criteria
✅ All CLI commands work correctly  
✅ Interactive mode works smoothly  
✅ Test coverage >80%  
✅ CI passes on all platforms  
✅ Install scripts work  
✅ Release workflow automated  

### Deliverables
- Complete CLI binary
- Interactive profile creation
- Comprehensive test suite
- Install scripts for Unix and Windows
- CI/CD pipelines
- Release automation

### Example Usage After This Phase
```bash
# Install ccm
curl -fsSL https://raw.githubusercontent.com/username/ccm/main/scripts/install.sh | bash

# Interactive setup
ccm add
# > Profile name: work
# > Provider: Anthropic (Claude)
# > Model [claude-sonnet-4-5-20250514]: 
# > API Key: **********************
# > Set as default? [Y/n]: Y
# Profile 'work' created and set as default

# List profiles
ccm list
# NAME      PROVIDER   MODEL                          DEFAULT  CURRENT
# work      anthropic  claude-sonnet-4-5-20250514    *        *
# personal  anthropic  claude-sonnet-4-5-20250514

# Switch profile
ccm use personal
# Switched to profile 'personal'

# Check status
ccm current
# personal

# Run diagnostics
ccm doctor
# All checks passed (8/8)

# Set up shell integration
echo 'eval "$(ccm env --shell bash --use-on-cd)"' >> ~/.bashrc
source ~/.bashrc
```

---

## 🎯 MVP Feature Checklist

### Core Features
- [ ] **Profile Management**
  - [ ] Profile CRUD operations
  - [ ] Default profile
  - [ ] Current profile
  - [ ] Profile validation
  
- [ ] **Credential Management**
  - [ ] Keychain backend
  - [ ] Encrypted file fallback
  - [ ] Environment variable backend
  - [ ] Credential masking
  
- [ ] **Configuration Injection**
  - [ ] Claude settings.json injection
  - [ ] Atomic file operations
  - [ ] Automatic backups
  - [ ] Preserve existing settings
  
- [ ] **Shell Integration**
  - [ ] Bash support
  - [ ] Zsh support
  - [ ] Fish support
  - [ ] PowerShell support
  - [ ] Auto-switch on cd
  
- [ ] **Project Configuration**
  - [ ] .ccmrc parser
  - [ ] Recursive search
  - [ ] Override support
  
- [ ] **CLI**
  - [ ] add command
  - [ ] remove command
  - [ ] list command
  - [ ] use command
  - [ ] current command
  - [ ] show command
  - [ ] init command
  - [ ] doctor command
  - [ ] env command
  - [ ] credential command
  
- [ ] **Testing**
  - [ ] Unit tests
  - [ ] Integration tests
  - [ ] E2E tests
  - [ ] >80% coverage
  
- [ ] **Documentation**
  - [ ] README
  - [ ] Getting started guide
  - [ ] CLI reference
  - [ ] Contributing guide

---

## 📊 Success Metrics

### Technical Metrics
- **Test Coverage**: >80%
- **Shell Hook Time**: <5ms
- **Command Response Time**: <50ms
- **Binary Size**: <10MB

### User Metrics
- **Time to First Profile**: <2 minutes from installation
- **Profile Switch Time**: <100ms
- **Documentation Clarity**: Users can self-serve

### Quality Metrics
- **Zero credential leaks**: Credentials never in logs
- **Zero data loss**: Backups for all modifications
- **Cross-platform parity**: Same behavior on all platforms

---

## 🚧 Known Limitations (MVP)

The MVP intentionally excludes:
- ❌ Profile templates (v1.1)
- ❌ Profile import/export (v1.1)
- ❌ Connection testing (v1.1)
- ❌ MCP server configuration (v1.2)
- ❌ Team sharing (v1.2)
- ❌ Usage analytics (v2.0)
- ❌ GUI/TUI interface (v2.0)
- ❌ VS Code extension (v2.0)

These are documented in [ROADMAP.md](./ROADMAP.md) for post-MVP development.

---

## 🎓 Learning Resources

As you implement each phase, refer to:
- **Rust Book**: https://doc.rust-lang.org/book/
- **Clap Documentation**: https://docs.rs/clap/
- **Keyring Crate**: https://docs.rs/keyring/
- **Serde Guide**: https://serde.rs/
- **RustCrypto**: https://github.com/RustCrypto

---

## 🆘 Getting Help

If you get stuck:
1. Check the architecture document
2. Look at similar implementations (fnm, nvm, asdf)
3. Review crate documentation
4. Test with simple examples first
5. Add debug logging to trace issues

---

## 🎉 Celebrating Milestones

- **Phase 0 Complete**: Foundation is solid! 🎯
- **Phase 1 Complete**: Profiles working! 📋
- **Phase 2 Complete**: Credentials secure! 🔐
- **Phase 3 Complete**: Claude injected! 💉
- **Phase 4 Complete**: Diagnostics ready! 🩺
- **Phase 5 Complete**: MVP SHIPPED! 🚀

---

## 📝 Daily Log Template

Keep a development log to track progress:
```markdown
## Day X - [Date]

### Completed
- [ ] Task 1
- [ ] Task 2

### In Progress
- [ ] Task 3

### Blockers
- Issue 1: Description and workaround

### Tomorrow
- [ ] Next task

### Notes
- Any insights or learnings
```

---

<div align="center">

**Let's build something amazing! 💪**

Progress: ░░░░░░░░░░░░░░░░░░░░ 0%

[Back to README](./README.md) • [Full Roadmap](./ROADMAP.md) • [Architecture](./docs/initial-architecture-design.md)

</div>