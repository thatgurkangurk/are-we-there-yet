mod fs;
mod cli;
mod update;
mod modrinth;

use clap::Parser;

use anyhow::Result;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        command.execute().await?;
    }

    Ok(())
}