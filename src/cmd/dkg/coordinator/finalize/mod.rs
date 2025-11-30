pub mod collect;

use anyhow::Result;
use clap::Args;

/// DKG finalize response collection (coordinator).
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    inner: collect::CommandArgs,
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> { self.inner.exec() }
}
