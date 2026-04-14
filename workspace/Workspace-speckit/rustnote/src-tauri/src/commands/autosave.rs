use crate::model::RecoverySnapshot;
use crate::services::autosave::{AutosaveConfig, AutosaveService, BackupFile, RustAutosaveManager, compute_content_hash};
use std::path::PathBuf;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use uuid::Uuid;

static AUTOSAVE_SERVICE: Lazy<Mutex<AutosaveService>> = Lazy::new(|| {
    let app_data_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rustnote");
    Mutex::new(AutosaveService::new(app_data_dir))
});

static RUST_AUTOSAVE_MANAGER: Lazy<RustAutosaveManager> = Lazy::new(|| {
    let app_data_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rustnote");
    RustAutosaveManager::new(app_data_dir)
});

fn get_app_data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rustnote")
}

#[tauri::command]
pub async fn trigger_autosave(
    file_path: Option<String>,
    content: String,
    cursor_offset: usize,
    title: String,
    auto_save: bool,
    auto_save_interval: u64,
) -> Result<Option<String>, String> {
    if !auto_save {
        return Ok(None);
    }

    let mut service = AUTOSAVE_SERVICE.lock().map_err(|e| e.to_string())?;

    let content_hash = compute_content_hash(&content);
    let doc_id = Uuid::new_v4();

    service.mark_dirty(doc_id, content_hash);

    if service.can_autosave(doc_id) {
        let app_data_dir = get_app_data_dir();
        let snapshot = RecoverySnapshot::new(file_path.clone(), content, cursor_offset, title);
        let id = snapshot.id.clone();
        
        snapshot.save(&app_data_dir).map_err(|e| e.to_string())?;
        service.record_autosave(doc_id, content_hash);
        
        Ok(Some(id))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn rust_autosave_trigger(
    doc_id: String,
    content: String,
    cursor_offset: usize,
    file_path: Option<String>,
    title: String,
) -> Result<Option<String>, String> {
    let uuid = Uuid::parse_str(&doc_id).map_err(|e| e.to_string())?;
    Ok(RUST_AUTOSAVE_MANAGER.trigger_autosave(uuid, &content, cursor_offset, file_path, &title))
}

#[tauri::command]
pub fn rust_autosave_mark_dirty(doc_id: String, content: String) -> Result<(), String> {
    let uuid = Uuid::parse_str(&doc_id).map_err(|e| e.to_string())?;
    let hash = compute_content_hash(&content);
    RUST_AUTOSAVE_MANAGER.mark_dirty(uuid, hash);
    Ok(())
}

#[tauri::command]
pub fn rust_autosave_create_backup(doc_id: String, content: String) -> Result<Option<BackupFile>, String> {
    let uuid = Uuid::parse_str(&doc_id).map_err(|e| e.to_string())?;
    Ok(RUST_AUTOSAVE_MANAGER.create_backup(uuid, &content))
}

#[tauri::command]
pub fn rust_autosave_list_backups(doc_id: Option<String>) -> Result<Vec<BackupFile>, String> {
    let uuid = doc_id.and_then(|id| Uuid::parse_str(&id).ok());
    Ok(RUST_AUTOSAVE_MANAGER.list_backups(uuid))
}

#[tauri::command]
pub fn rust_autosave_delete_backup(backup_id: String) -> Result<bool, String> {
    Ok(RUST_AUTOSAVE_MANAGER.delete_backup(&backup_id))
}

#[tauri::command]
pub fn rust_autosave_set_debounce(debounce_ms: u64) -> Result<(), String> {
    RUST_AUTOSAVE_MANAGER.set_debounce(debounce_ms);
    Ok(())
}

#[tauri::command]
pub fn rust_autosave_get_debounce() -> Result<u64, String> {
    Ok(RUST_AUTOSAVE_MANAGER.get_debounce())
}

#[tauri::command]
pub fn rust_autosave_set_enabled(enabled: bool) -> Result<(), String> {
    RUST_AUTOSAVE_MANAGER.set_enabled(enabled);
    Ok(())
}

#[tauri::command]
pub fn rust_autosave_is_enabled() -> Result<bool, String> {
    Ok(RUST_AUTOSAVE_MANAGER.is_enabled())
}

#[tauri::command]
pub async fn autosave_set_config(
    enabled: bool,
    interval_ms: u64,
    debounce_ms: u64,
) -> Result<(), String> {
    let mut service = AUTOSAVE_SERVICE.lock().map_err(|e| e.to_string())?;

    service.set_config(AutosaveConfig {
        enabled,
        interval_ms,
        debounce_ms,
    });

    Ok(())
}

#[tauri::command]
pub fn autosave_get_config() -> Result<String, String> {
    let service = AUTOSAVE_SERVICE.lock().map_err(|e| e.to_string())?;

    let config = service.get_config();
    serde_json::to_string(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn autosave_reset_document(doc_id: String) -> Result<(), String> {
    let mut service = AUTOSAVE_SERVICE.lock().map_err(|e| e.to_string())?;

    if let Ok(uuid) = Uuid::parse_str(&doc_id) {
        service.remove_document(uuid);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_content_hash_deterministic() {
        let content = "# Test Document\n\nSome content here";
        let hash1 = compute_content_hash(content);
        let hash2 = compute_content_hash(content);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_compute_content_hash_different() {
        let hash1 = compute_content_hash("content 1");
        let hash2 = compute_content_hash("content 2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_autosave_config_default_values() {
        let config = AutosaveConfig::default();
        assert!(config.enabled);
        assert_eq!(config.interval_ms, 30000);
        assert_eq!(config.debounce_ms, 2000);
    }
}