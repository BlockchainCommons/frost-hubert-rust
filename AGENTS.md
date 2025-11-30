# Guidelines for Developers working with the `frost` crate

Always read this *entire* file before working in this crate.

The `frost` CLI is a working tool for managing FROST (Flexible Round-Optimized Schnorr Threshold) participants. Current functionality:

## General Guidelines

- This crate is unreleased *de novo* development. Don't concern yourself with backward compatibility yet.

## Relevant Crates in This Workspace

- `bc-components`: Blockchain Commons Cryptographic Components
    - `XID`: eXtensible IDentifier
    - `ARID`: Apparently Random Identifier
- `bc-envelope`: Gordian Envelope
    - `Envelope`: Envelope Structure
    - `queries.rs`: Envelope Querying
- `dcbor`: Deterministic CBOR
    - `CBOR`: CBOR item
    - `Date`: Date Type
- `bc-xid`: XID (eXtensible IDentifiers)
    - `XIDDocument`: XID Document Structure
- `gstp`: Gordian Sealed Transaction Protocol
    - `SealedRequest`: Sealed Request Structure
    - `SealedResponse`: Sealed Response Structure
    - `SealedEvent`: Sealed Event Structure
- `hubert`: Gordian Hubert Protocol
- `bc-ur`: Blockchain Commons UR (Uniform Resources)
    - `UR`: UR Type
- `provenance-mark`: Provenance Marks
    - `ProvenanceMark`: Provenance Mark Type
    - `ProvenanceMarkGenerator`: Generator for Provenance Marks
    - `ProvenanceMarkResolution`: Resolution for Provenance Marks
- `research`: The Blockchain Commons Research Repository

## Tests

- Use the "expected text output rubric" approach for tests that compare text output.
- Use the `assert_actual_expected!` macro for comparing actual and expected text output in tests.
- Always use `indoc!` for multi-line expected text output in tests.

## Essential Hubert Knowledge (transport)

- Hubert is just a key/value transport keyed by ARID; always include and persist the ARID for the *next* hop when you expect a reply (store it as `listening_at_arid`, send it as `response_arid`).
- Field naming for ARID flow:
  - `response_arid`: tell the peer where to send their next message.
  - `send_to_arid`: coordinator records where to post to a participant.
  - `collect_from_arid`: coordinator records where to fetch a participant’s response.
  - `listening_at_arid`: local state for where *you* are listening next.
- Verbosity: use `--verbose` only when you want Hubert transfer logs; keep previews quiet.
- Previews: use `--preview` and pipe to `envelope format`; previews must not mutate local state or post to storage.
- Payload routing modes (transport-level concerns):
- Multicast preview/audit (e.g., preview invite): everyone can view the message, but per-recipient fields like response ARIDs must be individually encrypted under a `recipient` assertion on the inner assertion so other participants cannot see them.
  - Single-cast deliveries (e.g., Round 2 per-participant messages): the whole message is encrypted to one recipient; when wrapping per-recipient payloads, add a `recipient` assertion to the wrapped byte string so the coordinator can fan out later, but the payload itself stays inside the single-recipient envelope.

## Essential GSTP Knowledge (message encoding)

- Always include the request/response correlation: GSTP requests carry an ID; responses must use the same ID. Continuations (`peer_continuation`/`state`) let each side bounce back valid IDs and optional state.
- Use `SealedRequest::to_envelope_for_recipients` / `SealedResponse::to_envelope` with the sender’s signing keys and recipient encryption keys; `try_from_envelope` / `try_from_encrypted_envelope` on the receiver side.
- GSTP adds its own `recipient` encryption wrapper when you target specific recipients; do not confuse this with any inner `recipient` assertions you add for transport routing of individual fields (e.g., per-recipient response ARIDs).
- Function/parameter checks: compare against `Function::from("name")` rather than `.name()`; extract parameters via `extract_object_for_parameter`.
- Errors: `SealedResponse::error()` returns `Ok(envelope)` when an error is present—check it before reading `result()`.
- Dates/validity: include `valid_until` on requests when expiration matters; continuations are validated against expected IDs/dates.
- Byte data: wrap byte strings as CBOR byte strings (`CBOR::to_byte_string`) to avoid accidental arrays; unwrap with `try_leaf` + `CBOR::try_into_byte_string`.

