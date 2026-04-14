use crate::model::{RecoveryData, RecoverySnapshot, RecoverySnapshotMeta};
use std::path::PathBuf;

pub type RecoveryResult<T> = Result<T, RecoveryServiceError>;

#[derive(Debug, Clone)]
pub enum RecoveryServiceError {
    NotFound(String),
    Io(String),
    Serialization(String),
}

impl std::error::Error for RecoveryServiceError {}

impl std::fmt::Display for RecoveryServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecoveryServiceError::NotFound(s) => write!(f, "Snapshot not found: {}", s),
            RecoveryServiceError::Io(s) => write!(f, "IO error: {}", s),
            RecoveryServiceError::Serialization(s) => write!(f, "Serialization error: {}", s),
        }
    }
}

impl From<std::io::Error> for RecoveryServiceError {
    fn from(e: std::io::Error) -> Self {
        RecoveryServiceError::Io(e.to_string())
    }
}

impl From<serde_json::Error> for RecoveryServiceError {
    fn from(e: serde_json::Error) -> Self {
        RecoveryServiceError::Serialization(e.to_string())
    }
}

pub trait RecoveryServiceTrait: Send + Sync {
    fn save_snapshot(
        &self,
        file_path: Option<String>,
        content: String,
        cursor_offset: usize,
        title: String,
    ) -> RecoveryResult<String>;

    fn list_snapshots(&self) -> RecoveryResult<Vec<RecoverySnapshotMeta>>;

    fn restore_snapshot(&self, id: &str) -> RecoveryResult<RecoveryData>;

    fn delete_snapshot(&self, id: &str) -> RecoveryResult<()>;

    fn cleanup_old(&self, max_age_hours: u64) -> RecoveryResult<usize>;
}

pub struct RecoveryService {
    app_data_dir: PathBuf,
}

impl RecoveryService {
    pub fn new() -> Self {
        Self {
            app_data_dir: dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("rustnote"),
        }
    }
}

impl Default for RecoveryService {
    fn default() -> Self {
        Self::new()
    }
}

impl RecoveryServiceTrait for RecoveryService {
    fn save_snapshot(
        &self,
        file_path: Option<String>,
        content: String,
        cursor_offset: usize,
        title: String,
    ) -> RecoveryResult<String> {
        let snapshot = RecoverySnapshot::new(file_path, content, cursor_offset, title);
        let id = snapshot.id.clone();
        snapshot
            .save(&self.app_data_dir)
            .map_err(RecoveryServiceError::from)?;
        Ok(id)
    }

    fn list_snapshots(&self) -> RecoveryResult<Vec<RecoverySnapshotMeta>> {
        let snapshots =
            RecoverySnapshot::list(&self.app_data_dir).map_err(RecoveryServiceError::from)?;
        Ok(snapshots.iter().map(RecoverySnapshotMeta::from).collect())
    }

    fn restore_snapshot(&self, id: &str) -> RecoveryResult<RecoveryData> {
        let snapshots =
            RecoverySnapshot::list(&self.app_data_dir).map_err(RecoveryServiceError::from)?;

        let snapshot = snapshots
            .iter()
            .find(|s| s.id == id)
            .ok_or_else(|| RecoveryServiceError::NotFound(id.to_string()))?;

        Ok(snapshot.restore())
    }

    fn delete_snapshot(&self, id: &str) -> RecoveryResult<()> {
        RecoverySnapshot::delete(id, &self.app_data_dir).map_err(RecoveryServiceError::from)
    }

    fn cleanup_old(&self, max_age_hours: u64) -> RecoveryResult<usize> {
        let snapshots =
            RecoverySnapshot::list(&self.app_data_dir).map_err(RecoveryServiceError::from)?;

        let cutoff = chrono::Utc::now() - chrono::Duration::hours(max_age_hours as i64);
        let mut deleted = 0usize;

        for snapshot in snapshots {
            if snapshot.timestamp < cutoff {
                let _ = RecoverySnapshot::delete(&snapshot.id, &self.app_data_dir);
                deleted += 1;
            }
        }

        Ok(deleted)
    }
}
