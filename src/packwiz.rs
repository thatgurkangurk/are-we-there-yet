use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub struct PackToml {
    pub index: PackIndex,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PackIndex {
    pub file: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IndexToml {
    pub files: Vec<IndexFile>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IndexFile {
    pub file: String,
    #[serde(default)]
    pub metafile: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModToml {
    pub update: Option<ModUpdate>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModUpdate {
    pub modrinth: Option<ModrinthUpdate>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModrinthUpdate {
    #[serde(rename = "mod-id")]
    pub mod_id: String,
}

/// fetches and parses the pack.toml
pub async fn fetch_pack(client: &Client, url: &Url) -> Result<PackToml> {
    let text = client.get(url.clone()).send().await?.text().await?;
    toml::from_str(&text).context("Failed to parse pack.toml")
}

/// fetches and parses the index.toml using the base pack URL and the index file path
pub async fn fetch_index(
    client: &Client,
    base_url: &Url,
    index_path: &str,
) -> Result<(Url, IndexToml)> {
    let index_url = base_url
        .join(index_path)
        .context("Failed to resolve index URL")?;
    let text = client.get(index_url.clone()).send().await?.text().await?;
    let index = toml::from_str(&text).context("Failed to parse index.toml")?;

    Ok((index_url, index))
}

/// extracts all valid urls for files marked as `metafile = true` in the index
pub fn get_metafile_urls(index_url: &Url, index: &IndexToml) -> Vec<Url> {
    index
        .files
        .iter()
        .filter(|f| f.metafile)
        .filter_map(|f| index_url.join(&f.file).ok())
        .collect()
}

/// fetches a mod.toml metafile and attempts to extract the modrinth mod id.
/// returns `Ok(None)` if the file is valid but doesn't contain a modrinth mod id (github/file url/curseforge).
pub async fn fetch_modrinth_id(client: &Client, url: Url) -> Result<Option<String>> {
    let response = client.get(url).send().await?.text().await?;
    let mod_toml: ModToml = toml::from_str(&response).context("Failed to parse mod metafile")?;

    let mod_id = mod_toml.update.and_then(|u| u.modrinth).map(|m| m.mod_id);

    Ok(mod_id)
}
