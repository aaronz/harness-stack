use crate::model::{Document, RecoverySnapshot};
use crate::services::DocumentServiceError;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration as StdDuration, Instant};
use uuid::Uuid;

/// Maximum number of backup files to keep per document
const MAX_BACKUP_FILES: usize = 10;

/// Backup file metadata for tracking and cleanup
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupFile {
    pub id: String,
    pub doc_id: String,
    pub file_path: String,
    pub created_at: DateTime<Utc>,
    pub content_hash: u64,
    pub size_bytes: u64,
}

/// Autosave result type
pub type AutosaveResult<T> = Result<T, AutosaveServiceError>;

/// Error type for autosave operations
#[derive(Debug, Clone)]
pub enum AutosaveServiceError {
    Io(String),
    DocumentNotFound(String),
}

impl std::error::Error for AutosaveServiceError {}

impl std::fmt::Display for AutosaveServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutosaveServiceError::Io(s) => write!(f, "IO error: {}", s),
            AutosaveServiceError::DocumentNotFound(s) => write!(f, "Document not found: {}", s),
        }
    }
}

impl From<std::io::Error> for AutosaveServiceError {
    fn from(e: std::io::Error) -> Self {
        AutosaveServiceError::Io(e.to_string())
    }
}

/// AutosaveServiceTrait defines the interface for autosave operations
/// This trait matches PRD-10 specification for the autosave service
pub trait AutosaveServiceTrait: Send + Sync {
    /// Mark a document as dirty (needing autosave)
    fn mark_dirty(&mut self, doc_id: Uuid, content_hash: u64);

    /// Check if a document needs autosave
    fn needs_autosave(&self, doc_id: Uuid) -> bool;

    /// Check if enough time has passed since last save for debounced autosave
    fn can_autosave(&self, doc_id: Uuid) -> bool;

    /// Record that autosave was performed
    fn record_autosave(&mut self, doc_id: Uuid, content_hash: u64);

    /// Create a recovery snapshot for the document
    fn create_recovery_snapshot(
        &self,
        doc: &Document,
        cursor_offset: usize,
    ) -> AutosaveResult<String>;

    /// Remove tracking for a closed document
    fn remove_document(&mut self, doc_id: Uuid);

    /// Get all tracked document IDs
    fn tracked_documents(&self) -> Vec<Uuid>;

    /// Check if autosave is enabled
    fn is_enabled(&self) -> bool;

    /// Enable or disable autosave
    fn set_enabled(&mut self, enabled: bool);

    /// Get autosave interval in milliseconds
    fn get_interval(&self) -> u64;

    /// Set autosave interval in milliseconds
    fn set_interval(&mut self, interval_ms: u64);

    /// Get debounce delay in milliseconds
    fn get_debounce(&self) -> u64;

    /// Set debounce delay in milliseconds
    fn set_debounce(&mut self, debounce_ms: u64);

    /// Get current configuration
    fn get_config(&self) -> AutosaveConfig;

    /// Update configuration
    fn set_config(&mut self, config: AutosaveConfig);

    /// Check if a document is being tracked
    fn is_tracked(&self, doc_id: Uuid) -> bool;

    /// Reset dirty state without saving
    fn reset_dirty(&mut self, doc_id: Uuid);
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AutosaveConfig {
    pub enabled: bool,
    pub interval_ms: u64,
    pub debounce_ms: u64,
}

impl Default for AutosaveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_ms: 30000,
            debounce_ms: 2000,
        }
    }
}

/// Autosave state tracking for a document
#[derive(Debug, Clone)]
struct AutosaveState {
    last_content_hash: u64,
    last_save_time: Instant,
    pending_save: bool,
    dirty: bool,
}

impl AutosaveState {
    fn new() -> Self {
        Self {
            last_content_hash: 0,
            last_save_time: Instant::now(),
            pending_save: false,
            dirty: false,
        }
    }
}

/// Autosave service with Rust-side debounce timer
/// This provides more reliable autosave than frontend timers
pub struct AutosaveService {
    /// Document states being tracked
    states: HashMap<Uuid, AutosaveState>,
    /// Configuration
    config: AutosaveConfig,
    /// App data directory for recovery snapshots
    app_data_dir: PathBuf,
}

impl AutosaveService {
    pub fn new(app_data_dir: PathBuf) -> Self {
        Self {
            states: HashMap::new(),
            config: AutosaveConfig::default(),
            app_data_dir,
        }
    }

