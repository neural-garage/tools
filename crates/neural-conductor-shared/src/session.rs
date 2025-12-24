//! Session management types

use super::{SessionId, TaskStatus};
use serde::{Deserialize, Serialize};

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub workspace_path: String,
    pub status: TaskStatus,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Session {
    pub fn new(id: SessionId, workspace_path: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id,
            workspace_path,
            status: TaskStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
}