## Important Notes

- `frost-demo.py` generates `demo-log.md`. When you enhance the tool, consider enhancing the `frost-demo.py` script to reflect those changes in the demo log then regenerate `demo-log.md`.
- Before you run `frost-demo.py`, install the latest build of the `frost` tool with `cargo install --path .`.

## Workflows

1. **Registry Management** (`frost registry`)
   - Owner set with private XID documents
   - Participant add with signed public XID documents
   - Persistent JSON storage

2. **DKG Invite Flow** (`frost dkg coordinator invite` / `frost dkg participant receive` / `frost dkg participant round1`)
   - `invite`: Coordinator creates sealed/preview invites for participants
   - `receive`: Participants fetch and decrypt invites from Hubert or local envelope
   - `round1`: Participants accept or reject, posting Round 1 response to Hubert

3. **DKG Round 1** (`frost dkg coordinator round1`)
   - Coordinator fetches all participant responses from Hubert, validates GSTP responses, extracts Round 1 packages, saves to `collected_round1.json`, and posts individualized Round 2 requests (with optional preview)

4. **DKG Round 2** (`frost dkg coordinator round2` / `frost dkg participant round2`)
   - Participant: Responds with round2 packages, persists round2 secret, includes next `response_arid`, and updates `listening_at_arid`
   - Coordinator: Fetches/validates Round 2 responses, saves `collected_round2.json`, and immediately posts finalize requests to each participant (combined collect + finalize dispatch)

5. **DKG Finalize** (`frost dkg coordinator finalize` / `frost dkg participant finalize`)
   - Participant: Runs part3, produces key/public key packages, persists them, and returns finalize response
   - Coordinator: Collects finalize responses, writes `collected_finalize.json`, clears pending requests, and reports the group verifying key (`SigningPublicKey::Ed25519`, UR form `ur:signing-public-key`)

6. **Signing**
   - `sign coordinator invite`: builds session, per-participant ARIDs (commit/share), target envelope, and posts signInvite requests
   - `sign participant receive`: decrypts/validates signInvite, shows participants/target, persists session state + commit response ARID
   - `sign participant round1`: generates nonces/commitments, posts signRound1Response with next-hop share ARID, persists part1 state, updates listening ARID
   - `sign coordinator round1`: collects all commitments, stores `commitments.json`, and dispatches per-participant signRound2 requests
   - `sign participant round2`: fetches signRound2, validates session/commitments, produces signature share, posts to share ARID, and persists `share.json`
   - `sign coordinator round2`: collects signature shares, combines into full signature, and outputs signed envelope
   - `sign participant finalize`: validates full signature against target envelope

7. **Storage Backends**
   - Hubert server (HTTP)
   - Mainline DHT
   - IPFS
   - Hybrid (DHT + IPFS)

8. **Demo Script** (`frost-demo.py`)
   - Provisions 4 participants (Alice, Bob, Carol, Dan) in separate directories
   - Builds registries
   - Creates and responds to DKG invites via Hubert
   - Coordinator collects Round 1 packages and dispatches Round 2 requests
   - Coordinator collects Round 2 responses and dispatches finalize requests (combined command)
   - Participants post finalize responses
   - Coordinator collects finalize responses and outputs the group verifying key
   - Signing: builds a sample target envelope, previews `sign coordinator invite` (signInvite), posts it, participants run `sign participant receive`/`sign participant round1`, coordinator runs `sign coordinator round1` and posts signRound2 requests, participants run `sign participant round2` and post signature shares, coordinator runs `sign coordinator round2` to combine shares and output signed envelope, participants run `sign participant finalize` to validate the full signature
