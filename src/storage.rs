use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedStatus {
    pub text: String,
    pub emoji: String,
}

pub type StatusMap = HashMap<String, SavedStatus>;

fn statuses_path() -> Result<PathBuf> {
    Ok(config::config_dir()?.join("statuses.json"))
}

pub fn load_statuses() -> Result<StatusMap> {
    let path = statuses_path()?;

    if !path.exists() {
        return Ok(HashMap::new());
    }

    let contents = fs::read_to_string(&path)?;
    let statuses: StatusMap = serde_json::from_str(&contents)
        .context("Failed to parse statuses.json")?;

    Ok(statuses)
}

pub fn save_statuses(statuses: &StatusMap) -> Result<()> {
    let path = statuses_path()?;
    let contents = serde_json::to_string_pretty(statuses)?;
    fs::write(&path, contents)?;
    Ok(())
}
