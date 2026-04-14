use crate::model::export::{HtmlExportOptions, PdfExportOptions};
use crate::model::{FileEntry, Workspace};
use crate::services::export::ExportResult;
use std::fs;
use std::path::PathBuf;

pub type WorkspaceResult<T> = Result<T, WorkspaceServiceError>;

#[derive(Debug, Clone)]
pub enum WorkspaceServiceError {
    PathNotFound(String),
    Io(String),
    Parse(String),
}

impl std::error::Error for WorkspaceServiceError {}

impl std::fmt::Display for WorkspaceServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspaceServiceError::PathNotFound(s) => write!(f, "Path not found: {}", s),
            WorkspaceServiceError::Io(s) => write!(f, "IO error: {}", s),
            WorkspaceServiceError::Parse(s) => write!(f, "Parse error: {}", s),
        }
    }
}

impl From<std::io::Error> for WorkspaceServiceError {
    fn from(e: std::io::Error) -> Self {
        WorkspaceServiceError::Io(e.to_string())
    }
}

pub trait WorkspaceServiceTrait: Send + Sync {
    fn list_workspace(&self, path: &str) -> WorkspaceResult<Workspace>;
    fn read_dir(&self, path: &str) -> WorkspaceResult<Vec<FileEntry>>;
    fn get_workspace_root(&self) -> Option<String>;
    fn set_workspace_root(&mut self, path: String);
}

pub struct WorkspaceService {
    root_path: Option<String>,
}

impl WorkspaceService {
    pub fn new() -> Self {
        Self { root_path: None }
    }
}

impl Default for WorkspaceService {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceServiceTrait for WorkspaceService {
    fn list_workspace(&self, path: &str) -> WorkspaceResult<Workspace> {
        let mut workspace = Workspace::new(path.to_string());
        workspace.files = self.read_dir_recursive(path)?;
        Ok(workspace)
    }

    fn read_dir(&self, path: &str) -> WorkspaceResult<Vec<FileEntry>> {
        self.read_dir_recursive(path)
    }

    fn get_workspace_root(&self) -> Option<String> {
        self.root_path.clone()
    }

    fn set_workspace_root(&mut self, path: String) {
        self.root_path = Some(path);
    }
}

impl WorkspaceService {
    fn read_dir_recursive(&self, path: &str) -> WorkspaceResult<Vec<FileEntry>> {
        let mut entries = vec![];

        let read_dir = fs::read_dir(path).map_err(|e| WorkspaceServiceError::Io(e.to_string()))?;

        for entry in read_dir {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let file_path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            if file_name.starts_with('.') || file_name == "node_modules" || file_name == "target" {
                continue;
            }

            if file_path.is_dir() {
                let children = self
                    .read_dir_recursive(file_path.to_str().unwrap_or(""))
                    .unwrap_or_default();
                entries.push(FileEntry::new_directory(
                    file_name,
                    file_path.to_string_lossy().to_string(),
                    children,
                ));
            } else if file_name.ends_with(".md") {
                entries.push(FileEntry::new_file(
                    file_name,
                    file_path.to_string_lossy().to_string(),
                ));
            }
        }

        Ok(entries)
    }
}
