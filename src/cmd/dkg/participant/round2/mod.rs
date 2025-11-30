pub mod respond;

use anyhow::Result;
use clap::Args;

/// DKG Round 2 operations (participant).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    args: respond::CommandArgs,
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> { self.args.exec() }
}
