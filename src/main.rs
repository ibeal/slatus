mod config;
mod slack;
mod storage;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell};
use clap_complete_nushell::Nushell;
use std::io;

#[derive(Parser)]
#[command(name = "slatus")]
#[command(about = "Manage and quickly set Slack statuses", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
enum ShellArg {
    Bash,
    Elvish,
    Fish,
    Nushell,
    Powershell,
    Zsh,
}

#[derive(Subcommand)]
enum Commands {
    /// List all saved statuses
    List,

    /// Add a new saved status
    Add {
        /// Name to identify this status
        name: String,
        /// Status text (e.g., "In a meeting")
        text: String,
        /// Status emoji (e.g., ":calendar:")
        emoji: String,
    },

    /// Remove a saved status
    Remove {
        /// Name of the status to remove
        name: String,
    },

    /// Set your Slack status to a saved status
    Set {
        /// Name of the saved status to set
        name: String,
        /// Optional expiration in minutes (0 = no expiration)
        #[arg(short, long, default_value = "0")]
        expires: u64,
    },

    /// Clear your current Slack status
    Clear,

    /// Show your current Slack status
    Current,

    /// Configure the Slack token
    Config {
        /// Your Slack user token (xoxp-...)
        token: String,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: ShellArg,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => cmd_list()?,
        Commands::Add { name, text, emoji } => cmd_add(&name, &text, &emoji)?,
        Commands::Remove { name } => cmd_remove(&name)?,
        Commands::Set { name, expires } => cmd_set(&name, expires)?,
        Commands::Clear => cmd_clear()?,
        Commands::Current => cmd_current()?,
        Commands::Config { token } => cmd_config(&token)?,
        Commands::Completions { shell } => cmd_completions(shell),
    }

    Ok(())
}

fn cmd_list() -> Result<()> {
    let statuses = storage::load_statuses()?;

    if statuses.is_empty() {
        println!("No saved statuses. Add one with: slatus add <name> <text> <emoji>");
        return Ok(());
    }

    println!("{:<15} {:<30} {}", "NAME", "TEXT", "EMOJI");
    println!("{}", "-".repeat(60));
    for (name, status) in &statuses {
        println!("{:<15} {:<30} {}", name, status.text, status.emoji);
    }

    Ok(())
}

fn cmd_add(name: &str, text: &str, emoji: &str) -> Result<()> {
    let mut statuses = storage::load_statuses()?;

    let status = storage::SavedStatus {
        text: text.to_string(),
        emoji: emoji.to_string(),
    };

    statuses.insert(name.to_string(), status);
    storage::save_statuses(&statuses)?;

    println!("Saved status '{}': {} {}", name, emoji, text);
    Ok(())
}

fn cmd_remove(name: &str) -> Result<()> {
    let mut statuses = storage::load_statuses()?;

    if statuses.remove(name).is_some() {
        storage::save_statuses(&statuses)?;
        println!("Removed status '{}'", name);
    } else {
        println!("Status '{}' not found", name);
    }

    Ok(())
}

fn cmd_set(name: &str, expires_minutes: u64) -> Result<()> {
    let token = config::load_token()?;
    let statuses = storage::load_statuses()?;

    let status = statuses
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("Status '{}' not found. Use 'list' to see saved statuses.", name))?;

    let expiration = if expires_minutes > 0 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        now + (expires_minutes * 60)
    } else {
        0
    };

    slack::set_status(&token, &status.text, &status.emoji, expiration)?;

    if expires_minutes > 0 {
        println!("Status set: {} {} (expires in {} min)", status.emoji, status.text, expires_minutes);
    } else {
        println!("Status set: {} {}", status.emoji, status.text);
    }

    Ok(())
}

fn cmd_clear() -> Result<()> {
    let token = config::load_token()?;
    slack::set_status(&token, "", "", 0)?;
    println!("Status cleared");
    Ok(())
}

fn cmd_current() -> Result<()> {
    let token = config::load_token()?;
    let (text, emoji) = slack::get_status(&token)?;

    if text.is_empty() && emoji.is_empty() {
        println!("No status currently set");
    } else {
        println!("Current status: {} {}", emoji, text);
    }

    Ok(())
}

fn cmd_config(token: &str) -> Result<()> {
    if !token.starts_with("xoxp-") {
        anyhow::bail!("Token should start with 'xoxp-' (user token)");
    }

    config::save_token(token)?;
    println!("Token saved");
    Ok(())
}

fn cmd_completions(shell: ShellArg) {
    let mut cmd = Cli::command();
    match shell {
        ShellArg::Bash => generate(Shell::Bash, &mut cmd, "slatus", &mut io::stdout()),
        ShellArg::Elvish => generate(Shell::Elvish, &mut cmd, "slatus", &mut io::stdout()),
        ShellArg::Fish => generate(Shell::Fish, &mut cmd, "slatus", &mut io::stdout()),
        ShellArg::Nushell => generate(Nushell, &mut cmd, "slatus", &mut io::stdout()),
        ShellArg::Powershell => generate(Shell::PowerShell, &mut cmd, "slatus", &mut io::stdout()),
        ShellArg::Zsh => generate(Shell::Zsh, &mut cmd, "slatus", &mut io::stdout()),
    }
}
