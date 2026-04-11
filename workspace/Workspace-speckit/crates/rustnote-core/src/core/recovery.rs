use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverySnapshot {
    pub id: Uuid,
    pub file_path: PathBuf,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub cursor_offset: usize,
}

impl RecoverySnapshot {
    pub fn new(file_path: PathBuf, content: String, cursor_offset: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_path,
            content,
            timestamp: Utc::now(),
            cursor_offset,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverySnapshotMeta {
    pub id: Uuid,
    pub file_path: PathBuf,
    pub timestamp: DateTime<Utc>,
}
