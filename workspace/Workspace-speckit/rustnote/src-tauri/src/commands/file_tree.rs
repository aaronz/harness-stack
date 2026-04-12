use crate::commands::CommandError;
use std::fs;
use std::path::Path;

fn validate_path(path: &str) -> Result<(), CommandError> {
    let p = Path::new(path);
    
    if path.contains("..") {
        return Err(CommandError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Path traversal not allowed",
        )));
    }
    
    if !p.exists() {
        return Err(CommandError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Path does not exist: {}", path),
        )));
    }
    
    Ok(())
}

#[tauri::command]
pub async fn create_file(parent_path: String, name: String) -> Result<String, CommandError> {
    let file_path = Path::new(&parent_path).join(&name);
    let file_path_str = file_path.to_string_lossy().to_string();
    
    if !Path::new(&parent_path).exists() {
        return Err(CommandError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Parent directory does not exist: {}", parent_path),
        )));
    }
    
    if file_path.exists() {
        return Err(CommandError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("File already exists: {}", file_path_str),
        )));
    }
    
    fs::write(&file_path, "")?;
    
    Ok(file_path_str)
}

#[tauri::command]
pub async fn create_folder(parent_path: String, name: String) -> Result<String, CommandError> {
    let folder_path = Path::new(&parent_path).join(&name);
    let folder_path_str = folder_path.to_string_lossy().to_string();
    
    if !Path::new(&parent_path).exists() {
        return Err(CommandError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Parent directory does not exist: {}", parent_path),
        )));
    }
    
    if folder_path.exists() {
        return Err(CommandError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Folder already exists: {}", folder_path_str),
        )));
    }
    
    fs::create_dir(&folder_path)?;
    
    Ok(folder_path_str)
}

#[tauri::command]
pub async fn rename_item(old_path: String, new_name: String) -> Result<String, CommandError> {
    validate_path(&old_path)?;
    
    let old = Path::new(&old_path);
    let parent = old.parent().ok_or_else(|| {
        CommandError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Cannot get parent directory",
        ))
    })?;
    
    let new_path = parent.join(&new_name);
    let new_path_str = new_path.to_string_lossy().to_string();
    
    if new_path.exists() {
        return Err(CommandError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("An item with that name already exists: {}", new_path_str),
        )));
    }
    
    fs::rename(old, &new_path)?;
    
    Ok(new_path_str)
}

#[tauri::command]
pub async fn delete_item(path: String) -> Result<(), CommandError> {
    validate_path(&path)?;
    
    let p = Path::new(&path);
    
    if p.is_dir() {
        fs::remove_dir_all(p)?;
    } else {
        fs::remove_file(p)?;
    }
    
    Ok(())
}