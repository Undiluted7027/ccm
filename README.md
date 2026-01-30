# ccm - Claude Code Manager

<div align="center">

**🚀 The `fnm` for Claude Code**

*Effortless, secure, and project-aware configuration management for the Claude Code CLI.*

[![Rust](https://img.shields.io/badge/rust-stable-blue.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Status: MVP Development](https://img.shields.io/badge/status-MVP%20Development-orange.svg)](./MVP_ROADMAP.md)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.md)
[![Platform Support](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#-cross-platform-native)

[Documentation](./docs) • [MVP Roadmap](./MVP_ROADMAP.md) • [Full Roadmap](./ROADMAP.md) • [Contributing](./CONTRIBUTING.md)

</div>

---

## 🎯 What is ccm?

ccm is an open-source, fast, and reliable command-line tool that eliminates the pain of managing configurations for the Claude Code CLI. Inspired by the simplicity of tools like `fnm` and `nvm`, ccm brings **profiles**, **secure credential storage**, and **automatic directory-based switching** to your AI development workflow.

**The Problem**: Manually editing `~/.claude/settings.json` is tedious and error-prone. Juggling API keys for different providers (Anthropic, OpenRouter, local Ollama) is insecure. Ensuring consistent settings across a team is nearly impossible.

**The Solution**: ccm provides a simple, powerful interface to manage distinct configurations ("profiles"). Switch between a local model for a private project and a powerful cloud model for another with a single command—or have it happen automatically as you `cd` between directories.

## ✨ Key Features

### 🔄 **Effortless Profile Switching**
Create, manage, and switch between named configurations instantly. Stop editing JSON and start working.

### 🔐 **Secure Credential Storage**
API keys are never stored in plain text. ccm uses your OS's native keychain (macOS Keychain, Windows Credential Manager, Linux libsecret) with an encrypted file fallback.

### 📁 **Project-Specific Configuration**
Drop a `.ccmrc` file in your project's root to lock in the AI provider and model for everyone on your team. It's like `.nvmrc`, but for Claude Code.

### ⚡ **Automatic Switching**
With shell integration, ccm automatically activates the correct profile as you navigate your filesystem. Blazing fast shell hook (<5ms overhead) won't slow you down.

### 🩺 **Built-in Diagnostics**
Troubleshoot your setup with `ccm doctor`. It checks your installation, Claude Code CLI, profile validity, and credential access with clear, actionable advice.

### 💻 **Cross-Platform Native**
A single, native Rust binary with no runtime dependencies. Works identically on macOS, Linux, and Windows.

## 🆚 Why ccm?

| Feature | ccm | Manual / Scripts | claudecode-switch |
|---------|:---:|:----------------:|:-----------------:|
| Instant Switching | ✅ `ccm use` | ❌ Edit JSON | ⚠️ Basic |
| Secure Credentials | ✅ System Keychain | ❌ Plain text | ❌ Plain text |
| Project Config | ✅ `.ccmrc` | ❌ Custom scripts | ❌ Not supported |
| Auto `cd` Hook | ✅ Set-and-forget | ❌ Complex | ❌ Not supported |
| Team Consistency | ✅ Commit to git | ❌ Documentation | ❌ Not supported |
| Error-Proof | ✅ Validation + doctor | ⚠️ Typo-prone | ⚠️ Basic |
| Cross-Platform | ✅ Native binary | ❌ Shell-specific | ⚠️ Bash only |

## 🚀 Quick Start

### Installation

```bash
# macOS, Linux, or WSL (recommended)
curl -fsSL https://ccm.dev/install | bash

# Homebrew (macOS/Linux)
brew install ccm

# Cargo (Rust toolchain)
cargo install ccm

# Scoop (Windows)
scoop install ccm
```

### Shell Setup (One-Time)

Add to your `~/.zshrc`, `~/.bashrc`, or `~/.config/fish/config.fish`:

```bash
eval "$(ccm env --use-on-cd)"
```

Restart your shell for the changes to take effect.

### CLI Usage

```bash
# Create a profile interactively
ccm add anthropic
# → Wizard prompts for API key, stores securely in keychain

# Create a profile for local Ollama
ccm add local --base-url http://localhost:11434 --model llama3

# List all profiles
ccm list
#   anthropic   https://api.anthropic.com  (★ default)
# → local       http://localhost:11434

# Switch profiles
ccm use anthropic
# → Claude Code now uses Anthropic API

ccm use local
# → Claude Code now uses local Ollama

# Check current profile
ccm current
# → local

# Set up per-project configuration
cd ~/work/sensitive-project
ccm init --profile=local
# → Creates .ccmrc, auto-switches when you cd here

# Troubleshoot issues
ccm doctor
# → Checks installation, profiles, credentials, shell integration
```

### Project Configuration

Create a `.ccmrc` file in your project root:

```toml
profile = "local"

[override]
model = "deepseek-coder:33b"
timeout_ms = 180000
```

Now everyone on your team uses the same configuration. Commit it to git!

### Non-Interactive (CI/CD)

```bash
# Create profile from environment variables
ccm add ci-runner \
  --base-url https://api.anthropic.com \
  --auth-token-env ANTHROPIC_API_KEY \
  --model claude-haiku-4 \
  --non-interactive

ccm use ci-runner
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      CLI Layer                          │
│           (ccm add, use, list, doctor, etc.)            │
└─────────────────────┬───────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────┐
│                    ccm Core                             │
│         (Profile resolution & injection)                │
└─────┬──────────────────┬──────────────────┬─────────────┘
      │                  │                  │
┌─────▼────────┐  ┌─────▼────────┐  ┌─────▼────────┐
│   Profiles   │  │ Credentials  │  │   Project    │
│              │  │              │  │   Config     │
│ • TOML files │  │ • Keychain   │  │ • .ccmrc     │
│ • Validation │  │ • Encrypted  │  │ • Resolver   │
│ • Defaults   │  │ • Env vars   │  │ • Overrides  │
└──────────────┘  └──────────────┘  └──────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│              Claude Code Integration                    │
│         (~/.claude/settings.json injection)             │
└─────────────────────────────────────────────────────────┘
```

### Core Components

1. **Profile Manager**: CRUD operations on named configurations stored as TOML
2. **Credential Manager**: Secure storage via system keychain with encrypted fallback
3. **Config Injector**: Atomic writes to Claude Code's `settings.json`
4. **Project Resolver**: Recursive `.ccmrc` detection with override merging
5. **Shell Integration**: Fast hooks for auto-switching on `cd`
6. **Doctor**: Comprehensive diagnostics with actionable suggestions

## 📊 Provider Support

ccm works with any provider that Claude Code supports:

| Provider | Status | Use Case |
|----------|--------|----------|
| **Anthropic** | ✅ Ready | Official Claude API |
| **OpenRouter** | ✅ Ready | Multi-model gateway (200+ models) |
| **Ollama** | ✅ Ready | Local/private models |
| **AWS Bedrock** | ✅ Ready | Enterprise compliance |
| **Google Vertex AI** | ✅ Ready | GCP integration |
| **LiteLLM** | ✅ Ready | Self-hosted proxy |
| **Custom** | ✅ Ready | Any OpenAI-compatible endpoint |

## 🎯 Use Cases

### The Multi-Provider Developer

```bash
# Use Anthropic for complex tasks
ccm use anthropic
claude "Architect a microservices system"

# Switch to cheaper model for simple tasks
ccm use openrouter-haiku
claude "Fix this typo in the README"

# Use local model for sensitive code
ccm use local
claude "Refactor the auth module"
```

### The Cost-Conscious Team

```bash
# In expensive project directory
cd ~/work/critical-app
cat .ccmrc
# profile = "opus-prod"

# In learning/experimentation directory
cd ~/personal/experiments
cat .ccmrc
# profile = "haiku-cheap"

# Auto-switching ensures you never overspend
```

### The Privacy-First Developer

```bash
# Sensitive project uses local Ollama
cd ~/work/secret-project
ccm init --profile=local
# All AI requests stay on your machine

# Open source work uses cloud
cd ~/oss/public-project
ccm init --profile=anthropic
```

### The CI/CD Pipeline

```yaml
# .github/workflows/ai-review.yml
- name: Setup ccm
  run: |
    curl -fsSL https://ccm.dev/install | bash
    ccm add ci \
      --base-url https://api.anthropic.com \
      --auth-token-env ANTHROPIC_API_KEY \
      --model claude-haiku-4 \
      --non-interactive
    ccm use ci
    
- name: AI Code Review
  run: claude -p "Review the changes in this PR"
  env:
    ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
```

## 📚 Documentation

- **[Getting Started](./docs/getting-started.md)** — Installation and first steps
- **[Profiles Guide](./docs/guides/profiles.md)** — Creating and managing profiles
- **[Shell Integration](./docs/guides/shell-integration.md)** — Setup for bash, zsh, fish, PowerShell
- **[Project Configuration](./docs/guides/project-config.md)** — Using `.ccmrc` files
- **[CI/CD Guide](./docs/guides/ci-cd.md)** — Non-interactive setup for pipelines
- **[CLI Reference](./docs/reference/cli.md)** — Complete command documentation
- **[Troubleshooting](./docs/reference/troubleshooting.md)** — Common issues and solutions

## 🗺️ Roadmap

### ✅ Phase 1: MVP (Current)

Core functionality for a robust single-user experience.

- [x] Project architecture and design
- [ ] Profile management (`add`, `use`, `list`, `remove`, `show`)
- [ ] Secure credential storage (keychain + encrypted fallback)
- [ ] `.ccmrc` project configuration with overrides
- [ ] Shell integration (`bash`, `zsh`, `fish`, `PowerShell`)
- [ ] `ccm doctor` diagnostics
- [ ] Cross-platform support (macOS, Linux, Windows)

See [MVP_ROADMAP.md](./MVP_ROADMAP.md) for detailed timeline and specifications.

### 🚧 Phase 2: Polish & Teams

- [ ] Profile templates/presets (`--preset=ollama`)
- [ ] Connection testing (`ccm test <profile>`)
- [ ] Profile inheritance (`extends = "base"`)
- [ ] Profile export/import for team sharing
- [ ] CI/CD integrations (GitHub Actions, GitLab CI)
- [ ] Audit logging for compliance

### 🔮 Phase 3: Ecosystem

- [ ] MCP server configuration management
- [ ] Community profile registry
- [ ] VS Code extension
- [ ] Claude Desktop support
- [ ] Cost tracking per profile
- [ ] Smart routing (auto-select model by task complexity)

See [ROADMAP.md](./ROADMAP.md) for our complete long-term vision.

## 🚦 Current Status

**🟠 MVP Development in Progress**

ccm is under active development. The MVP will include:

- ✅ Architecture and design complete
- ⏳ Profile management system
- ⏳ Secure credential storage
- ⏳ Shell integration with auto-switching
- ⏳ Project configuration (`.ccmrc`)
- ⏳ Doctor command

**Expected MVP Release**: ~5 weeks

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

# Format and lint
cargo fmt
cargo clippy
```

### Project Structure

```
ccm/
├── crates/
│   ├── ccm/           # CLI binary
│   └── ccm-core/      # Core library
├── tests/             # Integration tests
├── docs/              # Documentation
└── scripts/           # Install scripts
```

## 🌟 Why "ccm"?

**ccm** stands for **Claude Code Manager** — a simple, memorable name that follows the tradition of developer tools like `npm`, `nvm`, and `fnm`. It's short, easy to type, and immediately communicates what it does.

## 📄 License

ccm is open-source software licensed under the [MIT License](./LICENSE).

## 🙏 Acknowledgments

Inspired by:
- [fnm](https://github.com/Schniz/fnm) — For showing how a Rust-based version manager should feel
- [nvm](https://github.com/nvm-sh/nvm) — For pioneering the `.nvmrc` convention
- [Claude Code](https://docs.anthropic.com/claude-code) — For building an amazing AI coding assistant
- [claudecode-switch](https://github.com/frondesce/claudecode-switch) — For proving the need exists

## 📬 Contact & Community

- **GitHub Issues**: [Report bugs or request features](https://github.com/Undiluted7027/ccm/issues)

---

<div align="center">

**Built with ❤️ for developers who want choice**

[⭐ Star us on GitHub](https://github.com/Undiluted7027/ccm) • [📖 Read the Docs](./docs)

</div>