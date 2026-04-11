use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
}

impl FileNode {
    pub fn new_file(path: PathBuf, name: String) -> Self {
        Self {
            path,
            name,
            is_dir: false,
            children: Vec::new(),
        }
    }

    pub fn new_dir(path: PathBuf, name: String) -> Self {
        Self {
            path,
            name,
            is_dir: true,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub root_path: PathBuf,
    pub tree: FileNode,
    pub active_file: Option<PathBuf>,
}

impl Workspace {
    pub fn new(root_path: PathBuf, tree: FileNode) -> Self {
        Self {
            root_path,
            tree,
            active_file: None,
        }
    }
}
