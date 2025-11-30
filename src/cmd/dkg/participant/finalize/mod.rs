pub mod respond;

use anyhow::Result;
use clap::Args;

/// DKG finalize (participant).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    inner: respond::CommandArgs,
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> { self.inner.exec() }
}
