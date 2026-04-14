use std::path::PathBuf;
use uuid::Uuid;

pub mod autosave;
pub mod document;
pub mod editor;
pub mod export;
pub mod file_watcher;
pub mod recovery;
pub mod settings;
pub mod workspace;

pub use autosave::{
    compute_content_hash, AutosaveConfig, AutosaveService, AutosaveServiceTrait, BackupFile,
    RustAutosaveManager,
};
pub use document::DocumentService;
pub use editor::EditorService;
pub use export::{ExportResult, ExportService, ExportServiceError, ExportServiceTrait};
pub use file_watcher::FileWatcherService;
pub use recovery::{RecoveryResult, RecoveryService, RecoveryServiceError, RecoveryServiceTrait};
pub use settings::SettingsService;
pub use workspace::{
    WorkspaceResult, WorkspaceService, WorkspaceServiceError, WorkspaceServiceTrait,
};

pub type DocumentResult<T> = Result<T, DocumentServiceError>;

#[derive(Debug, Clone)]
pub enum DocumentServiceError {
    NotFound(String),
    Io(String),
    Serialization(String),
}

impl std::error::Error for DocumentServiceError {}

impl std::fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentServiceError::NotFound(s) => write!(f, "Document not found: {}", s),
            DocumentServiceError::Io(s) => write!(f, "IO error: {}", s),
            DocumentServiceError::Serialization(s) => write!(f, "Serialization error: {}", s),
        }
    }
}

impl From<std::io::Error> for DocumentServiceError {
    fn from(e: std::io::Error) -> Self {
        DocumentServiceError::Io(e.to_string())
    }
}

impl From<serde_json::Error> for DocumentServiceError {
    fn from(e: serde_json::Error) -> Self {
        DocumentServiceError::Serialization(e.to_string())
    }
}

pub type EditorResult<T> = Result<T, EditorServiceError>;

#[derive(Debug, Clone, PartialEq)]
pub enum EditorServiceError {
    NotFound(String),
    InvalidPosition,
}

impl std::error::Error for EditorServiceError {}

impl std::fmt::Display for EditorServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorServiceError::NotFound(s) => write!(f, "Editor not found: {}", s),
            EditorServiceError::InvalidPosition => write!(f, "Invalid position"),
        }
    }
}

pub type FileWatcherResult<T> = Result<T, FileWatcherServiceError>;

#[derive(Debug, Clone)]
pub enum FileWatcherServiceError {
    PathNotFound(String),
    WatcherError(String),
}

impl std::error::Error for FileWatcherServiceError {}

impl std::fmt::Display for FileWatcherServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileWatcherServiceError::PathNotFound(s) => write!(f, "Path does not exist: {}", s),
            FileWatcherServiceError::WatcherError(s) => write!(f, "Watcher error: {}", s),
        }
    }
}

pub type SettingsResult<T> = Result<T, SettingsServiceError>;

#[derive(Debug, Clone)]
pub enum SettingsServiceError {
    Database(String),
    Serialization(String),
    Io(String),
}

impl std::error::Error for SettingsServiceError {}

impl std::fmt::Display for SettingsServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsServiceError::Database(s) => write!(f, "Database error: {}", s),
            SettingsServiceError::Serialization(s) => write!(f, "Serialization error: {}", s),
            SettingsServiceError::Io(s) => write!(f, "IO error: {}", s),
        }
    }
}

impl From<serde_json::Error> for SettingsServiceError {
    fn from(e: serde_json::Error) -> Self {
        SettingsServiceError::Serialization(e.to_string())
    }
}

impl From<std::io::Error> for SettingsServiceError {
    fn from(e: std::io::Error) -> Self {
        SettingsServiceError::Io(e.to_string())
    }
}

pub trait DocumentServiceTrait: Send + Sync {
    fn create(&mut self, title: String) -> DocumentResult<crate::model::Document>;
    fn open(&mut self, path: PathBuf) -> DocumentResult<crate::model::Document>;
    fn save(&mut self, id: Uuid, path: Option<PathBuf>) -> DocumentResult<()>;
    fn get_content(&self, id: Uuid) -> Option<String>;
    fn update_source(&mut self, id: Uuid, source: String) -> DocumentResult<()>;
    fn get(&self, id: Uuid) -> Option<crate::model::Document>;
    fn list_documents(&self) -> Vec<Uuid>;
    fn close(&mut self, id: Uuid) -> Option<crate::model::Document>;
}

pub trait EditorServiceTrait: Send + Sync {
    fn cursor(&self, id: Uuid) -> Option<crate::semantic::ast::Position>;
    fn set_cursor(&mut self, id: Uuid, pos: crate::semantic::ast::Position);
    fn selection(&self, id: Uuid) -> Option<crate::semantic::position::Selection>;
    fn set_selection(&mut self, id: Uuid, sel: crate::semantic::position::Selection);
    fn clear_selection(&mut self, id: Uuid);
    fn is_anchor_mode(&self, id: Uuid) -> bool;
    fn set_anchor_mode(&mut self, id: Uuid, mode: bool);
    fn start_selection(&mut self, id: Uuid, pos: crate::semantic::ast::Position);
    fn update_selection(&mut self, id: Uuid, head: crate::semantic::ast::Position);
    fn move_cursor(&mut self, id: Uuid, pos: crate::semantic::ast::Position);
    fn has_selection(&self, id: Uuid) -> bool;
    fn selected_range(
        &self,
        id: Uuid,
    ) -> Option<(
        crate::semantic::ast::Position,
        crate::semantic::ast::Position,
    )>;
    fn remove_editor(&mut self, id: Uuid);
}

pub trait FileWatcherServiceTrait: Send + Sync {
    fn watch(&mut self, path: &str) -> FileWatcherResult<()>;
    fn unwatch(&mut self, path: &str) -> FileWatcherResult<()>;
    fn poll_changes(&self) -> Vec<crate::services::file_watcher::FileChange>;
    fn check_file_changed(&self, path: &str) -> Option<bool>;
    fn update_file_state(&self, path: &str);
    fn is_watching(&self) -> bool;
}

pub trait SettingsServiceTrait: Send + Sync {
    fn read_settings(&self) -> SettingsResult<crate::model::Settings>;
    fn write_settings(&self, settings: &crate::model::Settings) -> SettingsResult<()>;
    fn update_setting(&self, key: &str, value: &str) -> SettingsResult<()>;
    fn get_setting(&self, key: &str) -> SettingsResult<Option<String>>;
    fn add_recent_file(&self, path: &str) -> SettingsResult<()>;
    fn get_recent_files(&self) -> SettingsResult<Vec<String>>;
    fn clear_recent_files(&self) -> SettingsResult<()>;
    fn add_recent_folder(&self, path: &str) -> SettingsResult<()>;
    fn get_recent_folders(&self) -> SettingsResult<Vec<String>>;
    fn clear_recent_folders(&self) -> SettingsResult<()>;
    fn set_workspace_state(&self, key: &str, value: &str) -> SettingsResult<()>;
    fn get_workspace_state(&self, key: &str) -> SettingsResult<Option<String>>;
    fn delete_workspace_state(&self, key: &str) -> SettingsResult<()>;
    fn verify_integrity(&self) -> SettingsResult<bool>;
}
