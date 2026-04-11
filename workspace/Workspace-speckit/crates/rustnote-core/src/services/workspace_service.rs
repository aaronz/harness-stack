use crate::core::workspace::{FileNode, Workspace};
use crate::error::Result;
use std::path::Path;
use walkdir::WalkDir;

pub struct WorkspaceService;

impl WorkspaceService {
    pub fn open(root: &Path) -> Result<Workspace> {
        let tree = Self::build_file_tree(root, root)?;
        Ok(Workspace::new(root.to_path_buf(), tree))
    }

    fn build_file_tree(root: &Path, current: &Path) -> Result<FileNode> {
        let name = current
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| root.to_string_lossy().to_string());

        if current.is_file() {
            return Ok(FileNode::new_file(current.to_path_buf(), name));
        }

        let mut children = Vec::new();
        if let Ok(entries) = std::fs::read_dir(current) {
            let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

            for entry in entries {
                let path = entry.path();
                let file_name = entry.file_name().to_string_lossy().to_string();
                if path.is_dir() && !file_name.starts_with('.') {
                    children.push(Self::build_file_tree(root, &path)?);
                } else if path.is_file() && file_name.ends_with(".md") {
                    children.push(FileNode::new_file(path, file_name));
                }
            }
        }

        Ok(FileNode {
            path: current.to_path_buf(),
            name,
            is_dir: true,
            children,
        })
    }

    pub fn create_file(parent: &Path, name: &str) -> Result<std::path::PathBuf> {
        let file_path = parent.join(name);
        std::fs::write(&file_path, "")?;
        Ok(file_path)
    }

    pub fn rename_file(path: &Path, new_name: &str) -> Result<()> {
        let new_path = path.parent().unwrap().join(new_name);
        std::fs::rename(path, &new_path)?;
        Ok(())
    }

    pub fn delete_file(path: &Path) -> Result<()> {
        if path.is_dir() {
            std::fs::remove_dir_all(path)?;
        } else {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }
}
