pub mod attach;
pub mod collect;
pub mod commit;
pub mod finalize;
pub mod receive;
pub mod share;
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
    /// Respond to a signShare request with a signature share (participant)
    Share(share::CommandArgs),
    /// Collect signature shares and fan out finalize packages (coordinator)
    Finalize(finalize::CommandArgs),
    /// Attach a finalized signature to the target envelope (participant)
    Attach(attach::CommandArgs),
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Start(args) => args.exec(),
            Commands::Receive(args) => args.exec(),
            Commands::Commit(args) => args.exec(),
            Commands::Collect(args) => args.exec(),
            Commands::Share(args) => args.exec(),
            Commands::Finalize(args) => args.exec(),
            Commands::Attach(args) => args.exec(),
        }
    }
}
