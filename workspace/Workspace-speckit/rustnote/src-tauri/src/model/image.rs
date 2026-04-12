use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image not found: {0}")]
    NotFound(String),
    #[error("Invalid image: {0}")]
    InvalidImage(String),
}

impl serde::Serialize for ImageError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub path: String,
    pub relative_path: String,
    pub file_name: String,
    pub size: u64,
}

/// Copy an image to the workspace's assets folder and return the relative path
pub fn copy_image_to_workspace(
    source_path: &Path,
    workspace_path: &Path,
) -> Result<ImageInfo, ImageError> {
    // Create assets directory if it doesn't exist
    let assets_dir = workspace_path.join("assets").join("images");
    fs::create_dir_all(&assets_dir)?;

    // Generate unique filename to avoid collisions
    let file_name = source_path
        .file_name()
        .ok_or_else(|| ImageError::InvalidImage("Invalid file name".to_string()))?;

    let extension = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");

    let unique_name = format!(
        "{}_{}.{}",
        uuid::Uuid::new_v4(),
        file_name
            .to_string_lossy()
            .replace(&format!(".{}", extension), ""),
        extension
    );

    let dest_path = assets_dir.join(&unique_name);

    // Copy the file
    fs::copy(source_path, &dest_path)?;

    // Get relative path from workspace root
    let relative_path = dest_path
        .strip_prefix(workspace_path)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| unique_name.clone());

    let size = fs::metadata(&dest_path)?.len();

    Ok(ImageInfo {
        path: dest_path.to_string_lossy().to_string(),
        relative_path,
        file_name: unique_name,
        size,
    })
}

/// Get image info from a path
pub fn get_image_info(path: &Path) -> Result<ImageInfo, ImageError> {
    if !path.exists() {
        return Err(ImageError::NotFound(path.to_string_lossy().to_string()));
    }

    let metadata = fs::metadata(path)?;
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    Ok(ImageInfo {
        path: path.to_string_lossy().to_string(),
        relative_path: file_name.clone(),
        file_name,
        size: metadata.len(),
    })
}

/// Save a base64-encoded image to the workspace assets folder and return the relative path
pub fn save_image_from_base64(
    base64_data: &str,
    workspace_path: &Path,
) -> Result<ImageInfo, ImageError> {
    // Create assets directory if it doesn't exist
    let assets_dir = workspace_path.join("assets").join("images");
    fs::create_dir_all(&assets_dir)?;

    // Decode the base64 data
    let image_data = if base64_data.contains(',') {
        // Handle data URL format: data:image/png;base64,xxxxx
        let parts: Vec<&str> = base64_data.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err(ImageError::InvalidImage(
                "Invalid base64 data URL format".to_string(),
            ));
        }
        // Parse the MIME type to determine extension
        let mime_part = parts[0];
        let data = parts[1];

        // Extract extension from mime type (e.g., "image/png" -> "png")
        let extension = if mime_part.contains("image/png") {
            "png"
        } else if mime_part.contains("image/jpeg") || mime_part.contains("image/jpg") {
            "jpg"
        } else if mime_part.contains("image/gif") {
            "gif"
        } else if mime_part.contains("image/webp") {
            "webp"
        } else if mime_part.contains("image/svg") {
            "svg"
        } else {
            "png" // default
        };

        let decoded = general_purpose::STANDARD
            .decode(data)
            .map_err(|e| ImageError::InvalidImage(format!("Base64 decode error: {}", e)))?;

        (decoded, extension.to_string())
    } else {
        // Simple base64 without data URL
        let decoded = general_purpose::STANDARD
            .decode(base64_data)
            .map_err(|e| ImageError::InvalidImage(format!("Base64 decode error: {}", e)))?;
        (decoded, "png".to_string())
    };

    let (decoded, extension) = image_data;

    // Generate unique filename
    let unique_name = format!("{}.{}", uuid::Uuid::new_v4(), extension);

    let dest_path = assets_dir.join(&unique_name);

    // Write the file
    fs::write(&dest_path, &decoded)?;

    // Get relative path from workspace root
    let relative_path = dest_path
        .strip_prefix(workspace_path)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| unique_name.clone());

    let size = decoded.len() as u64;

    Ok(ImageInfo {
        path: dest_path.to_string_lossy().to_string(),
        relative_path,
        file_name: unique_name,
        size,
    })
}
