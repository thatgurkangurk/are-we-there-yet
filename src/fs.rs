use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::{fs, fs::File, io::AsyncWriteExt};

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

pub async fn save_mod_statuses(
    results: &HashMap<String, bool>,
    out_path: &PathBuf,
    percentage: bool,
) -> Result<bool> {
    let mut mods: Vec<_> = results.keys().collect();
    mods.sort();

    let mut new_content = String::new();
    let mut enabled_count = 0;

    for mod_name in &mods {
        let status = results.get(*mod_name).unwrap_or(&false);
        if *status {
            enabled_count += 1;
        }
        let mark = if *status { "✅" } else { "❌" };
        new_content.push_str(&format!("{} - {}\n", mod_name, mark));
    }

    let total = mods.len();
    if total > 0 {
        if percentage {
            let percent = (enabled_count * 100) / total;
            new_content.push_str(&format!(
                "\ntotal: {}/{} ({}%)\n",
                enabled_count, total, percent
            ));
        } else {
            new_content.push_str(&format!("\ntotal: {}/{}\n", enabled_count, total));
        }
    }

    if out_path.exists() {
        if let Ok(existing_bytes) = fs::read(out_path).await {
            if existing_bytes == new_content.as_bytes() {
                return Ok(false);
            }
        }
    }

    let mut file = File::create(out_path).await?;
    file.write_all(new_content.as_bytes()).await?;

    Ok(true)
}
