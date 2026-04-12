pub mod document;
pub mod editor;
pub mod export;
pub mod file_tree;
pub mod file_watcher;
pub mod image;
pub mod recovery;
pub mod render;
pub mod settings;
pub mod workspace;

pub use document::*;
pub use editor::*;
pub use export::*;
pub use file_tree::*;
pub use file_watcher::*;
pub use image::*;
pub use recovery::*;
pub use render::*;
pub use settings::*;
pub use workspace::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Document not found: {0}")]
    DocumentNotFound(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("URL open error: {0}")]
    UrlOpenError(String),
}

impl serde::Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[tauri::command]
pub fn open_external_url(url: String) -> Result<(), CommandError> {
    open::that(&url).map_err(|e| CommandError::UrlOpenError(e.to_string()))?;
    Ok(())
}