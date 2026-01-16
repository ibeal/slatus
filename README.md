# slatus

A CLI tool for managing and quickly setting Slack statuses. Save your frequently used statuses and apply them with a single command.

## Installation

### Prerequisites

- [Rust toolchain](https://rustup.rs/)
- macOS: Xcode Command Line Tools (`xcode-select --install`)

### Building

```bash
cargo build --release

# Install to your PATH
cp target/release/slatus ~/.local/bin/
# or
cargo install --path .
```

### Shell Completions

Generate completions for your shell:

```bash
# Bash
slatus completions bash > ~/.local/share/bash-completion/completions/slatus

# Zsh
slatus completions zsh > ~/.zfunc/_slatus

# Fish
slatus completions fish > ~/.config/fish/completions/slatus.fish

# Nushell
slatus completions nushell | save ~/.config/nushell/completions/slatus.nu
# Then add to config.nu:
#   source ~/.config/nushell/completions/slatus.nu
#   use completions *

# PowerShell
slatus completions powershell > _slatus.ps1
```

## Setup

### Getting a Slack Token

1. Go to [api.slack.com/apps](https://api.slack.com/apps)
2. Create a new app (or select an existing one)
3. Add the `users.profile:write` OAuth scope under **User Token Scopes**
4. Install the app to your workspace
5. Copy the **User OAuth Token** (starts with `xoxp-`)

### Configure the Token

```bash
slatus config xoxp-your-token-here
```

## Usage

```
slatus <COMMAND>

Commands:
  list         List all saved statuses
  add          Add a new saved status
  remove       Remove a saved status
  set          Set your Slack status to a saved status
  clear        Clear your current Slack status
  current      Show your current Slack status
  config       Configure the Slack token
  completions  Generate shell completions
  help         Print help for a command
```

### Examples

```bash
# Add some statuses
slatus add meeting "In a meeting" ":calendar:"
slatus add lunch "Out for lunch" ":hamburger:"
slatus add focus "Focus time - minimal interruptions" ":headphones:"
slatus add vacation "On vacation" ":palm_tree:"

# List saved statuses
slatus list

# Set a status
slatus set meeting

# Set a status with expiration (in minutes)
slatus set lunch --expires 60

# Check current status
slatus current

# Clear status
slatus clear

# Remove a saved status
slatus remove vacation
```

## Data Storage

Configuration and data are stored together in a platform-specific location:

| Platform | Location |
|----------|----------|
| macOS | `~/Library/Application Support/com.slatus.slatus/` |
| Linux | `~/.config/slatus/` |

Files:
- `token` - Your Slack API token (permissions: 600)
- `statuses.json` - Your saved statuses

## Building with Nix

If you encounter linker errors on macOS (e.g., `library not found for -liconv`), you can use the provided `shell.nix`:

```bash
nix-shell --run "cargo build --release"
```

## Contributing

### Architecture

- **minreq** with native-tls for HTTP requests (chosen for minimal dependencies)
- **clap** for CLI argument parsing
- **directories** crate for platform-specific config paths
- Synchronous design (no async runtime needed)

### Project Structure

```
src/
├── main.rs      # CLI commands and entry point
├── config.rs    # Token storage
├── storage.rs   # Saved statuses storage
└── slack.rs     # Slack API client
```

## License

MIT