    pub fn set_config(&mut self, config: AutosaveConfig) {
        self.set_config_impl(config);
    }

    pub fn get_config(&self) -> AutosaveConfig {
        self.get_config_impl()
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.set_enabled_impl(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled_impl()
    }

    pub fn set_interval(&mut self, interval_ms: u64) {
        self.set_interval_impl(interval_ms);
    }

    pub fn get_interval(&self) -> u64 {
        self.get_interval_impl()
    }

    pub fn set_debounce(&mut self, debounce_ms: u64) {
        self.set_debounce_impl(debounce_ms);
    }

    pub fn get_debounce(&self) -> u64 {
        self.get_debounce_impl()
    }

    pub fn mark_dirty(&mut self, doc_id: Uuid, content_hash: u64) {
        self.mark_dirty_impl(doc_id, content_hash);
    }

    pub fn needs_autosave(&self, doc_id: Uuid) -> bool {
        self.needs_autosave_impl(doc_id)
    }

    pub fn can_autosave(&self, doc_id: Uuid) -> bool {
        self.can_autosave_impl(doc_id)
    }

    pub fn record_autosave(&mut self, doc_id: Uuid, content_hash: u64) {
        self.record_autosave_impl(doc_id, content_hash);
    }

    pub fn create_recovery_snapshot(
        &self,
        doc: &Document,
        cursor_offset: usize,
    ) -> Result<String, DocumentServiceError> {
        self.create_recovery_snapshot_impl(doc, cursor_offset)
    }

    pub fn remove_document(&mut self, doc_id: Uuid) {
        self.remove_document_impl(doc_id);
    }

    pub fn tracked_documents(&self) -> Vec<Uuid> {
        self.tracked_documents_impl()
    }

    pub fn is_tracked(&self, doc_id: Uuid) -> bool {
        self.is_tracked_impl(doc_id)
    }

    pub fn reset_dirty(&mut self, doc_id: Uuid) {
        self.reset_dirty_impl(doc_id);
    }

    fn set_config_impl(&mut self, config: AutosaveConfig) {
        self.config = config;
    }

    fn get_config_impl(&self) -> AutosaveConfig {
        self.config.clone()
    }

    fn set_enabled_impl(&mut self, enabled: bool) {
        self.config.enabled = enabled;
    }

    fn is_enabled_impl(&self) -> bool {
        self.config.enabled
    }

    fn set_interval_impl(&mut self, interval_ms: u64) {
        self.config.interval_ms = interval_ms;
    }

    fn get_interval_impl(&self) -> u64 {
        self.config.interval_ms
    }

    fn set_debounce_impl(&mut self, debounce_ms: u64) {
        self.config.debounce_ms = debounce_ms;
    }

    fn get_debounce_impl(&self) -> u64 {
        self.config.debounce_ms
    }

    fn mark_dirty_impl(&mut self, doc_id: Uuid, content_hash: u64) {
        if let Some(state) = self.states.get_mut(&doc_id) {
            state.dirty = true;
            if content_hash != state.last_content_hash {
                state.pending_save = true;
            }
        } else {
            let mut state = AutosaveState::new();
            state.dirty = true;
            state.last_content_hash = content_hash;
            state.pending_save = true;
            self.states.insert(doc_id, state);
        }
    }

    fn needs_autosave_impl(&self, doc_id: Uuid) -> bool {
        self.config.enabled && self.states.get(&doc_id).map_or(false, |s| s.pending_save)
    }

    fn can_autosave_impl(&self, doc_id: Uuid) -> bool {
        if !self.config.enabled {
            return false;
        }

        if let Some(state) = self.states.get(&doc_id) {
            if !state.pending_save {
                return false;
            }
            let elapsed = state.last_save_time.elapsed();
            elapsed >= StdDuration::from_millis(self.config.debounce_ms)
        } else {
            false
        }
    }

    fn record_autosave_impl(&mut self, doc_id: Uuid, content_hash: u64) {
        if let Some(state) = self.states.get_mut(&doc_id) {
            state.last_save_time = Instant::now();
            state.last_content_hash = content_hash;
            state.pending_save = false;
            state.dirty = false;
        }
    }

    fn create_recovery_snapshot_impl(
        &self,
        doc: &Document,
        cursor_offset: usize,
    ) -> Result<String, DocumentServiceError> {
        let snapshot = RecoverySnapshot::new(
            doc.file_path.clone(),
            doc.content.clone(),
            cursor_offset,
            doc.title.clone(),
        );

        snapshot
            .save(&self.app_data_dir)
            .map_err(|e| DocumentServiceError::Io(e.to_string()))?;

        Ok(snapshot.id)
    }

    fn remove_document_impl(&mut self, doc_id: Uuid) {
        self.states.remove(&doc_id);
    }

    fn tracked_documents_impl(&self) -> Vec<Uuid> {
        self.states.keys().cloned().collect()
    }

    fn is_tracked_impl(&self, doc_id: Uuid) -> bool {
        self.states.contains_key(&doc_id)
    }

    fn reset_dirty_impl(&mut self, doc_id: Uuid) {
        if let Some(state) = self.states.get_mut(&doc_id) {
            state.pending_save = false;
            state.dirty = false;
        }
    }
}

impl AutosaveServiceTrait for AutosaveService {
    fn mark_dirty(&mut self, doc_id: Uuid, content_hash: u64) {
        self.mark_dirty_impl(doc_id, content_hash);
    }

