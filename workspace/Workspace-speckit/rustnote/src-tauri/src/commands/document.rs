use crate::model::Document;
use crate::commands::CommandError;
use std::fs;

#[tauri::command]
pub async fn create_document(title: String) -> Result<Document, CommandError> {
    Ok(Document::new(title))
}

#[tauri::command]
pub async fn open_document(path: String) -> Result<Document, CommandError> {
    let content = fs::read_to_string(&path)?;
    Document::from_file(&path, content).map_err(CommandError::Io)
}

#[tauri::command]
pub async fn save_document(path: String, content: String) -> Result<(), CommandError> {
    let temp_path = format!("{}.tmp", path);
    fs::write(&temp_path, &content)?;
    fs::rename(&temp_path, &path)?;
    Ok(())
}

#[tauri::command]
pub async fn read_document_content(path: String) -> Result<String, CommandError> {
    fs::read_to_string(&path).map_err(CommandError::Io)
}

#[tauri::command]
pub async fn write_document_content(path: String, content: String) -> Result<(), CommandError> {
    fs::write(&path, &content).map_err(CommandError::Io)
}