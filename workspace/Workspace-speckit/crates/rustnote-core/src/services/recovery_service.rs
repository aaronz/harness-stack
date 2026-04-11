use crate::core::recovery::{RecoverySnapshot, RecoverySnapshotMeta};
use crate::error::Result;
use std::fs;
use std::path::PathBuf;

pub struct RecoveryService;

impl RecoveryService {
    fn snapshots_dir() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rustnote")
            .join("snapshots")
    }

    pub fn save_snapshot(snapshot: &RecoverySnapshot) -> Result<()> {
        let dir = Self::snapshots_dir();
        fs::create_dir_all(&dir)?;
        let path = dir.join(format!("{}.json", snapshot.id));
        let content = serde_json::to_string_pretty(snapshot)
            .map_err(|e| crate::error::Error::Serialization(e.to_string()))?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn list_snapshots() -> Result<Vec<RecoverySnapshotMeta>> {
        let dir = Self::snapshots_dir();
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut snapshots = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(snapshot) = serde_json::from_str::<RecoverySnapshot>(&content) {
                        snapshots.push(RecoverySnapshotMeta {
                            id: snapshot.id,
                            file_path: snapshot.file_path,
                            timestamp: snapshot.timestamp,
                        });
                    }
                }
            }
        }
        snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(snapshots)
    }

    pub fn restore_snapshot(id: &uuid::Uuid) -> Result<(String, usize)> {
        let dir = Self::snapshots_dir();
        let path = dir.join(format!("{}.json", id));
        let content = fs::read_to_string(&path)?;
        let snapshot: RecoverySnapshot = serde_json::from_str(&content)
            .map_err(|e| crate::error::Error::Serialization(e.to_string()))?;
        Ok((snapshot.content, snapshot.cursor_offset))
    }

    pub fn delete_snapshot(id: &uuid::Uuid) -> Result<()> {
        let dir = Self::snapshots_dir();
        let path = dir.join(format!("{}.json", id));
        if path.exists() {
            fs::remove_file(&path)?;
        }
        Ok(())
    }
}
