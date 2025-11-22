#![allow(dead_code)]
use bc_components::ARID;

use crate::cmd::dkg::DkgParticipantStatus;

pub struct DkgParticipantInfo {
    response_arid: ARID, // ARID of the participant's DKG response
    status: DkgParticipantStatus, // Status of the participant in the DKG
}
