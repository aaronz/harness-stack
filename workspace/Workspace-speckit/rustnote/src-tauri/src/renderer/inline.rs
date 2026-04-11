pub struct InlineRenderer;

impl InlineRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render_emphasis(&self, content: &str, strong: bool) -> String {
        if strong {
            format!("<strong>{}</strong>", content)
        } else {
            format!("<em>{}</em>", content)
        }
    }

    pub fn render_code(&self, content: &str) -> String {
        format!("<code>{}</code>", self.escape_html(content))
    }

    pub fn render_link(&self, content: &str, url: &str) -> String {
        format!("<a href=\"{}\">{}</a>", self.escape_html(url), content)
    }

    pub fn render_image(&self, alt: &str, url: &str) -> String {
        format!(
            "<img src=\"{}\" alt=\"{}\" />",
            self.escape_html(url),
            self.escape_html(alt)
        )
    }

    pub fn render_strikethrough(&self, content: &str) -> String {
        format!("<del>{}</del>", content)
    }

    pub fn render_inline_code(&self, content: &str) -> String {
        format!("<code>{}</code>", self.escape_html(content))
    }

    fn escape_html(&self, s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    pub fn render_text(&self, text: &str) -> String {
        self.escape_html(text)
    }

    pub fn render_soft_break(&self) -> String {
        "\n".to_string()
    }

    pub fn render_hard_break(&self) -> String {
        "<br />\n".to_string()
    }
}

impl Default for InlineRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_emphasis() {
        let renderer = InlineRenderer::new();
        assert_eq!(renderer.render_emphasis("text", false), "<em>text</em>");
        assert_eq!(
            renderer.render_emphasis("text", true),
            "<strong>text</strong>"
        );
    }

    #[test]
    fn test_render_code() {
        let renderer = InlineRenderer::new();
        assert_eq!(renderer.render_code("`code`"), "<code>`code`</code>");
    }

    #[test]
    fn test_render_link() {
        let renderer = InlineRenderer::new();
        assert_eq!(
            renderer.render_link("text", "http://example.com"),
            "<a href=\"http://example.com\">text</a>"
        );
    }

    #[test]
    fn test_escape_html() {
        let renderer = InlineRenderer::new();
        assert_eq!(renderer.escape_html("<div>"), "&lt;div&gt;");
    }
}
