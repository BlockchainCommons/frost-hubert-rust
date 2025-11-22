use bc_components::ARID;
use bc_envelope::prelude::*;

pub struct DkgParticipantResponse {
    session: ARID, // Identifies the DKG session
    is_accepted: bool, // Indicates whether the participant accepted or declined the invite
}

impl DkgParticipantResponse {
    pub fn new(session: ARID, is_accepted: bool) -> Self {
        Self { session, is_accepted }
    }

    pub fn session(&self) -> ARID {
        self.session
    }

    pub fn is_accepted(&self) -> bool {
        self.is_accepted
    }
}

impl From<DkgParticipantResponse> for Envelope {
    fn from(response: DkgParticipantResponse) -> Self {
        Envelope::unit()
            .add_type("dkg-participant-response")
            .add_assertion("session", response.session)
            .add_assertion("is_accepted", response.is_accepted)
    }
}

impl TryFrom<Envelope> for DkgParticipantResponse {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type_envelope("dkg-participant-response")?;
        let session: ARID = envelope.try_object_for_predicate("session")?;
        let is_accepted: bool = envelope.try_object_for_predicate("is_accepted")?;
        Ok(DkgParticipantResponse { session, is_accepted })
    }
}
