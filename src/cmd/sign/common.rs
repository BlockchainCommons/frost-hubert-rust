use std::path::{Path, PathBuf};

use bc_components::ARID;

/// Returns the signing state directory for a group (without session).
/// Path: `{registry_dir}/group-state/{group_id.hex()}/signing`
pub fn signing_state_dir_for_group(
    registry_path: &Path,
    group_id: &ARID,
) -> PathBuf {
    let base = registry_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("group-state")
        .join(group_id.hex())
        .join("signing")
}

/// Returns the signing state directory for a specific session.
/// Path: `{registry_dir}/group-state/{group_id.hex()}/signing/{session_id.
/// hex()}`
pub fn signing_state_dir(
    registry_path: &Path,
    group_id: &ARID,
    session_id: &ARID,
) -> PathBuf {
    signing_state_dir_for_group(registry_path, group_id).join(session_id.hex())
}
