use crate::model::{Workspace, FileEntry};
use crate::commands::CommandError;
use std::fs;

fn read_dir_recursive(path: &str) -> Result<Vec<FileEntry>, CommandError> {
    let mut entries = vec![];
    
    let read_dir = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(e) => return Err(CommandError::Io(e)),
    };
    
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
            let children = read_dir_recursive(file_path.to_str().unwrap_or("")).unwrap_or_default();
            entries.push(FileEntry::new_directory(file_name, file_path.to_string_lossy().to_string(), children));
        } else if file_name.ends_with(".md") {
            entries.push(FileEntry::new_file(file_name, file_path.to_string_lossy().to_string()));
        }
    }
    
    Ok(entries)
}

#[tauri::command]
pub async fn list_workspace(path: String) -> Result<Workspace, CommandError> {
    let mut workspace = Workspace::new(path.clone());
    workspace.files = read_dir_recursive(&path)?;
    Ok(workspace)
}