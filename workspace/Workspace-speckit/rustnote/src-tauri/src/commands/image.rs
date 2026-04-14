use crate::model::{copy_image_to_workspace, get_image_info, save_image_from_base64, ImageInfo};
use std::path::{Path, PathBuf};
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

/// Decode URL-encoded characters in a path string (e.g., `%20` → space).
pub fn url_decode_path(path: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = path.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '%' && i + 2 < chars.len() {
            let hex: String = chars[i + 1..i + 3].iter().collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
                i += 3;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

pub fn resolve_relative_path(doc_dir: &Path, image_ref: &str) -> PathBuf {
    let decoded_ref = url_decode_path(image_ref);
    let normalized_ref = decoded_ref.replace('\\', "/");
    let relative_path = Path::new(&normalized_ref);

    let mut components: Vec<std::path::Component> = doc_dir.components().collect();

    for component in relative_path.components() {
        match component {
            std::path::Component::ParentDir => {
                if components.len() > 1 {
                    let last = *components.last().unwrap();
                    if !matches!(
                        last,
                        std::path::Component::Prefix(_) | std::path::Component::RootDir
                    ) {
                        components.pop();
                    }
                }
            }
            std::path::Component::Normal(s) => {
                components.push(std::path::Component::Normal(s));
            }
            std::path::Component::CurDir => {}
            _ => {}
        }
    }

    components.iter().collect()
}