    fn needs_autosave(&self, doc_id: Uuid) -> bool {
        self.needs_autosave_impl(doc_id)
    }

    fn can_autosave(&self, doc_id: Uuid) -> bool {
        self.can_autosave_impl(doc_id)
    }

    fn record_autosave(&mut self, doc_id: Uuid, content_hash: u64) {
        self.record_autosave_impl(doc_id, content_hash);
    }

    fn create_recovery_snapshot(
        &self,
        doc: &Document,
        cursor_offset: usize,
    ) -> AutosaveResult<String> {
        let snapshot = RecoverySnapshot::new(
            doc.file_path.clone(),
            doc.content.clone(),
            cursor_offset,
            doc.title.clone(),
        );

        snapshot
            .save(&self.app_data_dir)
            .map_err(|e| AutosaveServiceError::Io(e.to_string()))?;

        Ok(snapshot.id)
    }

    fn remove_document(&mut self, doc_id: Uuid) {
        self.remove_document_impl(doc_id);
    }

    fn tracked_documents(&self) -> Vec<Uuid> {
        self.tracked_documents_impl()
    }

    fn is_enabled(&self) -> bool {
        self.is_enabled_impl()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.set_enabled_impl(enabled);
    }

    fn get_interval(&self) -> u64 {
        self.get_interval_impl()
    }

    fn set_interval(&mut self, interval_ms: u64) {
        self.set_interval_impl(interval_ms);
    }

    fn get_debounce(&self) -> u64 {
        self.get_debounce_impl()
    }

    fn set_debounce(&mut self, debounce_ms: u64) {
        self.set_debounce_impl(debounce_ms);
    }

    fn get_config(&self) -> AutosaveConfig {
        self.get_config_impl()
    }

    fn set_config(&mut self, config: AutosaveConfig) {
        self.set_config_impl(config);
    }

    fn is_tracked(&self, doc_id: Uuid) -> bool {
        self.is_tracked_impl(doc_id)
    }

    fn reset_dirty(&mut self, doc_id: Uuid) {
        self.reset_dirty_impl(doc_id);
    }
}

impl Default for AutosaveService {
    fn default() -> Self {
        Self::new(PathBuf::new())
    }
}

/// Rust-side autosave manager with independent debounce timer
/// This runs independently of the frontend timer to provide backup autosave
pub struct RustAutosaveManager {
    service: Arc<Mutex<AutosaveService>>,
    backup_dir: PathBuf,
    running: Arc<Mutex<bool>>,
}

impl RustAutosaveManager {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let backup_dir = app_data_dir.join("rust_backups");
        let _ = fs::create_dir_all(&backup_dir);
        Self {
            service: Arc::new(Mutex::new(AutosaveService::new(app_data_dir.clone()))),
            backup_dir,
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self) {
        let mut running = self.running.lock().unwrap();
        if *running {
            return;
        }
        *running = true;
        drop(running);

        let service = Arc::clone(&self.service);
        let backup_dir = self.backup_dir.clone();
        let running = Arc::clone(&self.running);

        thread::spawn(move || loop {
            {
                let running_guard = running.lock().unwrap();
                if !*running_guard {
                    break;
                }
            }

            let _doc_id = {
                let mut service = service.lock().unwrap();
                let tracked = service.tracked_documents();

                for doc_id in tracked {
                    if service.can_autosave(doc_id) {
                        let config = service.get_config();
                        if config.enabled {
                            let content = format!("Auto-saved at {}", Utc::now());
                            let cursor_offset = 0;
                            let title = "Auto-save".to_string();
                            service.record_autosave(doc_id, 0);
                            let _ = service.create_recovery_snapshot(
                                &Document::from_file("auto_backup.md", content.clone())
                                    .unwrap_or_else(|_| {
                                        let mut doc = Document::new(title.clone());
                                        doc.content = content.clone();
                                        doc
                                    }),
                                cursor_offset,
                            );
                        }
                    }
                }
                Uuid::nil()
            };
            thread::sleep(StdDuration::from_millis(100));
        });
    }

    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
    }

