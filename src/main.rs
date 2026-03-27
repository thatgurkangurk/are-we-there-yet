mod cli;
mod fs;
mod modrinth;
mod update;
mod version;

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
