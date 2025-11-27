pub mod start;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Threshold signing operations.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start a threshold signing session (coordinator only)
    Start(start::CommandArgs),
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Start(args) => args.exec(),
        }
    }
}
