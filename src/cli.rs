use std::{collections::HashSet, path::PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use url::Url;

use crate::{fs::save_mod_statuses, modrinth, packwiz, update};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Check {
        #[arg(long)]
        mod_list: PathBuf,

        #[arg(long)]
        version: String,

        #[arg(long)]
        out: PathBuf,

        #[arg(long, action = clap::ArgAction::SetTrue)]
        percentage: bool,
    },
    Packwiz {
        #[arg()]
        url: Url,

        #[arg(long)]
        version: String,

        #[arg(long)]
        out: PathBuf,

        #[arg(long, action = clap::ArgAction::SetTrue)]
        percentage: bool,
    },
    Update,
}

impl Commands {
    pub async fn execute(self) -> Result<()> {
        if let Commands::Update = self {
            update::update()?;
            return Ok(());
        }

        match self {
            Commands::Check {
                mod_list,
                version,
                out,
                percentage,
            } => {
                let config = crate::fs::read_toml_file(mod_list).await?;

                let mods = config.mods.modrinth.project_ids;

                let ferinth = modrinth::create_ferinth();

                let results = crate::version::are_on_version(&ferinth, mods, &version).await?;

                let changed = save_mod_statuses(&results, &out, percentage).await?;

                println!("done! check {}", &out.display());

                if !changed {
                    println!("[note] no update since last check")
                }
            }
            Commands::Packwiz {
                url,
                version,
                out,
                percentage,
            } => {
                let client = Client::new();
                let pack = packwiz::fetch_pack(&client, &url).await?;

                let (index_url, index) =
                    packwiz::fetch_index(&client, &url, &pack.index.file).await?;

                let metafile_urls = packwiz::get_metafile_urls(&index_url, &index);
                let total_files = metafile_urls.len() as u64;

                let pb = ProgressBar::new(total_files);
                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                        .expect("Invalid progress bar template")
                        .progress_chars("#>-")
                );

                let mut stream = futures::stream::iter(metafile_urls)
                    .map(|url| {
                        let client_clone = client.clone();

                        let filename = url
                            .path_segments()
                            .and_then(|segments| segments.last())
                            .unwrap_or("unknown.toml")
                            .to_string();

                        tokio::spawn(async move {
                            let res = packwiz::fetch_modrinth_id(&client_clone, url).await;
                            (filename, res)
                        })
                    })
                    .buffer_unordered(4);

                let mut modrinth_ids = HashSet::new();

                while let Some(result) = stream.next().await {
                    if let Ok((filename, fetch_result)) = result {
                        pb.set_message(format!("reading {}...", filename));
                        pb.inc(1);

                        if let Ok(Some(id)) = fetch_result {
                            modrinth_ids.insert(id);
                        }
                    }
                }

                pb.finish_with_message("done!");

                let ferinth = modrinth::create_ferinth();

                let results = crate::version::are_on_version(
                    &ferinth,
                    modrinth_ids.into_iter().collect(),
                    &version,
                )
                .await?;

                let changed = save_mod_statuses(&results, &out, percentage).await?;

                println!("done! check {}", &out.display());

                if !changed {
                    println!("[note] no update since last check")
                }
            }
            Commands::Update => unreachable!(), // already handled
        }

        Ok(())
    }
}
