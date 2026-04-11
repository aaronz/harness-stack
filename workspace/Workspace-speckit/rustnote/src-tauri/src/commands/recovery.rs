use crate::model::{RecoveryData, RecoverySnapshot, RecoverySnapshotMeta};
use std::path::PathBuf;

fn get_app_data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rustnote")
}

#[tauri::command]
pub async fn save_recovery_snapshot(
    file_path: Option<String>,
    content: String,
    cursor_offset: usize,
    title: String,
) -> Result<String, String> {
    let app_data_dir = get_app_data_dir();
    let snapshot = RecoverySnapshot::new(file_path, content, cursor_offset, title);
    let id = snapshot.id.clone();
    
    snapshot.save(&app_data_dir).map_err(|e| e.to_string())?;
    
    Ok(id)
}

#[tauri::command]
pub async fn list_recovery_snapshots() -> Result<Vec<RecoverySnapshotMeta>, String> {
    let app_data_dir = get_app_data_dir();
    let snapshots = RecoverySnapshot::list(&app_data_dir).map_err(|e| e.to_string())?;
    
    Ok(snapshots.iter().map(RecoverySnapshotMeta::from).collect())
}

#[tauri::command]
pub async fn restore_recovery_snapshot(id: String) -> Result<RecoveryData, String> {
    let app_data_dir = get_app_data_dir();
    let snapshots = RecoverySnapshot::list(&app_data_dir).map_err(|e| e.to_string())?;
    
    let snapshot = snapshots
        .iter()
        .find(|s| s.id == id)
        .ok_or_else(|| "Snapshot not found".to_string())?;
    
    Ok(snapshot.restore())
}

#[tauri::command]
pub async fn delete_recovery_snapshot(id: String) -> Result<(), String> {
    let app_data_dir = get_app_data_dir();
    RecoverySnapshot::delete(&id, &app_data_dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cleanup_old_snapshots(max_age_hours: u64) -> Result<usize, String> {
    let app_data_dir = get_app_data_dir();
    let snapshots = RecoverySnapshot::list(&app_data_dir).map_err(|e| e.to_string())?;
    
    let cutoff = chrono::Utc::now() - chrono::Duration::hours(max_age_hours as i64);
    let mut deleted = 0;
    
    for snapshot in snapshots {
        if snapshot.timestamp < cutoff {
            let _ = RecoverySnapshot::delete(&snapshot.id, &app_data_dir);
            deleted += 1;
        }
    }
    
    Ok(deleted)
}