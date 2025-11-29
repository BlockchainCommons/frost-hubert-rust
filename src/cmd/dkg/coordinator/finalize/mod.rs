pub mod collect;
pub mod send;

use anyhow::Result;
use clap::{Args, Subcommand};

/// DKG finalize operations (coordinator).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Send finalize packages to all participants (coordinator only)
    Send(send::CommandArgs),
    /// Collect finalize responses (coordinator only)
    Collect(collect::CommandArgs),
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Send(args) => args.exec(),
            Commands::Collect(args) => args.exec(),
        }
    }
}
