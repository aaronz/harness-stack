pub struct BlockRenderer;

impl BlockRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render_document(&self, markdown: &str) -> String {
        comrak::markdown_to_html(markdown, &comrak::Options::default())
    }
}

impl Default for BlockRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_heading() {
        let renderer = BlockRenderer::new();
        let result = renderer.render_document("# Hello");
        assert!(result.contains("<h1>"));
        assert!(result.contains("Hello"));
    }

    #[test]
    fn test_render_paragraph() {
        let renderer = BlockRenderer::new();
        let result = renderer.render_document("Hello world");
        assert!(result.contains("<p>"));
    }

    #[test]
    fn test_render_blockquote() {
        let renderer = BlockRenderer::new();
        let result = renderer.render_document("> quote");
        assert!(result.contains("<blockquote>"));
    }
}
