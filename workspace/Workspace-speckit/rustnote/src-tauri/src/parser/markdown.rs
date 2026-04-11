use comrak::{markdown_to_html, Options};

pub struct MarkdownParser {
    options: Options<'static>,
}

impl MarkdownParser {
    pub fn new() -> Self {
        let mut options = Options::default();
        options.extension.table = true;
        options.extension.tasklist = true;
        options.extension.strikethrough = true;
        options.extension.autolink = true;
        Self { options }
    }

    pub fn parse_to_html(&self, markdown: &str) -> String {
        markdown_to_html(markdown, &self.options)
    }

    pub fn serialize_to_markdown(&self, markdown: &str) -> String {
        markdown.to_string()
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parser() -> MarkdownParser {
        MarkdownParser::new()
    }

    #[test]
    fn test_parse_heading() {
        let result = parser().parse_to_html("# Hello World");
        assert!(result.contains("<h1>"));
        assert!(result.contains("Hello World"));
    }

    #[test]
    fn test_parse_bold_italic() {
        let result = parser().parse_to_html("**bold** and *italic*");
        assert!(result.contains("<strong>bold</strong>"));
        assert!(result.contains("<em>italic</em>"));
    }

    #[test]
    fn test_parse_list() {
        let result = parser().parse_to_html("- item 1\n- item 2");
        assert!(result.contains("<ul>"));
        assert!(result.contains("<li>"));
    }

    #[test]
    fn test_parse_code_block() {
        let result = parser().parse_to_html("```rust\nfn main() {}\n```");
        assert!(result.contains("<code"));
        assert!(result.contains("rust"));
    }

    #[test]
    fn test_parse_table() {
        let result = parser().parse_to_html("| a | b |\n|---|---|\n| 1 | 2 |");
        eprintln!("Table output: {:?}", result);
        assert!(result.contains("<table>"));
        assert!(result.contains("<td>"));
    }

    #[test]
    fn test_parse_task_list() {
        let result = parser().parse_to_html("- [ ] unchecked\n- [x] checked");
        eprintln!("Task list output: {:?}", result);
        assert!(result.contains("input"));
    }
}
