use crate::model::RecoverySnapshot;
use crate::services::autosave::{AutosaveConfig, AutosaveService, compute_content_hash};
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