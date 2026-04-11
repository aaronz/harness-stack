use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn highlight(&self, code: &str, language: &str) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or(self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];

        highlighted_html_for_string(code, &self.syntax_set, syntax, theme)
            .unwrap_or_else(|_| code.to_string())
    }

    pub fn highlight_html(&self, code: &str, language: &str) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];

        let html = highlighted_html_for_string(code, &self.syntax_set, syntax, theme);
        match html {
            Ok(result) => result,
            Err(_) => {
                let escaped = code
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;");
                format!("<pre class=\"highlight\"><code>{}</code></pre>", escaped)
            }
        }
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_rust() {
        let hl = SyntaxHighlighter::new();
        let result = hl.highlight("fn main() {}", "rust");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_highlight_unknown_language() {
        let hl = SyntaxHighlighter::new();
        let result = hl.highlight("some code", "unknown_lang");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_highlight_html_rust() {
        let hl = SyntaxHighlighter::new();
        let result = hl.highlight_html("fn main() {}", "rust");
        assert!(result.contains("<pre"));
        assert!(result.contains("</pre>"));
    }

    #[test]
    fn test_highlight_html_unknown_language() {
        let hl = SyntaxHighlighter::new();
        let result = hl.highlight_html("some code", "unknown_lang");
        assert!(result.contains("<pre"));
        assert!(result.contains("</pre>"));
        assert!(result.contains("some code"));
    }
}