    pub fn mark_dirty(&self, doc_id: Uuid, content_hash: u64) {
        if let Ok(mut service) = self.service.lock() {
            service.mark_dirty(doc_id, content_hash);
        }
    }

    pub fn needs_autosave(&self, doc_id: Uuid) -> bool {
        self.service
            .lock()
            .ok()
            .map_or(false, |s| s.needs_autosave(doc_id))
    }

    pub fn can_autosave(&self, doc_id: Uuid) -> bool {
        self.service
            .lock()
            .ok()
            .map_or(false, |s| s.can_autosave(doc_id))
    }

    pub fn trigger_autosave(
        &self,
        doc_id: Uuid,
        content: &str,
        cursor_offset: usize,
        file_path: Option<String>,
        title: &str,
    ) -> Option<String> {
        let mut service = match self.service.lock() {
            Ok(s) => s,
            Err(_) => return None,
        };

        let content_hash = compute_content_hash(content);
        service.mark_dirty(doc_id, content_hash);

        if service.can_autosave(doc_id) {
            let doc = Document::from_file(
                file_path.as_deref().unwrap_or("auto_backup.md"),
                content.to_string(),
            )
            .unwrap_or_else(|_| {
                let mut doc = Document::new(title.to_string());
                doc.content = content.to_string();
                doc
            });

            match service.create_recovery_snapshot(&doc, cursor_offset) {
                Ok(snapshot_id) => {
                    service.record_autosave(doc_id, content_hash);
                    Some(snapshot_id)
                }
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn create_backup(&self, doc_id: Uuid, content: &str) -> Option<BackupFile> {
        let backup_id = Uuid::new_v4().to_string();
        let file_name = format!("{}_{}.md", doc_id, backup_id);
        let file_path = self.backup_dir.join(&file_name);

        match fs::write(&file_path, content) {
            Ok(_) => {
                let metadata = fs::metadata(&file_path).ok()?;
                let backup = BackupFile {
                    id: backup_id,
                    doc_id: doc_id.to_string(),
                    file_path: file_path.to_string_lossy().to_string(),
                    created_at: Utc::now(),
                    content_hash: compute_content_hash(content),
                    size_bytes: metadata.len(),
                };
                self.cleanup_old_backups(doc_id);
                Some(backup)
            }
            Err(_) => None,
        }
    }

    pub fn cleanup_old_backups(&self, doc_id: Uuid) {
        let _ = self.cleanup_backups_for_doc(&doc_id.to_string(), MAX_BACKUP_FILES);
    }

    pub fn cleanup_backups_for_doc(&self, doc_id: &str, max_keep: usize) -> usize {
        if let Ok(entries) = fs::read_dir(&self.backup_dir) {
            let mut backups: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_name()
                        .to_string_lossy()
                        .starts_with(&format!("{}_", doc_id))
                })
                .collect();

            backups.sort_by_key(|e| e.metadata().ok().and_then(|m| m.created().ok()));

            let to_delete = backups.len().saturating_sub(max_keep);
            let mut deleted = 0;
            for entry in backups.iter().take(to_delete) {
                if fs::remove_file(entry.path()).is_ok() {
                    deleted += 1;
                }
            }
            deleted
        } else {
            0
        }
    }

    pub fn list_backups(&self, doc_id: Option<Uuid>) -> Vec<BackupFile> {
        let mut backups = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.backup_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".md") {
                    if let Some(ref did) = doc_id {
                        if !name.starts_with(&format!("{}_", did)) {
                            continue;
                        }
                    }

                    let parts: Vec<&str> = name.split('_').collect();
                    if parts.len() >= 2 {
                        let metadata = entry.metadata().ok();
                        let created_at = metadata
                            .as_ref()
                            .and_then(|m| m.created().ok())
                            .map(|t| DateTime::from(t))
                            .unwrap_or_else(Utc::now);

                        let content = fs::read_to_string(entry.path()).ok().unwrap_or_default();

                        backups.push(BackupFile {
                            id: parts
                                .get(1)
                                .unwrap_or(&"")
                                .trim_end_matches(".md")
                                .to_string(),
                            doc_id: parts.get(0).unwrap_or(&"").to_string(),
                            file_path: entry.path().to_string_lossy().to_string(),
                            created_at,
                            content_hash: compute_content_hash(&content),
                            size_bytes: metadata.as_ref().map(|m| m.len()).unwrap_or(0),
                        });
                    }
                }
            }
        }

        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        backups
    }

    pub fn delete_backup(&self, backup_id: &str) -> bool {
        if let Ok(entries) = fs::read_dir(&self.backup_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.contains(backup_id) && name.ends_with(".md") {
                    return fs::remove_file(entry.path()).is_ok();
                }
            }
        }
        false
    }

    pub fn get_config(&self) -> Option<AutosaveConfig> {
        self.service.lock().ok().map(|s| s.get_config())
    }

    pub fn set_config(&self, config: AutosaveConfig) {
        if let Ok(mut service) = self.service.lock() {
            service.set_config(config);
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.service.lock().ok().map_or(false, |s| s.is_enabled())
    }

    pub fn set_enabled(&self, enabled: bool) {
        if let Ok(mut service) = self.service.lock() {
            service.set_enabled(enabled);
        }
    }

    pub fn get_debounce(&self) -> u64 {
        self.service.lock().ok().map_or(2000, |s| s.get_debounce())
    }

    pub fn set_debounce(&self, debounce_ms: u64) {
        if let Ok(mut service) = self.service.lock() {
            service.set_debounce(debounce_ms);
        }
    }
}

