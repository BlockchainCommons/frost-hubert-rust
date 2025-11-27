# Priority Tasks for `frost` Crate

## Current State Summary

The `frost` CLI is a working tool for managing FROST (Flexible Round-Optimized Schnorr Threshold) participants. Current functionality:

1. **Registry Management** (`frost registry`)
   - Owner set with private XID documents
   - Participant add with signed public XID documents
   - Persistent JSON storage

2. **DKG Invite Flow** (`frost dkg invite`)
   - `send`: Coordinator creates sealed/preview invites for participants
   - `receive`: Participants fetch and decrypt invites from Hubert or local envelope
   - `respond`: Participants accept or reject, posting response to Hubert

3. **DKG Round 1** (`frost dkg round1`)
   - `collect`: Coordinator fetches all participant responses from Hubert, validates GSTP responses, extracts Round 1 packages, saves to `collected_round1.json`

4. **DKG Round 2** (`frost dkg round2`)
   - `send`: Coordinator sends individual sealed messages to each participant containing all Round 1 packages and their unique response ARID (posts to ARIDs participants specified in their invite responses)
   - `respond`: Participants respond with round2 packages, persist round2 secret, include next `response_arid`, and update `listening_at_arid`
   - `collect`: Coordinator fetches/validates Round 2 responses, saves `collected_round2.json`, and updates pending_requests for finalize phase

5. **DKG Finalize** (`frost dkg finalize`)
   - `send`: Coordinator distributes collected Round 2 packages to each participant (with new `responseArid` for finalize respond)
   - `respond`: Participants run part3, produce key/public key packages, persist them, and return finalize response

5. **Storage Backends**
   - Hubert server (HTTP)
   - Mainline DHT
   - IPFS
   - Hybrid (DHT + IPFS)

6. **Demo Script** (`frost-demo.py`)
   - Provisions 4 participants (Alice, Bob, Carol, Dan) in separate directories
   - Builds registries
   - Creates and responds to DKG invites via Hubert
   - Coordinator collects Round 1 packages
   - Coordinator sends Round 2 request

## Where the Demo Stops

The `demo-log.md` now runs through Round 2 send/respond/collect and finalize send. Each participant has:
- `registry.json` - Group membership, pending_requests (Round 2), updated `listening_at_arid` for finalize
- `group-state/<group-id>/round1_secret.json` - Round 1 secret (participants only)
- `group-state/<group-id>/round1_package.json` - Round 1 package (participants only)
- `group-state/<group-id>/collected_round1.json` - All Round 1 packages (coordinator only)
- `group-state/<group-id>/round2_secret.json` - Round 2 secret (participants only)
- `group-state/<group-id>/collected_round2.json` - Round 2 packages keyed by sender/recipient plus next `response_arid`
- Finalize requests sent and participants respond with key/public key packages

## Next Steps (Priority Order)

### 1. Coordinator Collects Finalize Responses

**Command (to implement):** `frost dkg finalize collect`

Coordinator:
- Fetches all finalize responses (key/public key packages) from Hubert
- Validates function/group/participants
- Aggregates/stores group verifying key (`SigningPublicKey::Ed25519`, UR form `ur:signing-public-key`; CBOR form `SigningPublicKey(...)`)
- Persists collected finalize data (e.g., `collected_finalize.json`) and updates registry

### 2. Participants Finalize Key Generation

**Command:** `frost dkg finalize respond`

Each participant:
- Fetches their incoming Round 2 packages
- Runs `frost_ed25519::keys::dkg::part3` with their Round 2 secret and incoming packages
- Produces `KeyPackage` and `PublicKeyPackage`
- Stores `key_package.json`
- Posts confirmation to coordinator
 - NOTE: final group verifying key should be a `SigningPublicKey::Ed25519` (`ur:signing-public-key` in UR form; CBOR encoded as `SigningPublicKey(...)` in envelopes). Ensure final outputs use the UR form in text (like JSON) and binary CBOR form in envelopes.

### 5. Group Status and Listing

**Commands:**
- `frost group list` - List all groups in registry with status
- `frost group info <GROUP_ID>` - Show group details, participants, coordinator, signing threshold

### 6. Threshold Signing

Once key generation is complete (group verifying key is `SigningPublicKey::Ed25519`, UR form `ur:signing-public-key`; CBOR form `SigningPublicKey(...)` in envelopes):
- `frost sign start` (coordinator)
  - Input: group ID, target envelope (assumed pre-wrapped as needed)
  - Derive target digest = digest(subject) and session ID (ARID)
  - Generate per-participant request ARIDs (where each participant will fetch the first message), plus a commitment response ARID and per-participant share ARIDs
  - Multicast pattern mirrors `dkg invite`: inside the request body, include a `participant` parameter per participant whose `response_arid` is individually encrypted to that participant; then wrap/encrypt the whole GSTP request to all participants (multiple `recipient` assertions) so non-recipients cannot see others’ ARIDs
  - Send initial “signCommit” requests: one per participant, encrypted to them, carrying group ID, target digest, participant list/threshold, commitment response ARID, and their per-participant share ARID
  - Persist session state under `group-state/<group-id>/signing/<session-id>/...`
- `frost sign commit` (participant)
  - Receive “signCommit”, run FROST signing part1 -> commitment(s)
  - Post commitment to coordinator’s commitment ARID; include participant’s next `response_arid` for signature share
- `frost sign collect` (coordinator)
  - Collect commitments from commitment ARID, validate
  - Send per-participant GSTP “signShare” requests with aggregated commitments and per-participant share ARID; update pending_requests for share collection
- `frost sign share` (participant)
  - Receive “signShare”, run FROST signing part2 -> signature share
  - Post signature share to coordinator’s share collection ARID
- `frost sign finish` (coordinator)
  - Collect signature shares, aggregate to final `Signature::Ed25519`
  - Persist/print signature (UR for display, CBOR `Signature(...)` in envelopes)
  - Any participant can attach signature to target envelope as `'signed': Signature`

Notes:
- All signing is over the digest of the *subject* of the target envelope.
- ARID flow: `send_to_arid` (coordinator posting), `collect_from_arid` (coordinator collecting), participant `response_arid` fields in GSTP payloads, and local `listening_at_arid`.
- Previews (`--preview`) remain dry-run and non-mutating; `--verbose` only when Hubert transfer logs are desired.
- Store session artifacts (commitments, shares, final signature, metadata) under `group-state/<group-id>/signing/<session-id>/`.

## Implementation Notes

### GroupRecord Enhancements

The `ContributionPaths` structure now used for Round 2:
```rust
pub struct ContributionPaths {
    pub round1_secret: Option<String>,
    pub round1_package: Option<String>,
    pub round2_secret: Option<String>,  // Populated during round2 respond
    pub key_package: Option<String>,    // Ready for finalize
}
```

The `PendingRequests` structure tracks response ARIDs across phases:
```rust
pub struct PendingRequests {
    requests: Vec<PendingRequest>,  // Maps participant XID to send_to / collect_from
}
```

### GSTP Flow

The pattern is established:
1. Coordinator sends `SealedRequest` with function name and parameters
2. Participants decrypt, validate, process
3. Participants send `SealedResponse` with result or error
4. Coordinator collects responses

### Test Coverage

Add integration tests for:
- Round 2 message construction and distribution
- Key package generation
- End-to-end signing flow

### Demo Script Updates

- Demo now includes preview Round 2 response (no state change) followed by sealed post with `--verbose` for Hubert interactions.
- After implementing subsequent phases, extend `frost-demo.py` to cover round2 collect, finalize send/respond, and signing.
