# ccm Roadmap

> The journey to making Claude Code configuration as simple as `fnm use` for Node.js

---

## Vision

**ccm** aims to become the standard way developers manage Claude Code configurations, just as fnm became the preferred Node.js version manager by prioritizing speed, simplicity, and developer experience.

### The World We're Building

Imagine a world where:

- **Switching AI providers is instant**: `ccm use openrouter` takes milliseconds, not minutes of JSON editing
- **Projects declare their AI requirements**: A `.ccmrc` file in your repo ensures everyone uses the right model
- **Credentials are secure by default**: API keys live in your system keychain, not scattered across dotfiles
- **Teams share configurations, not secrets**: Export a profile template, let teammates add their own keys
- **The AI coding experience is provider-agnostic**: Use Claude Code's excellent UX with any compatible backend

### Why This Matters

The AI coding assistant landscape is evolving rapidly:
- OpenRouter offers 200+ models through one API
- Local models (Ollama, LMStudio) provide privacy and cost savings
- Cloud providers (AWS Bedrock, Google Vertex AI) offer enterprise compliance
- New providers emerge monthly with competitive pricing

Developers shouldn't be locked into one provider. They shouldn't manually edit JSON files. They deserve the same DX for AI configuration that they have for everything else.

---

## Current State: The Problem

### Today's Claude Code Configuration

```
~/.claude/
├── settings.json           # Manual JSON editing 😫
├── settings.local.json     # More JSON...
└── .claude.json            # Legacy format still used

Environment variables:
- ANTHROPIC_API_KEY         # Where does this go?
- ANTHROPIC_BASE_URL        # In .bashrc? .zshrc? settings.json?
- ANTHROPIC_MODEL           # Different per project?
```

### What Developers Resort To

