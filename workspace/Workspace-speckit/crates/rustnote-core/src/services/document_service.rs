use crate::core::{Document, EditorState};
use crate::error::{Error, Result};
use crate::parser::{parse_markdown, Serializer};
use std::fs;
use std::path::Path;

pub struct DocumentService;

impl DocumentService {
    pub fn open(path: &Path) -> Result<(String, Document)> {
        let content = fs::read_to_string(path)?;
        let doc = parse_markdown(&content);
        Ok((content, doc))
    }

    pub fn save(path: &Path, content: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(())
    }

    pub fn reload(path: &Path) -> Result<String> {
        fs::read_to_string(path).map_err(|e| e.into())
    }

    pub fn render_html(content: &str) -> String {
        let doc = parse_markdown(content);
        Serializer::to_html(&doc)
    }

    pub fn get_outline(content: &str) -> Vec<OutlineItem> {
        let doc = parse_markdown(content);
        crate::parser::SemanticTree::get_headings(&doc)
            .into_iter()
            .map(|h| OutlineItem {
                level: h.level,
                text: h.text,
            })
            .collect()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OutlineItem {
    pub level: u8,
    pub text: String,
}
