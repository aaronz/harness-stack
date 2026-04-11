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
}
