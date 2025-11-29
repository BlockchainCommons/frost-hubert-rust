pub mod receive;
pub mod respond;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Participant DKG invite operations.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Retrieve or inspect a DKG invite
    Receive(receive::CommandArgs),
    /// Respond to a DKG invite
    Respond(respond::CommandArgs),
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Receive(args) => args.exec(),
            Commands::Respond(args) => args.exec(),
        }
    }
}