#[cfg(test)]
mod rust_autosave_manager_tests {
    use super::*;
    use tempfile::TempDir;

    fn create_temp_backup_dir() -> PathBuf {
        let temp_dir = TempDir::new().unwrap();
        temp_dir.into_path()
    }

    #[test]
    fn test_rust_autosave_manager_new() {
        let dir = create_temp_backup_dir();
        let manager = RustAutosaveManager::new(dir.clone());

        assert!(manager.get_config().is_some());
        let config = manager.get_config().unwrap();
        assert!(config.enabled);
        assert_eq!(config.debounce_ms, 2000);
    }

    #[test]
    fn test_rust_autosave_manager_config() {
        let dir = create_temp_backup_dir();
        let manager = RustAutosaveManager::new(dir);

        manager.set_enabled(false);
        assert!(!manager.is_enabled());

        manager.set_enabled(true);
        assert!(manager.is_enabled());

        manager.set_debounce(5000);
        assert_eq!(manager.get_debounce(), 5000);
    }

    #[test]
    fn test_create_backup() {
        let dir = create_temp_backup_dir();
        let manager = RustAutosaveManager::new(dir);

        let doc_id = Uuid::new_v4();
        let content = "# Test Backup\n\nThis is a test backup.";

        let backup = manager.create_backup(doc_id, content);
        assert!(backup.is_some());

        let backup = backup.unwrap();
        assert_eq!(backup.doc_id, doc_id.to_string());
        assert_eq!(backup.content_hash, compute_content_hash(content));
    }

    #[test]
    fn test_list_backups() {
        let dir = create_temp_backup_dir();
        let manager = RustAutosaveManager::new(dir);

        let doc_id = Uuid::new_v4();

        manager.create_backup(doc_id, "Backup 1").unwrap();
        std::thread::sleep(StdDuration::from_millis(10));
        manager.create_backup(doc_id, "Backup 2").unwrap();

        let backups = manager.list_backups(None);
        assert!(backups.len() >= 2);

        let doc_backups = manager.list_backups(Some(doc_id));
        assert_eq!(doc_backups.len(), 2);
    }

    #[test]
    fn test_cleanup_old_backups() {
        let dir = create_temp_backup_dir();
        let manager = RustAutosaveManager::new(dir);

        let doc_id = Uuid::new_v4();

        for i in 0..15 {
            manager
                .create_backup(doc_id, &format!("Backup {}", i))
                .unwrap();
            std::thread::sleep(StdDuration::from_millis(5));
        }

        let backups = manager.list_backups(Some(doc_id));
        assert!(backups.len() <= MAX_BACKUP_FILES);
    }

