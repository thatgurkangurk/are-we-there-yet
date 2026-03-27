use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mods: Mods,
}

#[derive(Debug, Deserialize)]
pub struct Mods {
    pub modrinth: Modrinth,
}

#[derive(Debug, Deserialize)]
pub struct Modrinth {
    #[serde(rename = "project-ids")]
    pub project_ids: Vec<String>,
}

pub async fn read_toml_file(path: PathBuf) -> Result<Config> {
    let content = fs::read_to_string(&path)
        .await
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let config: Config = toml::from_str(&content)
        .with_context(|| format!("Failed to parse TOML: {}", path.display()))?;

    for id in &config.mods.modrinth.project_ids {
        if !crate::modrinth::is_valid_modrinth_slug(id) {
            anyhow::bail!("Invalid Modrinth ID: {}", id);
        }
    }

    Ok(config)
}
