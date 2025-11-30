pub mod finalize;
pub mod receive;
pub mod round1;
pub mod round2;

use anyhow::Result;
use clap::{Args, Subcommand};

/// Participant-only DKG commands.
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
    /// Respond to a DKG invite (Round 1)
    Round1(round1::CommandArgs),
    /// DKG Round 2 operations
    Round2(round2::CommandArgs),
    /// DKG finalize operations
    Finalize(finalize::CommandArgs),
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Receive(args) => args.exec(),
            Commands::Round1(args) => args.exec(),
            Commands::Round2(args) => args.exec(),
            Commands::Finalize(args) => args.exec(),
        }
    }
}
