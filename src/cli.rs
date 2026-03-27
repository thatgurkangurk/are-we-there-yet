use std::path::PathBuf;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

use crate::update;

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
            Commands::Check { mod_list } => {
                let config = crate::fs::read_toml_file(mod_list).await?;

                let mods = config.mods.modrinth.project_ids.join(", ");

                println!("{}", mods)
            }
            Commands::Update => unreachable!(), // already handled
        }

        Ok(())
    }
}
