//! Durable message-ID floor and interruption guard for offline mail retention.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{DbError, DbResult};

pub const STATE_FILE: &str = ".mail-retention.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionState {
    pub version: u32,
    pub complete: bool,
    pub message_id_floor: i64,
    pub run_dir: PathBuf,
}

/// Read the retained allocator floor, refusing malformed or symlinked evidence.
///
/// # Errors
/// Returns an error if the record cannot be safely read or is invalid.
pub fn read_state(storage_root: &Path) -> DbResult<Option<RetentionState>> {
    let path = storage_root.join(STATE_FILE);
    let metadata = match std::fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(DbError::Sqlite(format!("retention state: {error}"))),
    };
    if !metadata.file_type().is_file() || metadata.len() > 16_384 {
        return Err(DbError::Sqlite(format!(
            "retention state {} must be a bounded regular file",
            path.display()
        )));
    }
    let state: RetentionState = serde_json::from_slice(
        &mcp_agent_mail_core::disk::read_regular_file_no_follow_bounded(&path, 16_384)
            .map_err(|error| DbError::Sqlite(error.to_string()))?,
    )
    .map_err(|error| DbError::Sqlite(format!("retention state: {error}")))?;
    if state.version != 1 || state.message_id_floor < 0 || !state.run_dir.is_absolute() {
        return Err(DbError::Sqlite("invalid retention state".to_owned()));
    }
    Ok(Some(state))
}

/// Runtime startup and reconstruction fail closed while maintenance is unfinished.
///
/// # Errors
/// Returns an error for an unfinished operation or invalid record.
pub fn require_complete(storage_root: &Path) -> DbResult<Option<i64>> {
    match read_state(storage_root)? {
        Some(state) if !state.complete => Err(DbError::Sqlite(format!(
            "mail retention is unfinished; resume `am archive prune --apply --run-dir {}` before starting the service",
            state.run_dir.display()
        ))),
        Some(state) => Ok(Some(state.message_id_floor)),
        None => Ok(None),
    }
}
