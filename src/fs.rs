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
) -> Result<()> {
    let mut file = File::create(out_path).await?;

    let mut mods: Vec<_> = results.keys().collect();
    mods.sort();

    let mut enabled_count = 0;

    for mod_name in &mods {
        let status = results.get(*mod_name).unwrap_or(&false);
        if *status {
            enabled_count += 1;
        }
        let mark = if *status { "✅" } else { "❌" };
        file.write_all(format!("{} - {}\n", mod_name, mark).as_bytes())
            .await?;
    }

    let total = mods.len();
    if total > 0 {
        let total_line = if percentage {
            let percent = (enabled_count * 100) / total;
            format!("\ntotal: {}/{} ({}%)\n", enabled_count, total, percent)
        } else {
            format!("\ntotal: {}/{}\n", enabled_count, total)
        };
        file.write_all(total_line.as_bytes()).await?;
    }

    Ok(())
}
