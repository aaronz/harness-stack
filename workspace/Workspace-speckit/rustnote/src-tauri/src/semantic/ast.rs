use comrak::{markdown_to_html, Options};

pub struct SemanticDocument {
    source: String,
}

impl SemanticDocument {
    pub fn parse(source: &str) -> Self {
        SemanticDocument {
            source: source.to_string(),
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn serialize_to_commonmark(&self) -> String {
        self.source.clone()
    }

    pub fn offset_to_position(&self, offset: usize) -> Position {
        let offset = offset.min(self.source.len());
        let mut line: u32 = 0;
        let mut column: u32 = 0;
        let mut current_offset: usize = 0;

        for ch in self.source.chars() {
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

        Position::new(offset, line, column)
    }

    pub fn position_to_offset(&self, pos: Position) -> usize {
        let mut current_line: u32 = 0;
        let mut current_col: u32 = 0;
        let mut offset: usize = 0;

        for ch in self.source.chars() {
            if current_line == pos.line && current_col >= pos.column {
                break;
            }
            if ch == '\n' {
                current_line += 1;
                current_col = 0;
            } else {
                current_col += 1;
            }
            offset += ch.len_utf8();
        }

        offset.min(self.source.len())
    }

    pub fn html(&self) -> String {
        markdown_to_html(&self.source, &Options::default())
    }

    pub fn get_headings(&self) -> Vec<HeadingInfo> {
        let mut headings = Vec::new();
        let mut offset = 0;
        for line in self.source.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                let level = trimmed.chars().take_while(|&c| c == '#').count();
                if level <= 6 && trimmed.chars().nth(level) == Some(' ') {
                    let text = trimmed[level + 1..].trim();
                    headings.push(HeadingInfo {
                        level: level as u8,
                        text: text.to_string(),
                        offset,
                    });
                }
            }
            offset += line.len() + 1;
        }
        headings
    }

    pub fn get_list_items(&self) -> Vec<ListItemInfo> {
        let mut items = Vec::new();
        let mut offset = 0;
        for line in self.source.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("- [ ] ")
                || trimmed.starts_with("- [x] ")
                || trimmed.starts_with("- [X] ")
            {
                let checked = trimmed.starts_with("- [x] ") || trimmed.starts_with("- [X] ");
                let text = trimmed[6..].trim();
                items.push(ListItemInfo {
                    checked,
                    text: text.to_string(),
                    offset,
                });
            } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                let text = trimmed[2..].trim();
                items.push(ListItemInfo {
                    checked: false,
                    text: text.to_string(),
                    offset,
                });
            }
            offset += line.len() + 1;
        }
        items
    }

    pub fn get_paragraphs(&self) -> Vec<ParagraphInfo> {
        let mut paragraphs = Vec::new();
        let mut offset = 0;
        let mut index = 0;

        for line in self.source.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty()
                && !trimmed.starts_with('#')
                && !trimmed.starts_with("- ")
                && !trimmed.starts_with("* ")
                && !trimmed.starts_with("> ")
                && !trimmed.starts_with("```")
                && !trimmed.starts_with("| ")
            {
                paragraphs.push(ParagraphInfo {
                    index,
                    offset,
                    length: line.len(),
                });
                index += 1;
            }
            offset += line.len() + 1;
        }
        paragraphs
    }

    pub fn get_paragraph_at(&self, offset: usize) -> usize {
        let paragraphs = self.get_paragraphs();
        if paragraphs.is_empty() {
            return 0;
        }

        let mut result = 0;
        for para in paragraphs {
            if para.offset <= offset {
                result = para.index;
            } else {
                break;
            }
        }
        result
    }

    pub fn get_headings_with_positions(&self) -> Vec<HeadingInfo> {
        self.get_headings()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub offset: usize,
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub fn new(offset: usize, line: u32, column: u32) -> Self {
        Self {
            offset,
            line,
            column,
        }
    }

    pub fn from_offset(offset: usize) -> Self {
        Self {
            offset,
            line: 0,
            column: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRange {
    pub start: Position,
    pub end: Position,
}

impl SourceRange {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end.offset.saturating_sub(self.start.offset)
    }

    pub fn is_empty(&self) -> bool {
        self.start.offset >= self.end.offset
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListType {
    pub ordered: bool,
    pub start: u32,
    pub marker: char,
}

impl ListType {
    pub fn new(ordered: bool, start: u32, marker: char) -> Self {
        Self {
            ordered,
            start,
            marker,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HeadingInfo {
    pub level: u8,
    pub text: String,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct ListItemInfo {
    pub checked: bool,
    pub text: String,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct ParagraphInfo {
    pub index: usize,
    pub offset: usize,
    pub length: usize,
}

impl std::fmt::Debug for SemanticDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SemanticDocument")
            .field("source", &self.source.chars().take(50).collect::<String>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let source = "# Hello World";
        let doc = SemanticDocument::parse(source);
        assert!(doc.html().contains("<h1>"));
        assert_eq!(doc.source(), source);
    }

    #[test]
    fn test_offset_to_position() {
        let source = "Line 1\nLine 2\nLine 3";
        let doc = SemanticDocument::parse(source);
        let pos = doc.offset_to_position(10);
        assert_eq!(pos.line, 1);
    }

    #[test]
    fn test_position_to_offset() {
        let source = "Line 1\nLine 2\nLine 3";
        let doc = SemanticDocument::parse(source);
        let pos = Position::new(12, 1, 5);
        let offset = doc.position_to_offset(pos);
        assert_eq!(offset, 12);
    }

    #[test]
    fn test_serialize_heading() {
        let source = "# Hello";
        let doc = SemanticDocument::parse(source);
        let output = doc.serialize_to_commonmark();
        assert!(output.to_lowercase().contains("# hello"));
    }

    #[test]
    fn test_serialize_bold() {
        let source = "**bold** text";
        let doc = SemanticDocument::parse(source);
        let output = doc.serialize_to_commonmark();
        assert!(output.contains("**bold**"));
    }

    #[test]
    fn test_get_headings() {
        let source = "# Heading 1\n## Heading 2\n### Heading 3";
        let doc = SemanticDocument::parse(source);
        let headings = doc.get_headings();
        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0].level, 1);
        assert_eq!(headings[1].level, 2);
        assert_eq!(headings[2].level, 3);
    }

    #[test]
    fn test_get_list_items() {
        let source = "- [ ] unchecked\n- [x] checked";
        let doc = SemanticDocument::parse(source);
        let items = doc.get_list_items();
        assert_eq!(items.len(), 2);
        assert!(!items[0].checked);
        assert!(items[1].checked);
    }

    #[test]
    fn test_roundtrip_fidelity() {
        let source = "# Hello\n\nThis is **bold** and *italic*.\n\n- Item 1\n- Item 2\n\n```rust\nfn main() {}\n```";
        let doc = SemanticDocument::parse(source);
        let output = doc.serialize_to_commonmark();
        assert_eq!(output, source);
    }

    #[test]
    fn test_roundtrip_task_list() {
        let source = "- [ ] unchecked\n- [x] checked";
        let doc = SemanticDocument::parse(source);
        let output = doc.serialize_to_commonmark();
        assert_eq!(output, source);
    }

    #[test]
    fn test_roundtrip_blockquote() {
        let source = "> This is a quote";
        let doc = SemanticDocument::parse(source);
        let output = doc.serialize_to_commonmark();
        assert_eq!(output, source);
    }

    #[test]
    fn test_roundtrip_table() {
        let source = "| a | b |\n|---|---|\n| 1 | 2 |";
        let doc = SemanticDocument::parse(source);
        let output = doc.serialize_to_commonmark();
        assert_eq!(output, source);
    }
}