    #[test]
    fn test_delete_backup() {
        let dir = create_temp_backup_dir();
        let manager = RustAutosaveManager::new(dir);

        let doc_id = Uuid::new_v4();
        let backup = manager.create_backup(doc_id, "To be deleted").unwrap();

        let backups_before = manager.list_backups(Some(doc_id));
        assert!(!backups_before.is_empty());

        let deleted = manager.delete_backup(&backup.id);
        assert!(deleted);

        let backups_after = manager.list_backups(Some(doc_id));
        assert!(backups_after.is_empty());
    }

    #[test]
    fn test_trigger_autosave() {
        let dir = create_temp_backup_dir();
        let manager = RustAutosaveManager::new(dir);

        manager.set_debounce(100);

        let doc_id = Uuid::new_v4();
        let content = "# Test Content\n\nTest autosave trigger.";

        let result = manager.trigger_autosave(doc_id, content, 10, None, "Test Doc");
        assert!(result.is_none());

        std::thread::sleep(StdDuration::from_millis(150));

        let result = manager.trigger_autosave(doc_id, content, 10, None, "Test Doc");
        assert!(result.is_some());
    }

    #[test]
    fn test_mark_dirty_and_autosave() {
        let dir = create_temp_backup_dir();
        let manager = RustAutosaveManager::new(dir);

        manager.set_debounce(100);

        let doc_id = Uuid::new_v4();
        let hash = compute_content_hash("content");

        manager.mark_dirty(doc_id, hash);
        assert!(manager.needs_autosave(doc_id));

        std::thread::sleep(StdDuration::from_millis(150));
        assert!(manager.can_autosave(doc_id));
    }
}

/// Compute a simple hash of content for change detection
pub fn compute_content_hash(content: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in content.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_autosave_config_default() {
        let config = AutosaveConfig::default();
        assert!(config.enabled);
        assert_eq!(config.interval_ms, 30000);
        assert_eq!(config.debounce_ms, 2000);
    }

    #[test]
    fn test_autosave_service_new() {
        let service = AutosaveService::new(PathBuf::from("/test"));
        assert!(service.is_enabled());
        assert_eq!(service.get_interval(), 30000);
        assert_eq!(service.get_debounce(), 2000);
    }

    #[test]
    fn test_mark_dirty() {
        let mut service = AutosaveService::new(PathBuf::from("/test"));
        let doc_id = Uuid::new_v4();

        service.mark_dirty(doc_id, 12345);
        assert!(service.needs_autosave(doc_id));
    }

    #[test]
    fn test_record_autosave() {
        let mut service = AutosaveService::new(PathBuf::from("/test"));
        let doc_id = Uuid::new_v4();

        service.mark_dirty(doc_id, 12345);
        assert!(service.needs_autosave(doc_id));

        service.record_autosave(doc_id, 12345);
        assert!(!service.needs_autosave(doc_id));
    }

    #[test]
    fn test_remove_document() {
        let mut service = AutosaveService::new(PathBuf::from("/test"));
        let doc_id = Uuid::new_v4();

        service.mark_dirty(doc_id, 12345);
        assert!(service.is_tracked(doc_id));

        service.remove_document(doc_id);
        assert!(!service.is_tracked(doc_id));
    }

    #[test]
    fn test_set_config() {
        let mut service = AutosaveService::new(PathBuf::from("/test"));

        service.set_config(AutosaveConfig {
            enabled: false,
            interval_ms: 60000,
            debounce_ms: 5000,
        });

        assert!(!service.is_enabled());
        assert_eq!(service.get_interval(), 60000);
        assert_eq!(service.get_debounce(), 5000);
    }

    #[test]
    fn test_compute_content_hash() {
        let hash1 = compute_content_hash("Hello World");
        let hash2 = compute_content_hash("Hello World");
        let hash3 = compute_content_hash("Different content");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_tracked_documents() {
        let mut service = AutosaveService::new(PathBuf::from("/test"));
        let doc1 = Uuid::new_v4();
        let doc2 = Uuid::new_v4();

        service.mark_dirty(doc1, 1);
        service.mark_dirty(doc2, 2);

        let tracked = service.tracked_documents();
        assert_eq!(tracked.len(), 2);
        assert!(tracked.contains(&doc1));
        assert!(tracked.contains(&doc2));
    }
}
