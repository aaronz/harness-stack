//! Search engine for Find/Replace functionality
//!
//! Implements regex-based search with:
//! - Case sensitivity option
//! - Match list generation
//! - Position tracking for each match

use regex::Regex;
use serde::{Deserialize, Serialize};

/// Search options
#[derive(Debug, Clone, Deserialize)]
pub struct SearchOptions {
    /// Case sensitive search
    pub case_sensitive: bool,
    /// Use regex pattern (true) or literal search (false)
    pub use_regex: bool,
}

/// A single search match with position information
#[derive(Debug, Clone, Serialize)]
pub struct SearchMatch {
    /// Start position of the match
    pub start: usize,
    /// End position of the match
    pub end: usize,
    /// Matched text
    pub text: String,
    /// Line number (0-indexed)
    pub line: u32,
    /// Column number (0-indexed)
    pub column: u32,
}

/// Search result containing all matches
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    /// All matches found
    pub matches: Vec<SearchMatch>,
    /// Total number of matches
    pub count: usize,
    /// Whether the search was successful (even if 0 matches)
    pub success: bool,
    /// Error message if search failed
    pub error: Option<String>,
}

impl SearchResult {
    /// Create a successful search result
    pub fn success(matches: Vec<SearchMatch>) -> Self {
        let count = matches.len();
        Self {
            matches,
            count,
            success: true,
            error: None,
        }
    }

    /// Create a failed search result
    pub fn error(msg: String) -> Self {
        Self {
            matches: Vec::new(),
            count: 0,
            success: false,
            error: Some(msg),
        }
    }
}

/// Search engine for finding matches in document content
pub struct SearchEngine;

impl SearchEngine {
    /// Create a new search engine
    pub fn new() -> Self {
        Self
    }

    /// Search content for the given pattern
    pub fn search(&self, content: &str, pattern: &str, options: &SearchOptions) -> SearchResult {
        if pattern.is_empty() {
            return SearchResult::success(Vec::new());
        }

        // Build regex pattern
        let regex_pattern = if options.use_regex {
            pattern.to_string()
        } else {
            // Escape special regex characters for literal search
            regex::escape(pattern)
        };

        // Build regex with options
        let regex_result = if options.case_sensitive {
            Regex::new(&regex_pattern)
        } else {
            Regex::new(&format!("(?i){}", regex_pattern))
        };

        let regex = match regex_result {
            Ok(r) => r,
            Err(e) => return SearchResult::error(format!("Invalid regex pattern: {}", e)),
        };

        // Find all matches
        let mut matches = Vec::new();
        let mut offset_accumulator: usize = 0;

        for line in content.lines() {
            let line_start_offset = offset_accumulator;

            for mat in regex.find_iter(line) {
                let start = line_start_offset + mat.start();
                let end = line_start_offset + mat.end();

                // Calculate line and column from offset
                let (line_num, column_num) = Self::offset_to_line_column(content, start);

                matches.push(SearchMatch {
                    start,
                    end,
                    text: mat.as_str().to_string(),
                    line: line_num,
                    column: column_num,
                });
            }

            offset_accumulator += line.len() + 1; // +1 for newline
        }

        SearchResult::success(matches)
    }

    /// Convert an offset to line and column numbers
    fn offset_to_line_column(content: &str, offset: usize) -> (u32, u32) {
        let offset = offset.min(content.len());
        let mut line: u32 = 0;
        let mut column: u32 = 0;
        let mut current_offset: usize = 0;

        for ch in content.chars() {
            if current_offset >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
            current_offset += ch.len_utf8();
        }

        (line, column)
    }

    /// Find the next match after the given offset
    pub fn find_next(
        &self,
        content: &str,
        pattern: &str,
        options: &SearchOptions,
        after_offset: usize,
    ) -> Option<SearchMatch> {
        let result = self.search(content, pattern, options);
        result.matches.into_iter().find(|m| m.start > after_offset)
    }

    /// Find the previous match before the given offset
    pub fn find_previous(
        &self,
        content: &str,
        pattern: &str,
        options: &SearchOptions,
        before_offset: usize,
    ) -> Option<SearchMatch> {
        let result = self.search(content, pattern, options);
        result
            .matches
            .into_iter()
            .filter(|m| m.start < before_offset)
            .last()
    }

    /// Replace a single match at the given position
    pub fn replace_match(
        content: &str,
        match_start: usize,
        match_end: usize,
        replacement: &str,
    ) -> String {
        let mut result = String::new();
        result.push_str(&content[..match_start]);
        result.push_str(replacement);
        result.push_str(&content[match_end..]);
        result
    }