1. **Shell aliases** that break when context changes
2. **Multiple JSON files** copied between projects  
3. **Wrapper scripts** like `claudecode-switch` (2 stars, limited features)
4. **Full alternatives** like OpenCode (loses Claude Code's polish)
5. **Just giving up** and using one config everywhere

---

## Roadmap Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              MVP (v1.0)                                 │
│                           February 2026                                 │
├─────────────────────────────────────────────────────────────────────────┤
│  ✓ Profile management (add, remove, list, use)                         │
│  ✓ Secure credential storage (keychain + encrypted fallback)           │
│  ✓ Shell integration (bash, zsh, fish) with auto-switch                │
│  ✓ Project config (.ccmrc) with directory-based auto-switching         │
│  ✓ Doctor command for troubleshooting                                  │
│  ✓ Cross-platform support (macOS, Linux, Windows)                      │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                            v1.1 - Polish                                │
│                            March 2026                                   │
├─────────────────────────────────────────────────────────────────────────┤
│  • Profile templates (quick-start for common providers)                 │
│  • Connection testing (`ccm test <profile>`)                            │
│  • Profile inheritance (base + overrides)                               │
│  • Improved error messages with suggestions                             │
│  • Shell prompt integration (show current profile)                      │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           v1.2 - Teams                                  │
│                            April 2026                                   │
├─────────────────────────────────────────────────────────────────────────┤
│  • Profile export/import (shareable configs without secrets)            │
│  • Organization profile templates (company-wide defaults)               │
│  • Audit logging (compliance-friendly)                                  │
│  • Environment variable expansion in profiles                           │
│  • CI/CD helpers (GitHub Actions, GitLab CI)                            │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         v2.0 - Ecosystem                                │
│                            Q3 2026                                      │
├─────────────────────────────────────────────────────────────────────────┤
│  • MCP server configuration management                                  │
│  • Profile registry (community-shared configs)                          │
│  • VS Code extension (status bar, quick switch)                         │
│  • Claude Desktop support                                               │
│  • Cost tracking per profile                                            │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                          Future Vision                                  │
│                           2026+                                         │
├─────────────────────────────────────────────────────────────────────────┤
│  • Smart routing (automatic model selection based on task)              │
│  • Provider health monitoring (failover when API is down)               │
│  • Usage analytics dashboard                                            │
│  • Multi-tool support (Cursor, Windsurf, other AI IDEs)                 │
│  • Plugin system for custom providers                                   │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## MVP (v1.0) — February 2026

### What We're Building

A fast, reliable, cross-platform CLI that handles the core use case: **switching between Claude Code configurations effortlessly**.

### Feature Breakdown

#### 1. Profile Management

**What it does**: Create, store, list, and switch between named configurations.

**How it works**:
- Profiles stored as individual TOML files in `~/.config/ccm/profiles/`
- Each profile defines: provider URL, model, and credential reference
- One profile can be marked as default

**Commands**:
```bash
ccm add anthropic          # Interactive wizard to create profile
ccm add local --preset     # Use built-in preset for Ollama
ccm list                   # Show all profiles with indicators
ccm use openrouter         # Switch active profile
ccm current                # Show what's active
ccm remove old-profile     # Delete a profile
```

**Why it matters**: Eliminates manual JSON editing for the 90% use case.

#### 2. Secure Credential Storage

**What it does**: Stores API keys securely, never in plain text.

**How it works**:
- Primary: System keychain (macOS Keychain, Linux libsecret, Windows Credential Manager)
- Fallback: AES-256 encrypted file at `~/.config/ccm/.credentials`
- Profile files only contain references, not actual keys

**Commands**:
```bash
ccm add myprofile          # Prompts for API key, stores securely
ccm credential set local   # Update credential for existing profile
ccm credential show local  # Display (masked) credential status
```

**Why it matters**: Prevents accidental credential exposure in git commits.

#### 3. Shell Integration

**What it does**: Enables instant switching and automatic per-directory profiles.

**How it works**:
- User adds one line to shell config: `eval "$(ccm env --use-on-cd)"`
- ccm generates shell-specific hooks for cd, prompt, and completions
- Written in Rust for < 5ms startup impact (following fnm's approach)

**Supported shells**:
- Bash
- Zsh  
- Fish
- PowerShell

**Commands**:
```bash
ccm env --shell bash       # Output bash integration code
ccm env --use-on-cd        # Include auto-switch hook
ccm completions bash       # Generate tab completions
```

**Why it matters**: Makes profile switching invisible and automatic.

#### 4. Project Configuration

**What it does**: Allows projects to declare their required Claude Code configuration.

**How it works**:
- Projects include a `.ccmrc` file (like `.nvmrc` for Node)
- Shell hook detects file on `cd` and switches automatically
- Supports profile name and optional overrides

**File format** (`.ccmrc`):
```toml
profile = "local"

[override]
model = "deepseek-coder:33b"
timeout_ms = 180000
```

**Commands**:
```bash
ccm init                   # Create .ccmrc in current directory
ccm init --profile=local   # Create with specific profile
```

**Why it matters**: Teams can standardize AI configurations per repository.

#### 5. Doctor Command

**What it does**: Diagnoses common issues and guides users to solutions.

**What it checks**:
1. ccm installation and configuration directory
2. Claude Code CLI presence and version
3. Profile validity (required fields, URL format)
4. Credential accessibility
5. Shell integration status
6. Active profile health

**Commands**:
```bash
ccm doctor                 # Run all diagnostics
ccm doctor --fix           # Attempt automatic fixes
```

**Why it matters**: Reduces support burden, empowers self-service troubleshooting.

#### 6. Cross-Platform Support

**What it does**: Works consistently across macOS, Linux, and Windows.

**How it works**:
- Single Rust binary with platform-specific keychain backends
- Path handling uses `dirs` crate for XDG compliance
- Windows supports both PowerShell and Git Bash

**Distribution**:
- Install script: `curl -fsSL https://ccm.dev/install | bash`
- Homebrew: `brew install ccm`
- Cargo: `cargo install ccm`
- Scoop: `scoop install ccm`
- GitHub Releases: Pre-built binaries

**Why it matters**: No one left behind regardless of their development environment.

---

## Post-MVP Roadmap

### v1.1 — Polish (March 2026)

#### Profile Templates

**Goal**: Reduce time-to-first-use for common providers.

**Implementation**:
- Built-in presets for: Anthropic, OpenRouter, Ollama, AWS Bedrock, Google Vertex AI
- `ccm add myprofile --preset=openrouter` fills in base_url and model suggestions
- Presets bundled in binary, updated with releases

**User Experience**:
```bash
$ ccm add --preset
? Select a provider template:
  > Anthropic (Official Claude API)
    OpenRouter (Multi-model gateway)
    Ollama (Local models)
    AWS Bedrock (Enterprise)
    Custom (Manual configuration)
```

#### Connection Testing

**Goal**: Validate configurations before use, catch issues early.

**Implementation**:
- `ccm test <profile>` sends minimal API request to verify credentials
- Reports latency, model availability, quota status (where supported)
- Integrated into `ccm add` wizard (optional)

**User Experience**:
```bash
$ ccm test openrouter
Testing openrouter...
✓ Connection successful (234ms)
✓ Model claude-sonnet-4 available
✓ API key valid (expires: never)
```

#### Profile Inheritance

**Goal**: Enable DRY configurations with shared base settings.

**Implementation**:
- Profiles can specify `extends = "base-profile"`
- Child profile inherits all settings, overrides specific fields
- Useful for team-wide defaults with individual customizations

**Profile format**:
```toml
[profile]
name = "my-openrouter"
extends = "team-openrouter"

[provider]
model = "anthropic/claude-opus-4"  # Override model only
```

#### Shell Prompt Integration

**Goal**: Always know which profile is active at a glance.

**Implementation**:
- `ccm env` exports `CCM_CURRENT_PROFILE` environment variable
- Provide example prompt configurations for popular themes
- Document integration with Starship, Oh My Zsh, etc.

**User Experience**:
```bash
~/projects/webapp (main) [ccm:local] $
```

---

### v1.2 — Teams (April 2026)

#### Profile Export/Import

**Goal**: Share configurations safely across teams.

**Implementation**:
- `ccm export <profile> > team-profile.toml` produces credential-free TOML
- `ccm import team-profile.toml` creates profile, prompts for credentials
- Support URL import: `ccm import https://company.com/profiles/default.toml`

**User Experience**:
```bash
# Team lead exports
$ ccm export company-standard > company.toml
$ git add company.toml

# New team member imports
$ ccm import ./company.toml
Profile 'company-standard' imported
Enter API key for company-standard: ****
✓ Profile ready to use
```

#### Organization Profile Templates

**Goal**: Centralized configuration management for companies.

**Implementation**:
- Support for loading profiles from network locations
- `ccm pull <org-url>` syncs organization templates
- Read-only organization profiles that can't be locally modified

**Configuration** (`~/.config/ccm/config.toml`):
```toml
[organization]
template_url = "https://internal.company.com/ccm-profiles/"
sync_interval = "24h"
require_approval = true
```

#### Audit Logging

**Goal**: Compliance-friendly tracking of configuration changes.

**Implementation**:
- Append-only log at `~/.config/ccm/audit.log`
- Records: timestamp, action, profile name, user (for multi-user systems)
- Optional forwarding to external logging systems

**Log format**:
```json
{"ts":"2026-04-15T10:23:45Z","action":"use","profile":"production","prev":"development"}
{"ts":"2026-04-15T10:25:12Z","action":"add","profile":"test-openrouter"}
```

#### CI/CD Helpers

**Goal**: First-class support for automated environments.

**Implementation**:
- GitHub Action: `anthropic/ccm-action`
- GitLab CI template
- Non-interactive mode: `ccm add --non-interactive --auth-token-env=API_KEY`

**GitHub Action example**:
```yaml
- uses: ccm-dev/setup-ccm@v1
  with:
    profile: ci-runner
    auth-token: ${{ secrets.ANTHROPIC_API_KEY }}
    model: claude-haiku-4
```

---

### v2.0 — Ecosystem (Q3 2026)

#### MCP Server Configuration Management

**Goal**: Extend ccm to manage Model Context Protocol servers alongside providers.

**Implementation**:
- Profiles can include MCP server configurations
- ccm writes to `.mcp.json` in addition to `settings.json`
- Toggle MCP servers per profile

**Profile format**:
```toml
[profile]
name = "full-stack"

[mcp.servers.github]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-github"]
env = { GITHUB_TOKEN = "keychain:ccm/github" }

[mcp.servers.postgres]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-postgres"]
env = { DATABASE_URL = "env:DATABASE_URL" }
```

#### Profile Registry

**Goal**: Discover and share community profiles.

**Implementation**:
- Central registry (like npm, crates.io) for profile templates
- `ccm search openrouter` finds community profiles
- `ccm install @community/openrouter-optimized` installs template
- Verification badges for trusted publishers

**User Experience**:
```bash
$ ccm search local
  @ccm/ollama-default      Official Ollama preset
  @community/lmstudio      LMStudio integration (★ 234)
  @company/secure-local    Air-gapped deployment template

$ ccm install @ccm/ollama-default
```

#### VS Code Extension

**Goal**: Visual profile management without leaving the editor.

**Implementation**:
- Status bar showing current profile
- Command palette: "ccm: Switch Profile"
- Sidebar panel listing profiles with quick-switch
- Automatic `.ccmrc` detection and prompts

#### Claude Desktop Support

**Goal**: Manage Claude Desktop app configuration alongside CLI.

**Implementation**:
- Detect Claude Desktop installation
- Write to appropriate config location
- Unified profile switching for both CLI and Desktop

#### Cost Tracking

**Goal**: Visibility into API costs per profile.

**Implementation**:
- Track token usage when possible (via response headers)
- Estimate costs based on model pricing
- `ccm stats` shows usage breakdown
- Optional warnings when approaching budgets

---

## Future Vision (2026+)

### Smart Routing

**Concept**: Automatically select the best model for each task.

**How it might work**:
- Analyze task complexity from prompt
- Route simple tasks to fast/cheap models (Haiku)
- Route complex tasks to powerful models (Opus)
- Learn from user corrections

**Profile format**:
```toml
[routing]
strategy = "smart"
fast_model = "claude-haiku-4"
default_model = "claude-sonnet-4"  
complex_model = "claude-opus-4"
complexity_threshold = 0.7
```

### Provider Health Monitoring

**Concept**: Automatic failover when primary provider has issues.

**How it might work**:
- Background health checks on active provider
- Define fallback chain in profile
- Automatic switch with notification
- Manual override always available

**Profile format**:
```toml
[failover]
enabled = true
fallback_profiles = ["openrouter-backup", "local-ollama"]
health_check_interval = "5m"
```

### Usage Analytics Dashboard

**Concept**: Web dashboard for teams to understand AI usage patterns.

**How it might work**:
- Optional telemetry (explicit opt-in)
- Team aggregation (no individual tracking)
- Insights: popular profiles, cost trends, failure rates
- Self-hostable for enterprises

### Multi-Tool Support

**Concept**: Extend beyond Claude Code to other AI coding tools.

**Potential targets**:
- Cursor
- Windsurf
- Cline
- Aider
- Continue.dev

**Implementation approach**:
- Plugin architecture for tool-specific configuration writers
- Unified profile format across tools
- Single `ccm use` switches all configured tools

### Plugin System

**Concept**: Allow community extensions for custom providers and features.

**How it might work**:
- Plugins as separate binaries following ccm protocol
- Discovery via plugin registry
- Examples: custom auth flows, private registries, enterprise integrations

---

## How to Extract Post-MVP Roadmap

For planning purposes, here's a structured view of post-MVP work:

### Post-MVP Phase 1: Polish (4-6 weeks)
- [ ] Profile templates/presets for common providers
- [ ] `ccm test` connection validation
- [ ] Profile inheritance (extends)
- [ ] Shell prompt integration
- [ ] Enhanced error messages

### Post-MVP Phase 2: Teams (6-8 weeks)
- [ ] Profile export/import
- [ ] Organization profile templates
- [ ] Audit logging
- [ ] Environment variable expansion
- [ ] CI/CD integrations (GitHub Actions, GitLab)

### Post-MVP Phase 3: Ecosystem (3-4 months)
- [ ] MCP server configuration
- [ ] Profile registry (community sharing)
- [ ] VS Code extension
- [ ] Claude Desktop support
- [ ] Cost tracking

### Post-MVP Phase 4: Advanced (6+ months)
- [ ] Smart model routing
- [ ] Provider health monitoring
- [ ] Usage analytics
- [ ] Multi-tool support
- [ ] Plugin system

---

## Contributing to the Roadmap

We welcome community input on prioritization. Open an issue or discussion to:

1. **Vote on features**: React with 👍 on roadmap issues
2. **Propose new features**: Open a feature request with use case
3. **Contribute implementations**: PRs welcome for roadmap items
4. **Share use cases**: Help us understand real-world needs

---

## Versioning Philosophy

- **Major versions** (2.0, 3.0): Breaking changes, major new capabilities
- **Minor versions** (1.1, 1.2): New features, backward compatible
- **Patch versions** (1.0.1): Bug fixes, documentation

We commit to:
- No breaking changes in minor versions
- Migration guides for major versions
- LTS support for v1.x even after v2.0 release

---

*Roadmap Version: 1.0*  
*Last Updated: January 30, 2026*  
*Next Review: March 2026*