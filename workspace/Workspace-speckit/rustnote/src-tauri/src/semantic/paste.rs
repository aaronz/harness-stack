pub struct HtmlToMarkdownConverter {
    preserve_whitespace: bool,
    strip_images: bool,
}

impl HtmlToMarkdownConverter {
    pub fn new() -> Self {
        Self {
            preserve_whitespace: false,
            strip_images: false,
        }
    }

    pub fn with_options(preserve_whitespace: bool, strip_images: bool) -> Self {
        Self {
            preserve_whitespace,
            strip_images,
        }
    }

    pub fn convert(&self, html: &str) -> String {
        if html.trim().is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut list_stack: Vec<char> = Vec::new();

        let bytes = html.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            if bytes[i] == b'<' {
                let remaining_len = len - i;

                if remaining_len >= 4 && &bytes[i..i + 4] == b"</b>" {
                    result.push_str("**");
                    i += 4;
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 4] == b"</i>" {
                    result.push('*');
                    i += 4;
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 4] == b"</a>" {
                    result.push_str("](url)");
                    i += 4;
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 4] == b"</li" {
                    i += 4;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 6 && &bytes[i..i + 5] == b"</div" {
                    result.push('\n');
                    i += 5;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 6 && &bytes[i..i + 5] == b"</spa" {
                    i += 5;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 6 && &bytes[i..i + 5] == b"</ul>" {
                    list_stack.pop();
                    result.push('\n');
                    i += 5;
                    continue;
                }
                if remaining_len >= 6 && &bytes[i..i + 5] == b"</ol>" {
                    list_stack.pop();
                    result.push('\n');
                    i += 5;
                    continue;
                }
                if remaining_len >= 5 && &bytes[i..i + 5] == b"</em>" {
                    result.push('*');
                    i += 5;
                    continue;
                }
                if remaining_len >= 6 && &bytes[i..i + 5] == b"</del" {
                    result.push_str("~~");
                    i += 5;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 7 && &bytes[i..i + 6] == b"</code" {
                    if !in_code_block {
                        result.push('`');
                    }
                    i += 6;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 9 && &bytes[i..i + 9] == b"</strong>" {
                    result.push_str("**");
                    i += 9;
                    continue;
                }
                if remaining_len >= 6 && &bytes[i..i + 6] == b"</del>" {
                    result.push_str("~~");
                    i += 6;
                    continue;
                }
                if remaining_len >= 8 && &bytes[i..i + 8] == b"</strike>" {
                    result.push_str("~~");
                    i += 8;
                    continue;
                }
                if remaining_len >= 12 && &bytes[i..i + 12] == b"</blockquote>" {
                    i += 12;
                    continue;
                }
                if remaining_len >= 5 && &bytes[i..i + 5] == b"</h1>" {
                    result.push('\n');
                    i += 5;
                    continue;
                }
                if remaining_len >= 5 && &bytes[i..i + 5] == b"</h2>" {
                    result.push('\n');
                    i += 5;
                    continue;
                }
                if remaining_len >= 5 && &bytes[i..i + 5] == b"</h3>" {
                    result.push('\n');
                    i += 5;
                    continue;
                }
                if remaining_len >= 5 && &bytes[i..i + 5] == b"</h4>" {
                    result.push('\n');
                    i += 5;
                    continue;
                }
                if remaining_len >= 5 && &bytes[i..i + 5] == b"</h5>" {
                    result.push('\n');
                    i += 5;
                    continue;
                }
                if remaining_len >= 5 && &bytes[i..i + 5] == b"</h6>" {
                    result.push('\n');
                    i += 5;
                    continue;
                }
                if remaining_len >= 5 && &bytes[i..i + 4] == b"<pre" {
                    in_code_block = true;
                    code_block_lang.clear();
                    i += 4;
                    // Extract language from class="language-rust" or class="rust"
                    let class_start = i;
                    while i < len && bytes[i] != b'>' && bytes[i] != b'<' {
                        i += 1;
                    }
                    if i < len && bytes[i] != b'<' {
                        let tag_slice = std::str::from_utf8(&bytes[class_start..i]).unwrap_or("");
                        if let Some(lang_pos) = tag_slice.find("language-") {
                            let after_lang = &tag_slice[lang_pos + 9..];
                            if let Some(end_quote) = after_lang.find('"') {
                                code_block_lang.push_str(&after_lang[..end_quote]);
                            } else if let Some(end_bracket) = after_lang.find('>') {
                                code_block_lang.push_str(&after_lang[..end_bracket]);
                            }
                        } else if let Some(class_pos) = tag_slice.find("class=\"") {
                            let after_class = &tag_slice[class_pos + 6..];
                            let end_pos = after_class.find('"').unwrap_or(0);
                            if end_pos > 0 && end_pos <= 10 {
                                let potential_lang = &after_class[..end_pos];
                                if !potential_lang.is_empty()
                                    && potential_lang.chars().all(|c| c.is_alphanumeric())
                                {
                                    code_block_lang.push_str(potential_lang);
                                }
                            }
                        }
                    }
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    // Output code fence with language
                    result.push_str("```");
                    if !code_block_lang.is_empty() {
                        result.push_str(&code_block_lang);
                    }
                    result.push('\n');
                    continue;
                }
                if remaining_len >= 6 && &bytes[i..i + 6] == b"</pre>" {
                    in_code_block = false;
                    result.push_str("```\n");
                    i += 6;
                    continue;
                }
                if remaining_len >= 7 && &bytes[i..i + 6] == b"<code>" {
                    if !in_code_block {
                        result.push('`');
                    }
                    i += 6;
                    continue;
                }
                if remaining_len >= 6 && &bytes[i..i + 5] == b"<code" {
                    if !in_code_block {
                        result.push('`');
                    }
                    i += 5;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<b>" {
                    result.push_str("**");
                    i += 3;
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<i>" {
                    result.push('*');
                    i += 3;
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 4] == b"<em>" {
                    result.push('*');
                    i += 4;
                    continue;
                }
                if remaining_len >= 8 && &bytes[i..i + 8] == b"<strong>" {
                    result.push_str("**");
                    i += 8;
                    continue;
                }
                if remaining_len >= 8 && &bytes[i..i + 7] == b"<b style" {
                    result.push_str("**");
                    i += 7;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 8 && &bytes[i..i + 7] == b"<i style" {
                    result.push('*');
                    i += 7;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<a " {
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    result.push('[');
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<p>" {
                    i += 3;
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<p " {
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<br" {
                    result.push_str("  \n");
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<ul" {
                    list_stack.push('-');
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    result.push('\n');
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<ol" {
                    list_stack.push('#');
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    result.push('\n');
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<li" {
                    let marker = list_stack.last().copied().unwrap_or('-');
                    result.push('\n');
                    result.push_str(&"  ".repeat(list_stack.len().saturating_sub(1)));
                    result.push(marker);
                    result.push(' ');
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 11 && &bytes[i..i + 11] == b"<blockquote" {
                    result.push_str("> ");
                    i += 11;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<di" {
                    result.push('\n');
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<sp" {
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<hr" {
                    result.push_str("\n---\n");
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 3 && &bytes[i..i + 3] == b"<h1" {
                    result.push_str("# ");
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 3 && &bytes[i..i + 3] == b"<h2" {
                    result.push_str("## ");
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 3 && &bytes[i..i + 3] == b"<h3" {
                    result.push_str("### ");
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 3 && &bytes[i..i + 3] == b"<h4" {
                    result.push_str("#### ");
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 3 && &bytes[i..i + 3] == b"<h5" {
                    result.push_str("##### ");
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 3 && &bytes[i..i + 3] == b"<h6" {
                    result.push_str("###### ");
                    i += 3;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 3 && &bytes[i..i + 3] == b"<s>" {
                    result.push_str("~~");
                    i += 3;
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 4] == b"</s>" {
                    result.push_str("~~");
                    i += 4;
                    continue;
                }
                if remaining_len >= 5 && &bytes[i..i + 5] == b"<del>" {
                    result.push_str("~~");
                    i += 5;
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 4] == b"<!--" {
                    i += 4;
                    while i + 2 < len {
                        if &bytes[i..i + 3] == b"-->" {
                            i += 3;
                            break;
                        }
                        i += 1;
                    }
                    continue;
                }

                i += 1;
                continue;
            }

            if bytes[i] == b'&' {
                let mut j = i;
                while j < len && j < i + 10 {
                    if bytes[j] == b';' {
                        let entity = std::str::from_utf8(&bytes[i..=j]).unwrap_or("");
                        let decoded = match entity {
                            "&amp;" => "&",
                            "&lt;" => "<",
                            "&gt;" => ">",
                            "&quot;" => "\"",
                            "&apos;" => "'",
                            "&nbsp;" => " ",
                            "&#39;" => "'",
                            "&#x27;" => "'",
                            _ => entity,
                        };
                        result.push_str(decoded);
                        i = j + 1;
                        break;
                    }
                    j += 1;
                }
                if j >= i + 10 || j >= len {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                continue;
            }

            result.push(bytes[i] as char);
            i += 1;
        }

        let mut cleaned = String::new();
        let mut last_was_newline = false;
        for ch in result.chars() {
            if ch == '\n' {
                if !last_was_newline {
                    cleaned.push(ch);
                    last_was_newline = true;
                }
            } else {
                cleaned.push(ch);
                last_was_newline = false;
            }
        }

        cleaned.trim().to_string()
    }

    pub fn convert_word_html(&self, html: &str) -> String {
        let normalized = html
            .replace("<o:p>", "")
            .replace("</o:p>", "")
            .replace("<![if !supportLists]>", "")
            .replace("<![endif]>", "")
            .replace("msonormal", "")
            .replace("xml:lang=", "lang=");

        self.convert(&normalized)
    }

    pub fn convert_web_html(&self, html: &str) -> String {
        let cleaned = html
            .strip_prefix("<html")
            .map(|s| {
                if let Some(end) = s.find("</html>") {
                    &s[..end]
                } else {
                    s
                }
            })
            .unwrap_or(html);

        self.convert(cleaned)
    }
}

impl Default for HtmlToMarkdownConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_empty() {
        let converter = HtmlToMarkdownConverter::new();
        assert_eq!(converter.convert(""), "");
        assert_eq!(converter.convert("   "), "");
    }

    #[test]
    fn test_convert_bold() {
        let converter = HtmlToMarkdownConverter::new();
        assert_eq!(converter.convert("<b>bold</b>"), "**bold**");
        assert_eq!(converter.convert("<strong>bold</strong>"), "**bold**");
    }

    #[test]
    fn test_convert_italic() {
        let converter = HtmlToMarkdownConverter::new();
        assert_eq!(converter.convert("<i>italic</i>"), "*italic*");
        assert_eq!(converter.convert("<em>italic</em>"), "*italic*");
    }

    #[test]
    fn test_convert_headings() {
        let converter = HtmlToMarkdownConverter::new();
        assert_eq!(converter.convert("<h1>Heading 1</h1>"), "# Heading 1");
        assert_eq!(converter.convert("<h2>Heading 2</h2>"), "## Heading 2");
        assert_eq!(converter.convert("<h3>Heading 3</h3>"), "### Heading 3");
    }

    #[test]
    fn test_convert_inline_code() {
        let converter = HtmlToMarkdownConverter::new();
        assert_eq!(
            converter.convert("<code>inline code</code>"),
            "`inline code`"
        );
    }

    #[test]
    fn test_convert_paragraphs() {
        let converter = HtmlToMarkdownConverter::new();
        let result = converter.convert("<p>First paragraph</p><p>Second paragraph</p>");
        assert!(result.contains("First paragraph"));
        assert!(result.contains("Second paragraph"));
    }

    #[test]
    fn test_convert_strikethrough() {
        let converter = HtmlToMarkdownConverter::new();
        assert_eq!(converter.convert("<del>deleted</del>"), "~~deleted~~");
        assert_eq!(
            converter.convert("<s>strikethrough</s>"),
            "~~strikethrough~~"
        );
    }

    #[test]
    fn test_convert_break() {
        let converter = HtmlToMarkdownConverter::new();
        let result = converter.convert("Line 1<br>Line 2");
        assert!(result.contains("Line 1"));
        assert!(result.contains("Line 2"));
    }

    #[test]
    fn test_convert_html_entities() {
        let converter = HtmlToMarkdownConverter::new();
        assert_eq!(converter.convert("&amp;"), "&");
        assert_eq!(converter.convert("&lt;script&gt;"), "<script>");
        assert_eq!(converter.convert("&quot;quoted&quot;"), "\"quoted\"");
    }

    #[test]
    fn test_word_html_conversion() {
        let converter = HtmlToMarkdownConverter::new();
        let word_html = "<o:p>content</o:p>";
        let result = converter.convert_word_html(word_html);
        assert_eq!(result, "content");
    }

    #[test]
    fn test_web_html_conversion() {
        let converter = HtmlToMarkdownConverter::new();
        let web_html = "<html><body><p>Web content</p></body></html>";
        let result = converter.convert_web_html(web_html);
        assert!(result.contains("Web content"));
    }

    #[test]
    fn test_mixed_formatting() {
        let converter = HtmlToMarkdownConverter::new();
        let html = "<p>This is <b>bold</b> and <i>italic</i> text.</p>";
        let result = converter.convert(html);
        assert!(result.contains("**bold**"));
        assert!(result.contains("*italic*"));
    }
}
