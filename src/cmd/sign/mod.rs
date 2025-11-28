pub mod collect;
pub mod commit;
pub mod receive;
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
    /// Inspect a signCommit request (participant)
    Receive(receive::CommandArgs),
    /// Respond to a signCommit request (participant)
    Commit(commit::CommandArgs),
    /// Collect signCommit responses and send signShare requests (coordinator)
    Collect(collect::CommandArgs),
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Start(args) => args.exec(),
            Commands::Receive(args) => args.exec(),
            Commands::Commit(args) => args.exec(),
            Commands::Collect(args) => args.exec(),
        }
    }
}
