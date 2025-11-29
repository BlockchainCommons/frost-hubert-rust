pub mod send;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Coordinator DKG invite operations.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Compose or send a DKG invite
    Send(send::CommandArgs),
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Send(args) => args.exec(),
        }
    }
}
