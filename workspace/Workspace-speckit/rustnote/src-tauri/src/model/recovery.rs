use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySnapshot {
    pub id: String,
    pub file_path: Option<String>,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub cursor_offset: usize,
    pub title: String,
}

impl RecoverySnapshot {
    pub fn new(
        file_path: Option<String>,
        content: String,
        cursor_offset: usize,
        title: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            file_path,
            content,
            timestamp: Utc::now(),
            cursor_offset,
            title,
        }
    }

    pub fn save(&self, app_data_dir: &PathBuf) -> Result<PathBuf, std::io::Error> {
        let snapshots_dir = app_data_dir.join("snapshots");
        fs::create_dir_all(&snapshots_dir)?;

        let file_path = snapshots_dir.join(format!("{}.json", self.id));
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(&file_path, json)?;

        Ok(file_path)
    }

    pub fn list(app_data_dir: &PathBuf) -> Result<Vec<RecoverySnapshot>, std::io::Error> {
        let snapshots_dir = app_data_dir.join("snapshots");

        if !snapshots_dir.exists() {
            return Ok(Vec::new());
        }

        let mut snapshots = Vec::new();

        for entry in fs::read_dir(&snapshots_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(json) = fs::read_to_string(&path) {
                    if let Ok(snapshot) = serde_json::from_str::<RecoverySnapshot>(&json) {
                        snapshots.push(snapshot);
                    }
                }
            }
        }

        snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(snapshots)
    }

    pub fn restore(&self) -> RecoveryData {
        RecoveryData {
            content: self.content.clone(),
            cursor_offset: self.cursor_offset,
            file_path: self.file_path.clone(),
            title: self.title.clone(),
        }
    }

    pub fn delete(id: &str, app_data_dir: &PathBuf) -> Result<(), std::io::Error> {
        let snapshots_dir = app_data_dir.join("snapshots");
        let file_path = snapshots_dir.join(format!("{}.json", id));

        if file_path.exists() {
            fs::remove_file(file_path)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryData {
    pub content: String,
    pub cursor_offset: usize,
    pub file_path: Option<String>,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySnapshotMeta {
    pub id: String,
    pub file_path: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub title: String,
    pub content_preview: String,
}

impl From<&RecoverySnapshot> for RecoverySnapshotMeta {
    fn from(snapshot: &RecoverySnapshot) -> Self {
        let preview = if snapshot.content.len() > 100 {
            format!("{}...", &snapshot.content[..100])
        } else {
            snapshot.content.clone()
        };

        Self {
            id: snapshot.id.clone(),
            file_path: snapshot.file_path.clone(),
            timestamp: snapshot.timestamp,
            title: snapshot.title.clone(),
            content_preview: preview,
        }
    }
}
