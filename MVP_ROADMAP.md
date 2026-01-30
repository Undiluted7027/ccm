# ccm MVP Roadmap

> Everything needed to ship v1.0 — focused on what users get and how they'll use it

---

## Table of Contents

1. [MVP Philosophy](#mvp-philosophy)
2. [User Stories & Use Cases](#user-stories--use-cases)
3. [Feature Specifications](#feature-specifications)
4. [Project File Structure](#project-file-structure)
5. [API Design Guidelines](#api-design-guidelines)
6. [Implementation Phases](#implementation-phases)
7. [Definition of Done](#definition-of-done)

---

## MVP Philosophy

### Core Principle: User Value First

Every feature in the MVP must answer:
1. **What does the user get?** (tangible benefit)
2. **How does the user use it?** (practical workflow)
3. **Why would they choose ccm?** (vs. alternatives)

### Non-Goals for MVP

- ❌ Profile registry/sharing marketplace
- ❌ VS Code extension
- ❌ MCP server management
- ❌ Cost tracking
- ❌ Smart routing
- ❌ Plugin system

These are explicitly deferred to post-MVP to maintain focus.

### Success Criteria

1. **Adoption**: 500+ GitHub stars in first month
2. **Reliability**: Zero data loss bugs in credential/profile management
3. **Performance**: < 50ms for any command, < 5ms shell hook overhead
4. **Coverage**: Works on macOS, Linux, and Windows

---

## User Stories & Use Cases

### User Story 1: The Multi-Provider Developer

> "As a developer who uses both Anthropic's API and OpenRouter, I want to switch between them instantly so I don't waste time editing config files."

**Current Pain**:
- Edit `~/.claude/settings.json` manually
- Remember different base URLs and API keys
- Risk typos that break Claude Code

**With ccm**:
```bash
# Setup (once)
ccm add anthropic
# → Wizard prompts for API key, stores securely

ccm add openrouter
# → Wizard prompts for OpenRouter key

# Daily use
ccm use anthropic      # Instant switch
claude                 # Uses Anthropic

ccm use openrouter     # Instant switch  
claude                 # Uses OpenRouter
```

**Why they'd choose ccm**: 
- 2-second switch vs. 2-minute JSON editing
- No risk of config typos
- Credentials stored securely

---

### User Story 2: The Cost-Conscious Developer

> "As a developer working on multiple projects, I want different projects to use different models so I can save money on simple tasks."

**Current Pain**:
- Manually change model before working on each project
- Forget and accidentally use expensive model on simple task
- No way to enforce per-project settings

**With ccm**:
```bash
# In expensive project (needs Opus)
cd ~/work/critical-app
ccm init --profile=opus-prod
# Creates .ccmrc with profile = "opus-prod"

# In learning project (Haiku is fine)
cd ~/personal/experiments
ccm init --profile=haiku-cheap
# Creates .ccmrc with profile = "haiku-cheap"

# Daily use - automatic switching!
cd ~/work/critical-app
# [ccm] Switched to opus-prod
claude                 # Uses Opus

cd ~/personal/experiments
# [ccm] Switched to haiku-cheap  
claude                 # Uses Haiku, saves money
```

**Why they'd choose ccm**:
- Automatic cost control per project
- Never accidentally use wrong model
- Set it once, forget about it

---

### User Story 3: The Privacy-Conscious Developer

> "As a developer working with sensitive code, I want to use local models for certain projects so my code never leaves my machine."

**Current Pain**:
- Complex Ollama setup with Claude Code
- No easy way to switch between cloud and local
- Risk of accidentally sending sensitive code to cloud

**With ccm**:
```bash
# Setup local profile
ccm add local --preset=ollama
# → Configures localhost:11434 as base URL

# Setup cloud profile
ccm add cloud
# → Normal Anthropic setup

# In sensitive project
cd ~/work/secret-project
ccm init --profile=local
# All Claude Code runs against local Ollama

# In normal project
cd ~/personal/open-source
ccm init --profile=cloud
# Uses cloud API
```

**Why they'd choose ccm**:
- Peace of mind for sensitive projects
- Easy toggle between local and cloud
- Project-level enforcement

---

### User Story 4: The Team Lead

> "As a team lead, I want all developers to use the same Claude Code configuration for our project so we have consistent AI assistance."

**Current Pain**:
- Write documentation that nobody follows
- Each developer has different settings
- Debugging issues caused by config differences

**With ccm**:
```bash
# Team lead creates .ccmrc and commits it
echo 'profile = "company-sonnet"' > .ccmrc
echo '[override]' >> .ccmrc
echo 'model = "claude-sonnet-4"' >> .ccmrc
git add .ccmrc
git commit -m "Standardize Claude Code config"

# New developer clones and sets up
git clone company/project
cd project
ccm add company-sonnet
# → Prompted for their own API key

# From now on, automatic compliance
cd project
# [ccm] Switched to company-sonnet
# Everyone uses the same model
```

**Why they'd choose ccm**:
- `.ccmrc` checked into git like `.nvmrc`
- Automatic compliance, no discipline required
- Each dev uses their own credentials

---

### User Story 5: The CI/CD Engineer

> "As a CI/CD engineer, I want to configure Claude Code in automated pipelines without interactive prompts."

**Current Pain**:
- No clear way to configure Claude Code non-interactively
- API keys end up in environment variables everywhere
- Different config for local vs. CI

**With ccm**:
```yaml
# .github/workflows/ai-review.yml
jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install ccm
        run: curl -fsSL https://ccm.dev/install | bash
        
      - name: Configure ccm
        run: |
          ccm add ci \
            --base-url=https://api.anthropic.com \
            --auth-token-env=ANTHROPIC_API_KEY \
            --model=claude-haiku-4 \
            --non-interactive
          ccm use ci
        env:
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
          
      - name: Run Claude Code review
        run: claude -p "Review the changes in this PR"
```

**Why they'd choose ccm**:
- Clean non-interactive mode for CI
- Credentials from environment variables
- Same tool locally and in CI

---

### User Story 6: The Troubleshooter

> "As a developer, when Claude Code isn't working, I want to quickly diagnose if it's a ccm configuration issue."

**Current Pain**:
- No idea if issue is Claude Code, API, or configuration
- Manually check multiple files
- Guess and check debugging

**With ccm**:
```bash
$ ccm doctor

ccm Doctor
──────────

✓ ccm installation
  Version: 1.0.0
  Config dir: ~/.config/ccm

✓ Claude Code CLI
  Version: 2.1.9
  Location: ~/.local/bin/claude

✓ Profiles
  Found 3 profiles: anthropic, openrouter, local
  Default: anthropic

✓ Active profile: openrouter
  Base URL: https://openrouter.ai/api/v1
  Model: anthropic/claude-sonnet-4
  Credentials: ✓ accessible

✓ Shell integration
  Shell: zsh
  Hook installed: yes
  Completions: yes

✓ Project configuration
  Current dir: ~/projects/myapp
  .ccmrc found: yes (profile = "openrouter")

All checks passed! ccm is configured correctly.

If Claude Code still isn't working, the issue is likely:
- API rate limits or quota
- Network connectivity
- Claude Code itself (try: claude --version)
```

**Why they'd choose ccm**:
- One command diagnosis
- Clear indication of what's wrong
- Actionable suggestions

---

## Feature Specifications

### Feature 1: Profile Management

#### What Users Get
- Named configurations they can switch between
- Organized storage of provider settings
- Clear visibility into available options

#### How Users Use It

**Creating a profile (interactive)**:
```bash
$ ccm add myprofile

Creating profile 'myprofile'
────────────────────────────

? Provider base URL: https://openrouter.ai/api/v1
? API key: ****************************************
? Default model: anthropic/claude-sonnet-4
? Small/fast model (optional): anthropic/claude-haiku-4

Testing connection...
✓ Connection successful

? Set as default profile? (y/N): y

Profile 'myprofile' created and set as default.
```

**Creating a profile (non-interactive)**:
```bash
ccm add ciprofile \
  --base-url=https://api.anthropic.com \
  --auth-token-env=API_KEY \
  --model=claude-haiku-4 \
  --non-interactive
```

**Listing profiles**:
```bash
$ ccm list

Profiles
────────
  anthropic       https://api.anthropic.com
→ openrouter      https://openrouter.ai/api/v1
  local           http://localhost:11434

(→ = active, ★ = default)
```

**Switching profiles**:
```bash
$ ccm use local
Switched to 'local' profile

$ ccm current
local
```

**Removing profiles**:
```bash
$ ccm remove old-profile
? Are you sure you want to remove 'old-profile'? (y/N): y
Profile 'old-profile' removed.
```

#### API Design Implications

```rust
// Profile struct
pub struct Profile {
    pub name: String,
    pub provider: ProviderConfig,
    pub metadata: ProfileMetadata,
}

pub struct ProviderConfig {
    pub base_url: String,
    pub model: String,
    pub small_fast_model: Option<String>,
    pub auth_token_source: CredentialSource,
    pub extra_env: HashMap<String, String>,
}

pub enum CredentialSource {
    Keychain { service: String },
    Environment { var_name: String },
    Encrypted { path: PathBuf },
}

// Profile manager interface
pub trait ProfileManager {
    fn list(&self) -> Result<Vec<ProfileSummary>>;
    fn get(&self, name: &str) -> Result<Profile>;
    fn create(&self, config: ProfileConfig) -> Result<Profile>;
    fn update(&self, name: &str, changes: ProfileChanges) -> Result<Profile>;
    fn delete(&self, name: &str) -> Result<()>;
    fn set_default(&self, name: &str) -> Result<()>;
    fn get_default(&self) -> Result<Option<String>>;
}
```

---

### Feature 2: Credential Storage

#### What Users Get
- Secure storage of API keys
- No credentials in plain text files
- Cross-platform consistent experience

#### How Users Use It

**During profile creation** (automatic):
```bash
$ ccm add myprofile
...
? API key: ****************************
  Storing securely in system keychain...
✓ Credential stored
```

**Updating credentials**:
```bash
$ ccm credential set myprofile
? New API key: ****************************
✓ Credential updated
```

**Checking credential status**:
```bash
$ ccm credential status myprofile
Profile: myprofile
Storage: system keychain (macOS Keychain)
Status: ✓ accessible
Last updated: 2026-01-15
```

**Using environment variable (CI)**:
```bash
$ ccm add ci --auth-token-env=ANTHROPIC_API_KEY --non-interactive
Profile 'ci' will read credentials from $ANTHROPIC_API_KEY
```

#### API Design Implications

```rust
// Credential manager interface
pub trait CredentialManager {
    fn store(&self, profile: &str, credential: &str) -> Result<()>;
    fn retrieve(&self, profile: &str) -> Result<String>;
    fn delete(&self, profile: &str) -> Result<()>;
    fn exists(&self, profile: &str) -> Result<bool>;
    fn storage_type(&self) -> StorageType;
}

pub enum StorageType {
    SystemKeychain,
    EncryptedFile,
    EnvironmentVariable,
}

// Platform-specific implementations
pub struct MacOSKeychain;
pub struct LinuxSecretService;
pub struct WindowsCredentialManager;
pub struct EncryptedFileStore;
pub struct EnvironmentVariableStore;

impl CredentialManager for MacOSKeychain {
    // Uses security framework
}

impl CredentialManager for LinuxSecretService {
    // Uses libsecret/kwallet
}

// Automatic backend selection
pub fn default_credential_manager() -> Box<dyn CredentialManager> {
    #[cfg(target_os = "macos")]
    return Box::new(MacOSKeychain::new());
    
    #[cfg(target_os = "linux")]
    return Box::new(LinuxSecretService::new().unwrap_or(EncryptedFileStore::new()));
    
    #[cfg(target_os = "windows")]
    return Box::new(WindowsCredentialManager::new());
}
```

---

### Feature 3: Configuration Injection

#### What Users Get
- Seamless integration with Claude Code
- No manual JSON editing ever
- Preserves other Claude Code settings

#### How Users Use It

**Automatic** (users don't directly interact):
```bash
$ ccm use openrouter
Switched to 'openrouter' profile

# Behind the scenes, ccm updated ~/.claude/settings.json:
# {
#   "env": {
#     "ANTHROPIC_AUTH_TOKEN": "sk-...",
#     "ANTHROPIC_BASE_URL": "https://openrouter.ai/api/v1",
#     "ANTHROPIC_MODEL": "anthropic/claude-sonnet-4"
#   },
#   "permissions": { ... }  ← preserved!
# }
```

**Viewing injected config**:
```bash
$ ccm show openrouter --applied

Profile: openrouter
───────────────────

Provider:
  Base URL: https://openrouter.ai/api/v1
  Model: anthropic/claude-sonnet-4
  Small model: anthropic/claude-haiku-4

Applied to Claude Code:
  ~/.claude/settings.json → env block updated
  
Other settings preserved:
  - permissions (19 rules)
  - mcpServers (3 servers)
```

#### API Design Implications

```rust
// Configuration injector interface
pub trait ConfigInjector {
    fn apply(&self, profile: &Profile) -> Result<()>;
    fn restore(&self) -> Result<()>;
    fn get_applied(&self) -> Result<Option<AppliedConfig>>;
}

pub struct ClaudeCodeInjector {
    settings_path: PathBuf,      // ~/.claude/settings.json
    backup_path: PathBuf,        // ~/.config/ccm/backup/
    state: InjectorState,
}

impl ClaudeCodeInjector {
    pub fn new() -> Self {
        Self {
            settings_path: dirs::home_dir()
                .unwrap()
                .join(".claude")
                .join("settings.json"),
            backup_path: dirs::config_dir()
                .unwrap()
                .join("ccm")
                .join("backup"),
            state: InjectorState::default(),
        }
    }
}

impl ConfigInjector for ClaudeCodeInjector {
    fn apply(&self, profile: &Profile) -> Result<()> {
        // 1. Read existing settings.json
        let mut settings = self.read_settings()?;
        
        // 2. Backup original env block
        self.backup_env(&settings)?;
        
        // 3. Merge profile env vars
        let credential = self.credential_manager.retrieve(&profile.name)?;
        settings.env.insert("ANTHROPIC_AUTH_TOKEN".into(), credential);
        settings.env.insert("ANTHROPIC_BASE_URL".into(), profile.provider.base_url.clone());
        settings.env.insert("ANTHROPIC_MODEL".into(), profile.provider.model.clone());
        
        // 4. Write atomically
        self.write_settings_atomic(&settings)?;
        
        // 5. Update state
        self.state.set_active(profile.name.clone());
        
        Ok(())
    }
}
```

---

### Feature 4: Shell Integration

#### What Users Get
- Automatic profile switching on directory change
- Fast shell startup (< 5ms overhead)
- Tab completion for all commands

#### How Users Use It

**Setup (one-time)**:
```bash
$ ccm env --shell zsh >> ~/.zshrc
$ source ~/.zshrc

# Or with auto-detection
$ ccm env >> ~/.zshrc  # Detects zsh automatically
```

**Enabling auto-switch**:
```bash
$ ccm env --use-on-cd --shell zsh
# Outputs shell code that hooks into cd
```

**Daily use**:
```bash
$ cd ~/projects/webapp
[ccm] Switched to 'local'

$ cd ~/projects/api
[ccm] Switched to 'openrouter'

$ cd ~
# (no message - using default)
```

**Tab completion**:
```bash
$ ccm u<TAB>
use     uninstall

$ ccm use <TAB>
anthropic    local    openrouter

$ ccm add --<TAB>
--base-url    --model    --non-interactive    --preset
```

#### API Design Implications

```rust
// Shell integration generator
pub struct ShellIntegration {
    shell: Shell,
    options: ShellOptions,
}

pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

pub struct ShellOptions {
    pub use_on_cd: bool,
    pub log_level: LogLevel,
}

impl ShellIntegration {
    pub fn generate(&self) -> String {
        match self.shell {
            Shell::Bash => self.generate_bash(),
            Shell::Zsh => self.generate_zsh(),
            Shell::Fish => self.generate_fish(),
            Shell::PowerShell => self.generate_powershell(),
        }
    }
    
    fn generate_zsh(&self) -> String {
        let mut output = String::new();
        
        // Base environment setup
        output.push_str(r#"
export CCM_SHELL="zsh"
export PATH="$HOME/.config/ccm/bin:$PATH"

ccm() {
    command ccm "$@"
    local ret=$?
    
    # Re-export after profile changes
    if [[ "$1" == "use" ]]; then
        export CCM_CURRENT_PROFILE=$(command ccm current --quiet 2>/dev/null)
    fi
    
    return $ret
}
"#);
        
        // Auto-switch hook
        if self.options.use_on_cd {
            output.push_str(r#"
__ccm_auto_switch() {
    local profile=$(command ccm resolve --path="$PWD" --quiet 2>/dev/null)
    if [[ -n "$profile" && "$profile" != "$CCM_CURRENT_PROFILE" ]]; then
        command ccm use "$profile" --quiet
        export CCM_CURRENT_PROFILE="$profile"
        echo "[ccm] Switched to '$profile'"
    fi
}

autoload -U add-zsh-hook
add-zsh-hook chpwd __ccm_auto_switch
__ccm_auto_switch  # Run on shell init
"#);
        }
        
        output
    }
}

// Completion generator
pub struct CompletionGenerator {
    shell: Shell,
}

impl CompletionGenerator {
    pub fn generate(&self) -> String {
        // Generate shell-specific completion scripts
        // Uses clap's built-in completion generation
        todo!()
    }
}
```

---

### Feature 5: Project Configuration

#### What Users Get
- Per-project AI configuration via `.ccmrc`
- Automatic switching when entering project
- Team-sharable settings (commit to git)

#### How Users Use It

**Creating project config**:
```bash
$ cd ~/projects/myapp

$ ccm init
? Profile for this project: local
Created .ccmrc

$ cat .ccmrc
profile = "local"
```

**With overrides**:
```bash
$ ccm init --profile=openrouter

# Edit .ccmrc to add overrides
$ cat .ccmrc
profile = "openrouter"

[override]
model = "anthropic/claude-opus-4"
timeout_ms = 180000
```

**Sharing with team**:
```bash
$ git add .ccmrc
$ git commit -m "Add Claude Code config"

# Teammate clones repo
$ git clone company/myapp
$ cd myapp
[ccm] Profile 'openrouter' not found. Create it? (Y/n): y
# → Interactive wizard starts
```

#### API Design Implications

```rust
// Project config structures
#[derive(Deserialize, Serialize)]
pub struct ProjectConfig {
    pub profile: String,
    pub override_: Option<ProfileOverride>,
}

#[derive(Deserialize, Serialize)]
pub struct ProfileOverride {
    pub model: Option<String>,
    pub small_fast_model: Option<String>,
    pub timeout_ms: Option<u64>,
    pub extra_env: Option<HashMap<String, String>>,
}

// Project config resolver
pub struct ProjectConfigResolver {
    search_paths: Vec<PathBuf>,
}

impl ProjectConfigResolver {
    pub fn find(&self, start_dir: &Path) -> Option<(PathBuf, ProjectConfig)> {
        let mut current = start_dir.to_path_buf();
        
        loop {
            let config_path = current.join(".ccmrc");
            if config_path.exists() {
                if let Ok(config) = self.parse_config(&config_path) {
                    return Some((config_path, config));
                }
            }
            
            if !current.pop() {
                break;
            }
        }
        
        None
    }
    
    pub fn resolve_effective_profile(
        &self,
        profile_manager: &dyn ProfileManager,
        cwd: &Path,
    ) -> Result<Profile> {
        // 1. Check for .ccmrc
        if let Some((_, project_config)) = self.find(cwd) {
            let mut profile = profile_manager.get(&project_config.profile)?;
            
            // 2. Apply overrides
            if let Some(overrides) = project_config.override_ {
                if let Some(model) = overrides.model {
                    profile.provider.model = model;
                }
                // ... apply other overrides
            }
            
            return Ok(profile);
        }
        
        // 3. Fall back to default
        if let Some(default_name) = profile_manager.get_default()? {
            return profile_manager.get(&default_name);
        }
        
        Err(Error::NoActiveProfile)
    }
}
```

---

### Feature 6: Doctor Command

#### What Users Get
- One-command health check
- Clear diagnosis of issues
- Actionable fix suggestions

#### How Users Use It

**Basic check**:
```bash
$ ccm doctor

ccm Doctor
══════════

Installation
  ✓ ccm binary: ~/.local/bin/ccm (v1.0.0)
  ✓ Config directory: ~/.config/ccm

Claude Code
  ✓ CLI installed: ~/.local/bin/claude (v2.1.9)
  ✓ Settings file: ~/.claude/settings.json

Profiles
  ✓ Profile count: 3
  ✓ Default profile: anthropic

Active Configuration
  ✓ Active profile: openrouter
  ✓ Credentials: accessible

Shell Integration
  ✓ Shell: zsh
  ✓ Hook installed: yes
  ✓ Completions: yes

All checks passed!
```

**With problems**:
```bash
$ ccm doctor

ccm Doctor
══════════

Installation
  ✓ ccm binary: ~/.local/bin/ccm (v1.0.0)
  ✓ Config directory: ~/.config/ccm

Claude Code
  ✗ CLI not found
    
    Claude Code CLI is not installed or not in PATH.
    
    To install Claude Code:
      npm install -g @anthropic-ai/claude-code
    
    Or visit: https://docs.anthropic.com/claude-code

Profiles
  ✓ Profile count: 3
  ⚠ Default profile 'old-profile' does not exist
    
    Your default profile references a deleted profile.
    
    To fix:
      ccm default anthropic

1 error, 1 warning found.
```

#### API Design Implications

```rust
// Doctor check system
pub struct Doctor {
    checks: Vec<Box<dyn DoctorCheck>>,
}

pub trait DoctorCheck {
    fn name(&self) -> &str;
    fn run(&self, context: &DoctorContext) -> CheckResult;
}

pub enum CheckResult {
    Pass { message: String },
    Warning { message: String, fix: Option<String> },
    Error { message: String, fix: Option<String> },
    Skip { reason: String },
}

pub struct DoctorContext {
    pub config_dir: PathBuf,
    pub profile_manager: Arc<dyn ProfileManager>,
    pub credential_manager: Arc<dyn CredentialManager>,
}

// Individual checks
pub struct CcmInstallationCheck;
pub struct ClaudeCodeCheck;
pub struct ProfilesCheck;
pub struct CredentialsCheck;
pub struct ShellIntegrationCheck;
pub struct ActiveProfileCheck;

impl Doctor {
    pub fn new() -> Self {
        Self {
            checks: vec![
                Box::new(CcmInstallationCheck),
                Box::new(ClaudeCodeCheck),
                Box::new(ProfilesCheck),
                Box::new(CredentialsCheck),
                Box::new(ShellIntegrationCheck),
                Box::new(ActiveProfileCheck),
            ],
        }
    }
    
    pub fn run(&self, context: &DoctorContext) -> DoctorReport {
        let results: Vec<_> = self.checks
            .iter()
            .map(|check| (check.name().to_string(), check.run(context)))
            .collect();
        
        DoctorReport { results }
    }
}
```

---

## Project File Structure

```
ccm/
├── Cargo.toml                    # Workspace manifest
├── Cargo.lock
├── README.md
├── LICENSE                       # MIT
├── CHANGELOG.md
├── CONTRIBUTING.md
│
├── docs/
│   ├── installation.md
│   ├── getting-started.md
│   ├── configuration.md
│   ├── shell-integration.md
│   └── ci-cd.md
│
├── crates/
│   ├── ccm/                      # Main CLI binary
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs           # Entry point
│   │       ├── cli.rs            # Clap command definitions
│   │       ├── commands/
│   │       │   ├── mod.rs
│   │       │   ├── add.rs        # ccm add
│   │       │   ├── remove.rs     # ccm remove
│   │       │   ├── list.rs       # ccm list
│   │       │   ├── use_.rs       # ccm use
│   │       │   ├── current.rs    # ccm current
│   │       │   ├── show.rs       # ccm show
│   │       │   ├── init.rs       # ccm init
│   │       │   ├── doctor.rs     # ccm doctor
│   │       │   ├── env.rs        # ccm env
│   │       │   ├── credential.rs # ccm credential
│   │       │   └── completions.rs
│   │       └── output.rs         # Formatting, colors
│   │
│   ├── ccm-core/                 # Core library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── profile/
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # ProfileManager trait + impl
│   │       │   ├── model.rs      # Profile, ProviderConfig structs
│   │       │   └── validation.rs
│   │       ├── credential/
│   │       │   ├── mod.rs
│   │       │   ├── manager.rs    # CredentialManager trait
│   │       │   ├── keychain.rs   # Platform-specific keychain
│   │       │   ├── encrypted.rs  # Encrypted file fallback
│   │       │   └── env.rs        # Environment variable source
│   │       ├── injector/
│   │       │   ├── mod.rs
│   │       │   ├── claude_code.rs # ClaudeCodeInjector
│   │       │   └── atomic.rs     # Atomic file writes
│   │       ├── project/
│   │       │   ├── mod.rs
│   │       │   ├── config.rs     # ProjectConfig model
│   │       │   └── resolver.rs   # Find .ccmrc
│   │       ├── shell/
│   │       │   ├── mod.rs
│   │       │   ├── integration.rs
│   │       │   ├── bash.rs
│   │       │   ├── zsh.rs
│   │       │   ├── fish.rs
│   │       │   └── powershell.rs
│   │       ├── doctor/
│   │       │   ├── mod.rs
│   │       │   ├── checks.rs
│   │       │   └── report.rs
│   │       ├── config.rs         # Global ccm config
│   │       ├── state.rs          # Runtime state tracking
│   │       ├── paths.rs          # XDG-compliant paths
│   │       └── error.rs          # Error types
│   │
│   └── ccm-test-utils/           # Shared test utilities
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── fixtures.rs
│           └── mock.rs
│
├── tests/                        # Integration tests
│   ├── integration/
│   │   ├── add_test.rs
│   │   ├── use_test.rs
│   │   ├── shell_test.rs
│   │   └── project_test.rs
│   └── e2e/
│       └── full_workflow_test.rs
│
├── scripts/
│   ├── install.sh               # curl installer
│   ├── install.ps1              # PowerShell installer
│   └── release.sh               # Build release binaries
│
├── .github/
│   ├── workflows/
│   │   ├── ci.yml               # Tests on PR
│   │   ├── release.yml          # Build + publish on tag
│   │   └── audit.yml            # Security audit
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE.md
│
└── assets/
    ├── logo.svg
    └── demo.gif
```

---

## API Design Guidelines

### Naming Conventions

| Context | Convention | Example |
|---------|------------|---------|
| Commands | Verb (single word) | `add`, `use`, `list`, `remove` |
| Subcommands | Noun or verb-noun | `credential set`, `env` |
| Flags | Lowercase kebab-case | `--base-url`, `--non-interactive` |
| Environment vars | SCREAMING_SNAKE_CASE | `CCM_CURRENT_PROFILE` |
| Config keys | snake_case | `default_profile`, `auth_token` |
| Struct fields | snake_case | `base_url`, `small_fast_model` |
| Trait methods | snake_case | `get_profile`, `apply_config` |

### Error Handling Pattern

```rust
// Custom error type with user-friendly messages
#[derive(Debug, thiserror::Error)]
pub enum CcmError {
    #[error("Profile '{0}' not found\n\nAvailable profiles:\n{1}\n\nTo create a new profile:\n  ccm add {0}")]
    ProfileNotFound(String, String),
    
    #[error("Failed to access credential for '{0}'\n\nThe credential may have been deleted or the keychain is locked.\n\nTo update the credential:\n  ccm credential set {0}")]
    CredentialAccessFailed(String),
    
    #[error("Claude Code settings file is malformed\n\nPath: {0}\n\nTo fix, either:\n  1. Fix the JSON manually\n  2. Delete the file and let Claude Code recreate it\n  3. Run: ccm doctor --fix")]
    SettingsMalformed(PathBuf),
}

// Result type alias
pub type Result<T> = std::result::Result<T, CcmError>;
```

### Command Interface Pattern

```rust
// Each command follows this pattern
pub trait Command {
    type Args;
    type Output;
    
    fn execute(&self, args: Self::Args, context: &Context) -> Result<Self::Output>;
}

// Example: ccm use
pub struct UseCommand;

pub struct UseArgs {
    pub profile: String,
    pub quiet: bool,
}

pub struct UseOutput {
    pub previous_profile: Option<String>,
    pub new_profile: String,
}

impl Command for UseCommand {
    type Args = UseArgs;
    type Output = UseOutput;
    
    fn execute(&self, args: Self::Args, context: &Context) -> Result<Self::Output> {
        let previous = context.state.get_active_profile()?;
        
        let profile = context.profile_manager.get(&args.profile)?;
        context.injector.apply(&profile)?;
        context.state.set_active_profile(&args.profile)?;
        
        Ok(UseOutput {
            previous_profile: previous,
            new_profile: args.profile,
        })
    }
}
```

### Configuration File Format

```toml
# ~/.config/ccm/config.toml
[settings]
default_profile = "anthropic"
log_level = "info"              # error, warn, info, debug, trace
color = "auto"                  # auto, always, never

[shell]
auto_switch = true
show_profile_in_prompt = true

[security]
credential_backend = "auto"     # auto, keychain, encrypted, env
```

### Profile File Format

```toml
# ~/.config/ccm/profiles/openrouter.toml
[profile]
name = "openrouter"
description = "Multi-model access via OpenRouter"
created_at = "2026-01-30T10:00:00Z"
last_used = "2026-01-30T14:32:00Z"

[provider]
base_url = "https://openrouter.ai/api/v1"
model = "anthropic/claude-sonnet-4"
small_fast_model = "openai/gpt-4o-mini"

[credentials]
source = "keychain"
keychain_service = "ccm/openrouter"

[extra_env]
API_TIMEOUT_MS = "120000"
```

---

## Implementation Phases

### Phase 1: Foundation (Week 1-2)

**Goal**: Core data structures and profile management working

**Tasks**:
- [ ] Set up Rust workspace structure
- [ ] Implement `Profile` and `ProviderConfig` models
- [ ] Implement profile TOML serialization/deserialization
- [ ] Implement `ProfileManager` with CRUD operations
- [ ] Implement basic CLI skeleton with clap
- [ ] Commands: `add`, `remove`, `list`, `show`
- [ ] Unit tests for profile management

**Deliverable**: Can create and list profiles (without credentials or injection)

### Phase 2: Credentials & Injection (Week 2-3)

**Goal**: Secure credential storage and Claude Code integration

**Tasks**:
- [ ] Implement keychain backend (macOS first)
- [ ] Implement encrypted file fallback
- [ ] Implement environment variable source
- [ ] Implement `ClaudeCodeInjector`
- [ ] Implement atomic file writes with backup
- [ ] Commands: `use`, `current`, `credential`
- [ ] Integration tests for full profile → injection flow

**Deliverable**: Can switch profiles and Claude Code uses correct config

### Phase 3: Shell Integration (Week 3-4)

**Goal**: Auto-switching and completions working

**Tasks**:
- [ ] Implement shell integration generator
- [ ] Implement zsh support first (most common)
- [ ] Implement bash support
- [ ] Implement fish support
- [ ] Implement cd hook for auto-switching
- [ ] Implement completion generator
- [ ] Commands: `env`, `completions`
- [ ] Manual testing across shells

**Deliverable**: `eval "$(ccm env)"` enables auto-switching

### Phase 4: Project Config & Doctor (Week 4-5)

**Goal**: Per-project config and troubleshooting

**Tasks**:
- [ ] Implement `.ccmrc` parser
- [ ] Implement recursive project config resolver
- [ ] Implement profile override merging
- [ ] Implement doctor check system
- [ ] Implement individual doctor checks
- [ ] Commands: `init`, `doctor`
- [ ] End-to-end tests for complete workflows

**Deliverable**: Projects can have `.ccmrc`, doctor diagnoses issues

### Phase 5: Polish & Distribution (Week 5-6)

**Goal**: Production-ready release

**Tasks**:
- [ ] Implement Linux keychain support
- [ ] Implement Windows credential manager
- [ ] Implement PowerShell support
- [ ] Create install script
- [ ] Set up GitHub Actions for CI/CD
- [ ] Set up release workflow for binaries
- [ ] Write documentation
- [ ] Create demo GIF
- [ ] Publish to Homebrew, Cargo, npm

**Deliverable**: v1.0.0 release

---

## Definition of Done

### For Each Feature

- [ ] Implementation complete
- [ ] Unit tests written and passing
- [ ] Integration tests for user workflows
- [ ] Error messages are user-friendly
- [ ] Documentation updated
- [ ] No compiler warnings
- [ ] `cargo clippy` passes
- [ ] `cargo fmt` applied

### For MVP Release

- [ ] All Phase 1-5 tasks complete
- [ ] All tests passing on macOS, Linux, Windows
- [ ] Performance targets met (< 50ms commands, < 5ms hooks)
- [ ] README with getting started guide
- [ ] Install script tested on fresh systems
- [ ] Release binaries built for all platforms
- [ ] Homebrew formula submitted
- [ ] Demo GIF created
- [ ] Announcement post drafted

---

## Metrics to Track

### Pre-Launch

- [ ] All tests passing in CI
- [ ] `ccm use` latency < 50ms (bench tests)
- [ ] Shell hook overhead < 5ms (bench tests)
- [ ] Binary size < 10MB

### Post-Launch (First Month)

- GitHub stars
- Install script downloads
- GitHub issues opened (bugs vs. features)
- Time to first issue resolution
- Community PRs

---

*MVP Roadmap Version: 1.0*  
*Last Updated: January 30, 2026*