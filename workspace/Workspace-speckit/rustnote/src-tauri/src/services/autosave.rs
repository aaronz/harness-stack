use crate::model::{Document, RecoverySnapshot};
use crate::services::{DocumentResult, DocumentServiceError};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

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
            elapsed >= Duration::from_millis(self.config.debounce_ms)
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
