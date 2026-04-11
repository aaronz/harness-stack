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
