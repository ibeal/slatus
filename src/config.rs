use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub fn config_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "slatus", "slatus")
        .context("Could not determine config directory")?;

    let config_dir = proj_dirs.config_dir().to_path_buf();
    fs::create_dir_all(&config_dir)?;

    Ok(config_dir)
}

fn token_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("token"))
}

pub fn save_token(token: &str) -> Result<()> {
    let path = token_path()?;
    fs::write(&path, token)?;

    // Set restrictive permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}

pub fn load_token() -> Result<String> {
    let path = token_path()?;

    fs::read_to_string(&path)
        .map(|s| s.trim().to_string())
        .context("No token configured. Run: slatus config <token>")
}
