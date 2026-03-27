use std::path::PathBuf;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashMap;

use crate::{fs::save_mod_statuses, modrinth, update};

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
            } => {
                let config = crate::fs::read_toml_file(mod_list).await?;

                let mods = config.mods.modrinth.project_ids;

                let ferinth = modrinth::create_ferinth();

                let results =
                    crate::version::check_mods_status_concurrent(mods, &ferinth, &version).await?;

                save_mod_statuses(&results, &out).await?;

                println!("done! check {}", &out.display())
            }
            Commands::Update => unreachable!(), // already handled
        }

        Ok(())
    }
}
