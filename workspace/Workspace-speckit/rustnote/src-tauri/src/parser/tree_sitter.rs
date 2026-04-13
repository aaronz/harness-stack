use std::time::{Duration, Instant};
use tree_sitter::{Parser, Point, Tree};

fn offset_to_point(source: &str, offset: usize) -> Point {
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut current_offset = 0usize;
    for ch in source.chars() {
        if current_offset >= offset {
            break;
        }
        if ch == '\n' {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
        current_offset += ch.len_utf8();
    }
    Point { row, column: col }
}

pub struct TreeSitterParser {
    parser: Parser,
    last_tree: Option<Tree>,
    last_source: Option<String>,
    last_parse_was_incremental: bool,
}

impl TreeSitterParser {
    pub fn new() -> Result<Self, String> {
        let mut parser = Parser::new();
        let lang = tree_sitter_markdown::language();
        parser
            .set_language(lang)
            .map_err(|e| format!("Failed to set language: {:?}", e))?;
        Ok(Self {
            parser,
            last_tree: None,
            last_source: None,
            last_parse_was_incremental: false,
        })
    }

    pub fn parse(&mut self, source: &str) -> Result<(Tree, Duration), String> {
        let start = Instant::now();
        let (tree, was_incremental) = match self.last_tree.clone() {
            Some(last_tree) => {
                let last_source = self.last_source.clone().unwrap_or_default();
                let (tree, incremental) =
                    self.try_incremental_parse(source, &last_source, &last_tree)?;
                (tree, incremental)
            }
            None => {
                let tree = self.full_parse(source)?;
                (tree, false)
            }
        };
        let duration = start.elapsed();
        self.last_tree = Some(tree.clone());
        self.last_source = Some(source.to_string());
        self.last_parse_was_incremental = was_incremental;
        Ok((tree, duration))
    }

    fn full_parse(&mut self, source: &str) -> Result<Tree, String> {
        self.parser
            .parse(source, None)
            .ok_or_else(|| "Failed to parse source".to_string())
    }

    fn try_incremental_parse(
        &mut self,
        new_source: &str,
        old_source: &str,
        old_tree: &Tree,
    ) -> Result<(Tree, bool), String> {
        let size_diff = (new_source.len() as i64 - old_source.len() as i64).abs();
        if size_diff > 1000 || size_diff > 10 {
            let tree = self.full_parse(new_source)?;
            return Ok((tree, false));
        }
        let Some((start_byte, old_end, new_end)) = Self::find_edit_region(old_source, new_source)
        else {
            let tree = self.full_parse(new_source)?;
            return Ok((tree, true));
        };
        let old_len = old_source.len();
        let start_position = offset_to_point(old_source, start_byte);
        let old_end_position = offset_to_point(old_source, old_end.min(old_len));
        let new_end_position = offset_to_point(new_source, new_end.min(new_source.len()));

        let edit = tree_sitter::InputEdit {
            start_byte,
            old_end_byte: old_end,
            new_end_byte: new_end,
            start_position,
            old_end_position,
            new_end_position,
        };

        let mut edited_tree = old_tree.clone();
        edited_tree.edit(&edit);
        let tree = self
            .parser
            .parse(new_source, Some(&edited_tree))
            .ok_or_else(|| "Incremental parse failed".to_string())?;
        Ok((tree, true))
    }

    fn find_edit_region(old: &str, new: &str) -> Option<(usize, usize, usize)> {
        let old_bytes = old.as_bytes();
        let new_bytes = new.as_bytes();
        let mut start = 0;
        while start < old_bytes.len()
            && start < new_bytes.len()
            && old_bytes[start] == new_bytes[start]
        {
            start += 1;
        }
        let mut old_end = old_bytes.len();
        let mut new_end = new_bytes.len();
        while old_end > start && new_end > start && old_bytes[old_end - 1] == new_bytes[new_end - 1]
        {
            old_end -= 1;
            new_end -= 1;
        }
        if (start == 0 || start >= old_end)
            && old_end == old_bytes.len()
            && new_end == new_bytes.len()
        {
            None
        } else {
            Some((start, old_end, new_end))
        }
    }

    pub fn root_node(&self) -> Option<&Tree> {
        self.last_tree.as_ref()
    }

    pub fn was_incremental(&self) -> bool {
        self.last_parse_was_incremental
    }
}

impl Default for TreeSitterParser {
    fn default() -> Self {
        Self::new().expect("Failed to create TreeSitterParser")
    }
}

pub struct ParseResult {
    pub tree: Tree,
    pub parse_time: Duration,
    pub was_incremental: bool,
    pub source_len: usize,
}

impl TreeSitterParser {
    pub fn parse_with_result(&mut self, source: &str) -> Result<ParseResult, String> {
        let (tree, parse_time) = self.parse(source)?;
        let was_incremental = self.was_incremental();
        let source_len = source.len();
        Ok(ParseResult {
            tree,
            parse_time,
            was_incremental,
            source_len,
        })
    }
}

pub struct IncrementalEdit {
    pub start_byte: usize,
    pub old_end_byte: usize,
    pub new_end_byte: usize,
    pub start_position: Point,
    pub old_end_position: Point,
    pub new_end_position: Point,
}

impl IncrementalEdit {
    pub fn from_sources(old_source: &str, new_source: &str, edit_start: usize) -> Self {
        let old_bytes = old_source.as_bytes();
        let new_bytes = new_source.as_bytes();
        let old_end = edit_start + old_bytes.len().saturating_sub(edit_start);
        let new_end = edit_start + new_bytes.len().saturating_sub(edit_start);
        let start_position = offset_to_point(old_source, edit_start);
        let old_end_position = offset_to_point(old_source, old_end.min(old_bytes.len()));
        let new_end_position = offset_to_point(new_source, new_end.min(new_bytes.len()));
        Self {
            start_byte: edit_start,
            old_end_byte: old_end,
            new_end_byte: new_end,
            start_position,
            old_end_position,
            new_end_position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_parser() {
        let parser = TreeSitterParser::new();
        assert!(parser.is_ok());
    }

    #[test]
    fn test_full_parse() {
        let mut parser = TreeSitterParser::new().unwrap();
        let source = "# Hello World\n\nThis is a paragraph.";
        let result = parser.parse(source);
        assert!(result.is_ok());
        let (tree, duration) = result.unwrap();
        assert!(tree.root_node().byte_range().end > 0);
        assert!(duration.as_millis() >= 0);
    }

    #[test]
    fn test_incremental_parse_small_edit() {
        let mut parser = TreeSitterParser::new().unwrap();
        let source = "# Hello World";
        parser.parse(source).unwrap();
        let new_source = "# Hello World!";
        let start = Instant::now();
        let result = parser.parse(new_source);
        let elapsed = start.elapsed();
        assert!(result.is_ok());
        assert!(elapsed.as_millis() < 100);
    }

    #[test]
    fn test_incremental_edit_detection() {
        let old = "# Hello World\n\nParagraph";
        let new = "# Hello World!\n\nParagraph";
        let edit = IncrementalEdit::from_sources(old, new, 12);
        assert_eq!(edit.start_byte, 12);
    }

    #[test]
    fn test_find_edit_region() {
        let old = "Hello World";
        let new = "Hello Beautiful World";
        let region = TreeSitterParser::find_edit_region(old, new);
        assert!(region.is_some());
        let (start, old_end, new_end) = region.unwrap();
        assert_eq!(start, 6);
        assert_eq!(old_end, 6);
        assert_eq!(new_end, 16);
    }

    #[test]
    fn test_no_change() {
        let old = "Hello World";
        let new = "Hello World";
        let region = TreeSitterParser::find_edit_region(old, new);
        assert!(region.is_none());
    }

    #[test]
    fn test_parse_paragraph() {
        let mut parser = TreeSitterParser::new().unwrap();
        let source = "This is a paragraph.";
        let result = parser.parse(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_heading() {
        let mut parser = TreeSitterParser::new().unwrap();
        let source = "# Heading 1\n## Heading 2\n### Heading 3";
        let result = parser.parse(source);
        assert!(result.is_ok());
        let (tree, _) = result.unwrap();
        assert!(tree.root_node().byte_range().end > 0);
    }

    #[test]
    fn test_parse_list() {
        let mut parser = TreeSitterParser::new().unwrap();
        let source = "- Item 1\n- Item 2\n- Item 3";
        let result = parser.parse(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_blockquote() {
        let mut parser = TreeSitterParser::new().unwrap();
        let source = "> This is a quote";
        let result = parser.parse(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_code_block() {
        let mut parser = TreeSitterParser::new().unwrap();
        let source = "```rust\nfn main() {}\n```";
        let result = parser.parse(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_table() {
        let mut parser = TreeSitterParser::new().unwrap();
        let source = "| Header 1 | Header 2 |\n| -------- | -------- |\n| Cell 1   | Cell 2   |";
        let result = parser.parse(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_was_incremental() {
        let mut parser = TreeSitterParser::new().unwrap();
        assert!(!parser.was_incremental());
        parser.parse("# Hello").unwrap();
        assert!(!parser.was_incremental());
        parser.parse("# Hello World").unwrap();
        assert!(parser.was_incremental());
    }

    #[test]
    fn test_parse_with_result() {
        let mut parser = TreeSitterParser::new().unwrap();
        let source = "# Test";
        let result = parser.parse_with_result(source);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.source_len, 6);
        assert!(!result.was_incremental);
    }

    #[test]
    fn test_large_change_full_parse() {
        let mut parser = TreeSitterParser::new().unwrap();
        let source = "# Short";
        parser.parse(source).unwrap();
        let new_source = "# This is a much longer heading with more content";
        let (tree, _) = parser.parse(new_source).unwrap();
        assert!(tree.root_node().byte_range().end > source.len());
    }
}
