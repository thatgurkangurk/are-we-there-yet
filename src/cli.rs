use std::path::PathBuf;

use clap::{Parser, Subcommand};
use anyhow::{Ok, Result};

use crate::update;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Check {
        #[arg(long)]
        mod_list: PathBuf
    },
    Update
}

impl Commands {
    pub async fn execute(self) -> Result<()> {
        if let Commands::Update = self {
            update::update()?;
            return Ok(());
        }

        Ok(())
    }
}