    /// Replace all matches with the given replacement
    pub fn replace_all(
        content: &str,
        pattern: &str,
        options: &SearchOptions,
        replacement: &str,
    ) -> SearchResult {
        let engine = SearchEngine::new();
        let result = engine.search(content, pattern, options);

        if !result.success {
            return result;
        }

        // Replace all matches from end to start to preserve offsets
        let mut replacements: Vec<(usize, usize, &str)> = result
            .matches
            .iter()
            .map(|m| (m.start, m.end, replacement))
            .collect();

        // Sort by start position descending to replace from end
        replacements.sort_by(|a, b| b.0.cmp(&a.0));

        let mut result_content = content.to_string();
        for (start, end, rep) in replacements {
            result_content = Self::replace_match(&result_content, start, end, rep);
        }

        SearchResult::success(vec![SearchMatch {
            start: 0,
            end: result_content.len(),
            text: result_content.clone(),
            line: 0,
            column: 0,
        }])
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_search() {
        let engine = SearchEngine::new();
        let content = "Hello World\nHello Rust\nHello";
        let options = SearchOptions {
            case_sensitive: true,
            use_regex: false,
        };

        let result = engine.search(content, "Hello", &options);
        assert_eq!(result.count, 3);
        assert!(result.success);
    }

    #[test]
    fn test_case_insensitive_search() {
        let engine = SearchEngine::new();
        let content = "Hello WORLD\nhello rust\nHELLO";
        let options = SearchOptions {
            case_sensitive: false,
            use_regex: false,
        };

        let result = engine.search(content, "hello", &options);
        assert_eq!(result.count, 3);
    }

    #[test]
    fn test_regex_search() {
        let engine = SearchEngine::new();
        let content = "test123\ntest456\ndemo789";
        let options = SearchOptions {
            case_sensitive: true,
            use_regex: true,
        };

        let result = engine.search(content, r"test\d+", &options);
        assert_eq!(result.count, 2);
    }

    #[test]
    fn test_regex_case_insensitive() {
        let engine = SearchEngine::new();
        let content = "TEST123\ntest456";
        let options = SearchOptions {
            case_sensitive: false,
            use_regex: true,
        };

        let result = engine.search(content, r"test\d+", &options);
        assert_eq!(result.count, 2);
    }

    #[test]
    fn test_empty_pattern() {
        let engine = SearchEngine::new();
        let content = "Hello World";
        let options = SearchOptions {
            case_sensitive: true,
            use_regex: false,
        };

        let result = engine.search(content, "", &options);
        assert_eq!(result.count, 0);
        assert!(result.success);
    }

    #[test]
    fn test_invalid_regex() {
        let engine = SearchEngine::new();
        let content = "Hello World";
        let options = SearchOptions {
            case_sensitive: true,
            use_regex: true,
        };

        let result = engine.search(content, "[invalid", &options);
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_match_positions() {
        let engine = SearchEngine::new();
        let content = "Hello World";
        let options = SearchOptions {
            case_sensitive: true,
            use_regex: false,
        };

        let result = engine.search(content, "World", &options);
        assert_eq!(result.count, 1);
        let m = &result.matches[0];
        assert_eq!(m.start, 6);
        assert_eq!(m.end, 11);
        assert_eq!(m.text, "World");
    }

    #[test]
    fn test_line_column_tracking() {
        let engine = SearchEngine::new();
        let content = "Line 1\nLine 2\nLine 3";
        let options = SearchOptions {
            case_sensitive: true,
            use_regex: false,
        };

        let result = engine.search(content, "Line 2", &options);
        assert_eq!(result.count, 1);
        let m = &result.matches[0];
        assert_eq!(m.line, 1);
        assert_eq!(m.column, 0);
    }

    #[test]
    fn test_find_next() {
        let engine = SearchEngine::new();
        let content = "aaa bbb ccc";
        let options = SearchOptions {
            case_sensitive: true,
            use_regex: false,
        };

        // Find next after first "a" at offset 0 - should find "a" at offset 1
        let next = engine.find_next(content, "a", &options, 0);
        assert!(next.is_some());
        assert_eq!(next.unwrap().start, 1);

        // Find next after "a" at offset 1 - should find "a" at offset 2
        let next = engine.find_next(content, "a", &options, 1);
        assert!(next.is_some());
        assert_eq!(next.unwrap().start, 2);

        // Find next after last "a" at offset 2 - no more matches
        let next = engine.find_next(content, "a", &options, 2);
        assert!(next.is_none());
    }

    #[test]
    fn test_find_previous() {
        let engine = SearchEngine::new();
        let content = "aaa bbb ccc";
        let options = SearchOptions {
            case_sensitive: true,
            use_regex: false,
        };

        // Find previous before "a" at offset 2 - should find "a" at offset 1
        let prev = engine.find_previous(content, "a", &options, 2);
        assert!(prev.is_some());
        assert_eq!(prev.unwrap().start, 1);

        // Find previous before "c" at offset 10 - should find "a" at offset 2
        let prev = engine.find_previous(content, "a", &options, 10);
        assert!(prev.is_some());
        assert_eq!(prev.unwrap().start, 2);
    }

    #[test]
    fn test_replace_match() {
        let content = "Hello World";
        let result = SearchEngine::replace_match(content, 6, 11, "Rust");
        assert_eq!(result, "Hello Rust");
    }

    #[test]
    fn test_replace_all() {
        let content = "Hello World\nHello Rust\nHello";
        let options = SearchOptions {
            case_sensitive: true,
            use_regex: false,
        };

        let result = SearchEngine::replace_all(content, "Hello", &options, "Hi");
        assert!(result.success);
        assert!(result.matches[0].text.contains("Hi World"));
        assert!(result.matches[0].text.contains("Hi Rust"));
        assert!(result.matches[0].text.contains("Hi"));
    }

    #[test]
    fn test_multiline_content() {
        let engine = SearchEngine::new();
        let content = "# Heading\n\nSome paragraph text.\n\n- List item\n\nAnother paragraph.";
        let options = SearchOptions {
            case_sensitive: true,
            use_regex: false,
        };

        let result = engine.search(content, "paragraph", &options);
        assert_eq!(result.count, 2);
    }

    #[test]
    fn test_special_characters_literal() {
        let engine = SearchEngine::new();
        let content = "Hello [World]\nHello (Rust)";
        let options = SearchOptions {
            case_sensitive: true,
            use_regex: false,
        };

        let result = engine.search(content, "[World]", &options);
        assert_eq!(result.count, 1);
    }
}
