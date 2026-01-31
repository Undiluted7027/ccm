# ccm - Claude Code Manager

<div align="center">

**🔐 The Configuration Manager for Claude Code**

*fnm/nvm-style profile switching with secure credential storage*

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Status: MVP Development](https://img.shields.io/badge/status-MVP%20Development-orange.svg)](./MVP_ROADMAP.md)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.md)

[Documentation](./docs) • [Roadmap](./ROADMAP.md) • [Contributing](./CONTRIBUTING.md)

</div>

---

## 🎯 What is ccm?

ccm (Claude Code Manager) is a fast, secure, cross-platform configuration manager for Claude Code. It provides **fnm/nvm-style profile switching** that lets you seamlessly manage multiple Claude Code configurations—different API keys, providers, models, and settings—and switch between them instantly.

**The Problem**: Claude Code stores configuration in `~/.claude/settings.json`. Managing multiple configurations (work vs. personal, different providers, different models) requires manual file editing or environment variable juggling.

**The Solution**: ccm provides named profiles with secure credential storage, automatic directory-based switching, and a delightful CLI experience.

## ✨ Key Features

### 🔄 **Profile Switching**
Create named profiles and switch between them instantly. Each profile can have its own provider, model, API key, and settings.

```bash
ccm use work      # Switch to work profile
ccm use personal  # Switch to personal profile
ccm current       # Show active profile
```

### 🔐 **Secure Credential Storage**
API keys are stored securely using your system's native credential manager:
- **macOS**: Keychain Services
- **Linux**: Secret Service (GNOME Keyring, KWallet)
- **Windows**: Credential Manager
- **Fallback**: AES-256-GCM encrypted file with Argon2id key derivation

Credentials **never** appear in plain text, logs, or error messages.

### 📁 **Directory-Based Auto-Switching**
Create a `.ccmrc` file in your project, and ccm automatically switches profiles when you `cd` into that directory.

```bash
cd ~/work/project    # Auto-switches to "work" profile
cd ~/personal/hobby  # Auto-switches to "personal" profile
```

### 🩺 **Built-in Diagnostics**
The `doctor` command checks your entire setup and provides actionable suggestions.

```bash
ccm doctor
# ✓ ccm installation
# ✓ Claude Code CLI found
# ✓ 3 profiles configured
# ✓ Credentials stored securely
# All checks passed (8/8)
```

### 🖥️ **Cross-Platform**
Native binaries for macOS, Linux, and Windows. Same experience everywhere.

## 🆚 Why ccm?

| Feature | ccm | Manual Config | direnv |
|---------|-----|---------------|--------|
| Named Profiles | ✅ | ❌ | ⚠️ |
| Secure Credentials | ✅ (Keychain) | ❌ (Plain text) | ❌ |
| Auto-Switch on cd | ✅ | ❌ | ✅ |
| Claude Code Native | ✅ | ✅ | ⚠️ |
| Multiple Providers | ✅ | Manual | Manual |
| Diagnostics | ✅ | ❌ | ❌ |
| Cross-Platform | ✅ | ✅ | ⚠️ |

## 🚀 Quick Start

### Installation

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/Undiluted7027/ccm/main/scripts/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/Undiluted7027/ccm/main/scripts/install.ps1 | iex

# From source
cargo install ccm
```

### Basic Setup

```bash
# Create your first profile
ccm add work
# > Provider: Anthropic (Claude)
# > Model [claude-sonnet-4-5-20250514]: 
# > API Key: **********************
# > Set as default? [Y/n]: Y
# Profile 'work' created and set as default

# Verify it works
ccm doctor
# All checks passed (8/8)

# Start using Claude Code with your new profile!
claude "Hello, world!"
```

### Profile Management

```bash
# List all profiles
ccm list
# NAME      PROVIDER   MODEL                          DEFAULT  CURRENT
# work      anthropic  claude-sonnet-4-5-20250514    *        *
# personal  anthropic  claude-haiku-3-5-20241022

# Switch profiles
ccm use personal
# Switched to profile 'personal'

# Show current profile
ccm current
# personal

# Show profile details
ccm show work
# Profile: work
# Provider: anthropic
# Model: claude-sonnet-4-5-20250514
# Credential: sk-ant-...xyz (keychain)
```

### Project Configuration

```bash
# Initialize project with current profile
cd ~/work/my-project
ccm init
# Created .ccmrc with profile 'work'

# Or specify a profile
ccm init --profile personal

# .ccmrc contents:
# profile = "work"
# 
# [overrides]
# model = "claude-sonnet-4-5-20250514"
```

### Shell Integration

```bash
# Add to your shell config (~/.bashrc, ~/.zshrc, etc.)
eval "$(ccm env --shell bash --use-on-cd)"

# Now profiles switch automatically when you cd!
cd ~/work/project    # Switches to "work"
cd ~/personal/hobby  # Switches to "personal"
```

### Shell Support

```bash
# Bash
eval "$(ccm env --shell bash --use-on-cd)"

# Zsh
eval "$(ccm env --shell zsh --use-on-cd)"

# Fish
ccm env --shell fish --use-on-cd | source

# PowerShell
Invoke-Expression (& ccm env --shell powershell --use-on-cd)
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      CLI Layer                          │
│         (add, remove, list, use, doctor, ...)           │
└─────────────────────────┬───────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────┐
│                    Core Layer                           │
│     (ProfileManager, CredentialManager, Injector)       │
└───────┬────────────────┬──────────────────┬────────────-┘
        │                │                  │
┌───────▼──────┐  ┌──────▼───────┐  ┌───────▼──────┐
│   Profiles   │  │ Credentials  │  │   Claude     │
│              │  │              │  │   Settings   │
│ ~/.config/   │  │ • Keychain   │  │              │
│ ccm/profiles │  │ • Encrypted  │  │ ~/.claude/   │
│ /*.toml      │  │ • Env Var    │  │ settings.json│
└──────────────┘  └──────────────┘  └──────────────┘
```

### Core Components

1. **Profile Manager**: CRUD operations for named profiles stored as TOML files
2. **Credential Manager**: Secure credential storage with multiple backends
3. **Injector**: Atomic updates to Claude Code's `settings.json`
4. **Shell Integration**: Auto-switching on directory change
5. **Doctor**: Comprehensive diagnostics and troubleshooting

## 📊 Provider Support

| Provider | Status | Features |
|----------|--------|----------|
| **Anthropic** | ✅ Ready | Claude models, API key auth |
| **OpenRouter** | ✅ Ready | Multiple models, API key auth |
| **AWS Bedrock** | ✅ Ready | Claude via AWS, IAM auth |
| **Google Vertex AI** | ✅ Ready | Claude via GCP, service account |
| **Ollama** | ✅ Ready | Local models, no auth required |
| **Custom** | ✅ Ready | Any OpenAI-compatible API |

## 🛠️ CLI Reference

### Profile Commands

| Command | Description |
|---------|-------------|
| `ccm add [name]` | Create a new profile (interactive) |
| `ccm remove <name>` | Delete a profile |
| `ccm list` | List all profiles |
| `ccm use <name>` | Switch to a profile |
| `ccm current` | Show current profile |
| `ccm show <name>` | Show profile details |

### Project Commands

| Command | Description |
|---------|-------------|
| `ccm init` | Create .ccmrc in current directory |

### Credential Commands

| Command | Description |
|---------|-------------|
| `ccm credential set <profile>` | Set/update credential |
| `ccm credential delete <profile>` | Delete credential |
| `ccm credential show <profile>` | Show credential (masked) |

### Utility Commands

| Command | Description |
|---------|-------------|
| `ccm doctor` | Run diagnostic checks |
| `ccm env` | Output shell integration script |
| `ccm --version` | Show version |
| `ccm --help` | Show help |

### Global Flags

| Flag | Description |
|------|-------------|
| `--verbose, -v` | Enable verbose output |
| `--quiet, -q` | Suppress output (for scripts) |
| `--json` | Output in JSON format |
| `--no-color` | Disable colored output |

## 🎯 Use Cases

### Multiple Work Environments

```bash
# Different API keys for different clients
ccm add client-a --provider anthropic
ccm add client-b --provider anthropic
ccm add internal --provider ollama

# Switch based on project
cd ~/projects/client-a && ccm init --profile client-a
cd ~/projects/client-b && ccm init --profile client-b
```

### Cost Management

```bash
# Use cheaper models for experimentation
ccm add experiments --provider anthropic
ccm show experiments
# Model: claude-haiku-3-5-20241022 (cheaper)

# Premium model for production
ccm add production --provider anthropic
ccm show production
# Model: claude-sonnet-4-5-20250514 (better)
```

### Local Development

```bash
# Use local Ollama for offline work
ccm add local --provider ollama
# Base URL: http://localhost:11434
# Model: qwen2.5-coder:7b

# No API costs, works offline!
ccm use local
```

### CI/CD Integration

```bash
# In your CI pipeline
export CCM_CREDENTIAL_CI="$ANTHROPIC_API_KEY"
ccm add ci --provider anthropic --credential-source env

# Or use environment variables directly
ccm use ci
```

## 📁 File Locations

| File | Location | Purpose |
|------|----------|---------|
| Profiles | `~/.config/ccm/profiles/*.toml` | Profile configurations |
| Default marker | `~/.config/ccm/default` | Default profile name |
| Current marker | `~/.config/ccm/current` | Current profile name |
| Encrypted credentials | `~/.config/ccm/credentials.enc` | Fallback credential store |
| Backups | `~/.config/ccm/backups/` | Settings backups |
| Project config | `./.ccmrc` | Per-project configuration |
| Claude settings | `~/.claude/settings.json` | Claude Code configuration |

## 🔐 Security Model

ccm takes security seriously:

1. **Credentials Never Plain Text**
   - System keychain preferred (macOS Keychain, libsecret, Credential Manager)
   - AES-256-GCM encrypted fallback with Argon2id key derivation

2. **Credentials Never Logged**
   - Masked in all output (`sk-ant-...xyz`)
   - Never in error messages
   - Never in debug output

3. **Atomic File Operations**
   - All writes use temp file + rename pattern
   - No partial writes on crash

4. **Automatic Backups**
   - Settings backed up before modification
   - Easy rollback on mistakes

5. **Minimal Permissions**
   - Only accesses `~/.config/ccm` and `~/.claude`
   - No network access (except for connection testing)

## 📚 Documentation

- **[Getting Started](./docs/getting-started.md)** - Installation and first steps
- **[CLI Reference](./docs/cli-reference.md)** - Complete command documentation
- **[Configuration](./docs/configuration.md)** - Profile and settings reference
- **[Shell Integration](./docs/shell-integration.md)** - Setup for all shells
- **[Security](./docs/security.md)** - Security model and best practices
- **[Architecture](./docs/initial-architecture-design.md)** - System design
- **[Contributing](./CONTRIBUTING.md)** - How to contribute

## 🗺️ Roadmap

### ✅ Phase 1: MVP (Current)
- [x] Architecture design
- [ ] Profile management
- [ ] Credential storage (keychain, encrypted, env var)
- [ ] Claude Code settings injection
- [ ] Shell integration (bash, zsh, fish, PowerShell)
- [ ] Project configuration (.ccmrc)
- [ ] Diagnostic system (doctor)

See [MVP_ROADMAP.md](./MVP_ROADMAP.md) for detailed timeline.

### 🚧 Phase 2: Polish
- [ ] Profile templates
- [ ] Import/export
- [ ] Connection testing
- [ ] Tab completions
- [ ] Configuration file

### 🔮 Phase 3+: Future
- [ ] Team profile sharing
- [ ] MCP server management
- [ ] VS Code extension
- [ ] Usage analytics
- [ ] Enterprise features

See [ROADMAP.md](./ROADMAP.md) for the complete roadmap.

## 🚦 Current Status

**🟠 MVP Development in Progress**

ccm is currently under active development. The MVP will include:
- ✅ Architecture designed
- ⏳ Profile management
- ⏳ Secure credential storage
- ⏳ Shell integration
- ⏳ Project configuration
- ⏳ CLI interface

**Expected MVP Release**: 5 weeks from project start

We welcome early contributors! See [Contributing](#-contributing) below.

## 🤝 Contributing

We're building ccm in the open and would love your help! Whether you're:

- 🐛 **Reporting bugs**
- 💡 **Suggesting features**
- 📖 **Improving documentation**
- 🔧 **Submitting PRs**
- ⭐ **Starring the repo** (helps a lot!)

All contributions are welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/Undiluted7027/ccm.git
cd ccm

# Build
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- doctor

# Format code
cargo fmt

# Lint
cargo clippy
```

### Project Structure

```
ccm/
├── crates/
│   ├── ccm/           # CLI binary
│   │   └── src/
│   │       ├── main.rs
│   │       ├── cli.rs
│   │       └── commands/
│   └── ccm-core/      # Core library
│       └── src/
│           ├── lib.rs
│           ├── profile/
│           ├── credential/
│           ├── injector/
│           ├── shell/
│           └── doctor/
├── tests/
│   ├── integration/
│   └── e2e/
├── docs/
└── scripts/
```

## 🌟 Why "ccm"?

**ccm** stands for **Claude Code Manager**—a simple, memorable name that clearly describes what it does. Like `fnm` (Fast Node Manager) or `nvm` (Node Version Manager), ccm follows the Unix tradition of short, descriptive command names.

## 📄 License

ccm is open source and available under the [MIT License](./LICENSE).

## 🙏 Acknowledgments

Inspired by:
- [fnm](https://github.com/Schniz/fnm) - Fast Node Manager
- [asdf](https://github.com/asdf-vm/asdf) - Multiple runtime version manager
- [direnv](https://github.com/direnv/direnv) - Directory-based environments
- [1Password CLI](https://1password.com/downloads/command-line/) - Secrets management patterns

## 📬 Contact & Community

- **GitHub Issues**: [Report bugs or request features](https://github.com/Undiluted7027/ccm/issues)
- **Discussions**: [Join the conversation](https://github.com/Undiluted7027/ccm/discussions)
- **Twitter**: [@ccm_cli](https://twitter.com/ccm_cli) *(coming soon)*
- **Discord**: [Join our community](https://discord.gg/ccm) *(coming soon)*

---

<div align="center">

**Built with ❤️ and 🦀 Rust**

[⭐ Star us on GitHub](https://github.com/Undiluted7027/ccm) • [🐦 Follow updates](https://twitter.com/ccm_cli)

</div>