use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub children: Vec<FileEntry>,
}

impl FileEntry {
    pub fn new_file(name: String, path: String) -> Self {
        Self {
            name,
            path,
            is_directory: false,
            children: vec![],
        }
    }

    pub fn new_directory(name: String, path: String, children: Vec<FileEntry>) -> Self {
        Self {
            name,
            path,
            is_directory: true,
            children,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub root_path: String,
    pub name: String,
    pub files: Vec<FileEntry>,
}

impl Workspace {
    pub fn new(root_path: String) -> Self {
        let name = std::path::Path::new(&root_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Workspace")
            .to_string();

        Self {
            id: Uuid::new_v4(),
            root_path,
            name,
            files: vec![],
        }
    }
}
