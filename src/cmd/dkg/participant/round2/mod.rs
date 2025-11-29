pub mod respond;

use anyhow::Result;
use clap::{Args, Subcommand};

/// DKG Round 2 operations (participant).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Respond to a Round 2 request (participant only)
    Respond(respond::CommandArgs),
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Respond(args) => args.exec(),
        }
    }
}
