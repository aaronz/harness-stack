use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::semantic::ast::SemanticDocument;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub file_path: Option<String>,
    pub is_dirty: bool,
    pub last_saved: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub headings: Vec<Heading>,
}

impl Document {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            content: String::new(),
            file_path: None,
            is_dirty: true,
            last_saved: None,
            created_at: now,
            updated_at: now,
            headings: Vec::new(),
        }
    }

    pub fn from_file(path: &str, content: String) -> Result<Self, std::io::Error> {
        let title = std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let now = Utc::now();
        let headings = Self::extract_headings_from_content(&content);
        Ok(Self {
            id: Uuid::new_v4(),
            title,
            content,
            file_path: Some(path.to_string()),
            is_dirty: false,
            last_saved: Some(now),
            created_at: now,
            updated_at: now,
            headings,
        })
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.is_dirty = true;
        self.updated_at = Utc::now();
        self.refresh_headings();
    }

    pub fn refresh_headings(&mut self) {
        self.headings = Self::extract_headings_from_content(&self.content);
    }

    pub fn mark_saved(&mut self) {
        self.is_dirty = false;
        self.last_saved = Some(Utc::now());
    }

    /// Extract headings from Markdown content using the semantic layer.
    pub fn extract_headings_from_content(content: &str) -> Vec<Heading> {
        let sem_doc = SemanticDocument::parse(content);
        sem_doc
            .get_headings()
            .into_iter()
            .map(|hi| Heading {
                level: hi.level,
                text: hi.text,
                position: hi.offset,
            })
            .collect()
    }
}

/// Heading extracted from a Markdown document.
/// Used for outline/TOC generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heading {
    /// Heading level (1-6)
    pub level: u8,
    /// Heading text content
    pub text: String,
    /// Character offset position in the document
    pub position: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_document() {
        let doc = Document::new("Test".to_string());
        assert_eq!(doc.title, "Test");
        assert!(doc.content.is_empty());
        assert!(doc.is_dirty);
    }

    #[test]
    fn test_update_content() {
        let mut doc = Document::new("Test".to_string());
        doc.update_content("# Hello".to_string());
        assert_eq!(doc.content, "# Hello");
        assert!(doc.is_dirty);
    }

    #[test]
    fn test_mark_saved() {
        let mut doc = Document::new("Test".to_string());
        doc.update_content("content".to_string());
        doc.mark_saved();
        assert!(!doc.is_dirty);
        assert!(doc.last_saved.is_some());
    }
}
