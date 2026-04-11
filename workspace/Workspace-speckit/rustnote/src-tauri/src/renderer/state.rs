pub struct RenderState;

impl RenderState {
    pub fn new() -> Self {
        Self
    }

    pub fn render_markdown_to_html(&self, markdown: &str) -> String {
        comrak::markdown_to_html(markdown, &comrak::Options::default())
    }
}

impl Default for RenderState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_markdown_to_html(markdown: &str) -> String {
    comrak::markdown_to_html(markdown, &comrak::Options::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_markdown_to_html() {
        let state = RenderState::new();
        let html = state.render_markdown_to_html("# Hello");
        assert!(html.contains("<h1>"));
    }

    #[test]
    fn test_render_markdown_to_html_fn() {
        let html = render_markdown_to_html("# Hello");
        assert!(html.contains("<h1>"));
    }
}
