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

/// Calculate the relative path from a document's directory to an image path.
/// 
/// This function takes:
/// - `doc_path`: The path to the document (can be absolute or just a filename)
/// - `image_path`: The absolute path to the image
/// 
/// Returns the relative path that should be used in Markdown syntax.
/// 
/// Examples:
/// - Document at `/workspace/project/notes/chapter.md`, image at `/workspace/project/assets/diagram.png`
///   Returns: `../assets/diagram.png`
/// - Document at `/workspace/project/notes/sub/chapter.md`, image at `/workspace/project/assets/img.png`
///   Returns: `../../assets/img.png`
/// - Document and image in same directory
///   Returns: just the filename
pub fn calculate_relative_path(doc_path: &Path, image_path: &Path) -> String {
    let doc_dir = doc_path.parent().unwrap_or(Path::new("."));
    let doc_dir_str = doc_dir.to_string_lossy();
    let img_str = image_path.to_string_lossy();
    
    let doc_parts: Vec<&str> = doc_dir_str.split(|c| c == '/' || c == '\\').filter(|s| !s.is_empty()).collect();
    let img_parts: Vec<&str> = img_str.split(|c| c == '/' || c == '\\').filter(|s| !s.is_empty()).collect();
    
    let common_len = doc_parts.iter()
        .zip(img_parts.iter())
        .take_while(|(a, b)| *a == *b)
        .count();
    
    let up_count = doc_parts.len() - common_len;
    
    let mut relative = String::new();
    
    for _ in 0..up_count {
        relative.push_str("../");
    }
    
    for (i, part) in img_parts[common_len..].iter().enumerate() {
        if i > 0 || (!relative.is_empty() && !relative.ends_with('/')) {
            relative.push('/');
        }
        relative.push_str(part);
    }
    
    if relative.is_empty() {
        relative.push('.');
    }
    
    relative
}

/// Format a path for use in Markdown, handling special characters.
/// Uses forward slashes for cross-platform compatibility.
pub fn format_markdown_path(path: &str) -> String {
    // Replace backslashes with forward slashes for cross-platform
    let normalized = path.replace('\\', "/");
    
    // URL-encode spaces and other special characters that might cause issues
    let mut result = String::new();
    for c in normalized.chars() {
        match c {
            ' ' => result.push_str("%20"),
            '#' => result.push_str("%23"),
            '%' => result.push_str("%25"),
            '&' => result.push_str("%26"),
            '?' => result.push_str("%3F"),
            '\'' => result.push_str("%27"),
            '"' => result.push_str("%22"),
            '<' => result.push_str("%3C"),
            '>' => result.push_str("%3E"),
            '[' => result.push_str("%5B"),
            ']' => result.push_str("%5D"),
            '^' => result.push_str("%5E"),
            '`' => result.push_str("%60"),
            '{' => result.push_str("%7B"),
            '|' => result.push_str("%7C"),
            '}' => result.push_str("%7D"),
            // For non-ASCII characters, use percent encoding
            c if c.is_ascii() => result.push(c),
            c => {
                // URL-encode non-ASCII characters
                for byte in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    
    result
}