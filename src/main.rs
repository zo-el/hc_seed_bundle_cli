use clap::Parser;
use color_eyre::eyre::Result;

mod commands;
mod error;

use commands::create::Create;
use error::CliError;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Create a new seed bundle
    Create(Create),
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    color_eyre::install()?;

    let cli = Cli::parse();
    
    match cli.command {
        Commands::Create(cmd) => cmd.execute().await?,
    }

    Ok(())
}
