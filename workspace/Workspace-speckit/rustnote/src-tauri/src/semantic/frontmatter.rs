//! Frontmatter parsing module for YAML frontmatter support
//!
//! This module provides structured parsing of YAML frontmatter using serde_yaml.
//! It handles complex YAML structures including nested objects and arrays,
//! while gracefully handling invalid YAML and missing frontmatter.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during frontmatter parsing
#[derive(Error, Debug, PartialEq)]
pub enum FrontmatterError {
    #[error("Invalid YAML syntax: {0}")]
    InvalidYaml(String),

    #[error("Frontmatter exceeds reasonable depth limit")]
    DepthLimitExceeded,

    #[error("Failed to serialize frontmatter: {0}")]
    SerializationError(String),
}

/// YAML frontmatter structure that supports complex YAML parsing
///
/// This struct can deserialize from any valid YAML frontmatter and provides
/// access to common fields like title, tags, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct YamlFrontmatter {
    /// Document title
    #[serde(default)]
    pub title: Option<String>,

    /// Document author
    #[serde(default)]
    pub author: Option<String>,

    /// Document date in YYYY-MM-DD format
    #[serde(default)]
    pub date: Option<String>,

    /// List of tags
    #[serde(default)]
    pub tags: Vec<String>,

    /// Custom metadata section for arbitrary key-value pairs
    #[serde(flatten, default)]
    pub metadata: serde_yaml::Value,
}

impl YamlFrontmatter {
    /// Parse YAML frontmatter from a string
    ///
    /// # Arguments
    /// * `input` - Raw YAML string (without --- markers)
    ///
    /// # Returns
    /// * `Ok(YamlFrontmatter)` on successful parse
    /// * `Err(FrontmatterError)` on invalid YAML
    pub fn parse(input: &str) -> Result<Self, FrontmatterError> {
        if input.trim().is_empty() {
            return Ok(YamlFrontmatter::default());
        }

        serde_yaml::from_str(input).map_err(|e| FrontmatterError::InvalidYaml(e.to_string()))
    }

    /// Get a string value from metadata by key
    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.metadata.get(key)?.as_str()
    }

    /// Get a nested value from metadata
    pub fn get_nested(&self, key: &str) -> Option<&serde_yaml::Value> {
        self.metadata.get(key)
    }

    /// Convert back to YAML string (without --- markers)
    pub fn to_yaml_string(&self) -> Result<String, FrontmatterError> {
        serde_yaml::to_string(self).map_err(|e| FrontmatterError::SerializationError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_frontmatter() {
        let result = YamlFrontmatter::parse("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), YamlFrontmatter::default());
    }

    #[test]
    fn test_parse_simple_frontmatter() {
        let yaml = "title: Hello\nauthor: Test";
        let result = YamlFrontmatter::parse(yaml).unwrap();
        assert_eq!(result.title, Some("Hello".to_string()));
        assert_eq!(result.author, Some("Test".to_string()));
    }

    #[test]
    fn test_parse_frontmatter_with_tags() {
        let yaml = "title: Test\ntags:\n  - a\n  - b";
        let result = YamlFrontmatter::parse(yaml).unwrap();
        assert_eq!(result.title, Some("Test".to_string()));
        assert_eq!(result.tags, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let yaml = "title: [unclosed";
        let result = YamlFrontmatter::parse(yaml);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FrontmatterError::InvalidYaml(_)
        ));
    }

    #[test]
    fn test_roundtrip_frontmatter() {
        let yaml = "title: Roundtrip Test\nauthor: Tester";
        let parsed = YamlFrontmatter::parse(yaml).unwrap();
        let serialized = parsed.to_yaml_string().unwrap();
        let reparsed = YamlFrontmatter::parse(&serialized).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
