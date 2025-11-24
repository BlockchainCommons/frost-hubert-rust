use anyhow::{Result, bail};
use clap::Parser;

use crate::{
    cmd::registry::participants_file_path,
    registry::{OwnerOutcome, OwnerRecord, Registry},
};

#[derive(Debug, Parser)]
#[doc(hidden)]
pub struct CommandArgs {
    /// Signed ur:xid document containing the owner's XID document (must
    /// include private keys)
    xid_document: String,
    /// Optional human readable alias for the owner
    pet_name: Option<String>,
    /// Optional registry path or filename override
    #[arg(long = "registry", value_name = "PATH")]
    registry: Option<String>,
}

impl CommandArgs {
    pub fn exec(self) -> Result<()> {
        let pet_name = normalize_pet_name(self.pet_name)?;
        let owner = OwnerRecord::from_signed_xid_ur(self.xid_document, pet_name)?;
        let path = participants_file_path(self.registry)?;
        let mut registry = Registry::load(&path)?;

        match registry.set_owner(owner)? {
            OwnerOutcome::AlreadyPresent => {
                println!("Owner already recorded");
            }
            OwnerOutcome::Inserted => {
                // println!("Owner stored in {}", path.display());
            }
        }

        // Always save to persist pet name updates on existing owner records.
        registry.save(&path)?;

        Ok(())
    }
}

fn normalize_pet_name(pet_name: Option<String>) -> Result<Option<String>> {
    match pet_name {
        None => Ok(None),
        Some(name) => {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                bail!("Pet name cannot be empty");
            }
            Ok(Some(trimmed.to_owned()))
        }
    }
}
