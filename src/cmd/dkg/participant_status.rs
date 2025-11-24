use anyhow::{Error, Result, bail};
use bc_envelope::prelude::*;

pub enum DkgParticipantStatus {
    Pending,  // Participant has been invited but has not yet responded
    Accepted, // Participant has accepted the invite
    Declined, // Participant has declined the invite
}

impl From<DkgParticipantStatus> for String {
    fn from(status: DkgParticipantStatus) -> Self {
        match status {
            DkgParticipantStatus::Pending => "pending".to_string(),
            DkgParticipantStatus::Accepted => "accepted".to_string(),
            DkgParticipantStatus::Declined => "declined".to_string(),
        }
    }
}

impl TryFrom<String> for DkgParticipantStatus {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "pending" => Ok(DkgParticipantStatus::Pending),
            "accepted" => Ok(DkgParticipantStatus::Accepted),
            "declined" => Ok(DkgParticipantStatus::Declined),
            _ => bail!("Invalid DkgParticipantStatus string: {}", value),
        }
    }
}

impl From<DkgParticipantStatus> for Envelope {
    fn from(status: DkgParticipantStatus) -> Self {
        Envelope::new(String::from(status))
    }
}

impl TryFrom<Envelope> for DkgParticipantStatus {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        let str: String = envelope.subject().try_into()?;
        DkgParticipantStatus::try_from(str)
            .map_err(|e| bc_envelope::Error::msg(e.to_string()))
    }
}
