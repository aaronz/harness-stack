use crate::model::{copy_image_to_workspace, get_image_info, save_image_from_base64, ImageInfo};
use std::path::Path;
use tauri::command;

#[command]
pub async fn insert_image(
    image_path: String,
    workspace_path: Option<String>,
) -> Result<ImageInfo, String> {
    let source = Path::new(&image_path);
    
    if !source.exists() {
        return Err(format!("Image not found: {}", image_path));
    }
    
    let info = if let Some(ws) = workspace_path {
        let ws_path = Path::new(&ws);
        copy_image_to_workspace(source, ws_path).map_err(|e| e.to_string())?
    } else {
        get_image_info(source).map_err(|e| e.to_string())?
    };
    
    Ok(info)
}

#[command]
pub async fn save_image_from_base64_cmd(
    base64_data: String,
    workspace_path: String,
) -> Result<ImageInfo, String> {
    let ws_path = Path::new(&workspace_path);
    save_image_from_base64(&base64_data, ws_path).map_err(|e| e.to_string())
}

#[command]
pub fn image_markdown_from_path(image_path: String, relative_path: String) -> String {
    let file_name = Path::new(&image_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "image".to_string());
    format!("![{}]({})", file_name, relative_path)
}