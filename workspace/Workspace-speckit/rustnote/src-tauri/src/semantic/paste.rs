/// Configuration options for HTML to Markdown conversion
#[derive(Debug, Clone)]
pub struct PasteOptions {
    /// Preserve whitespace in output
    pub preserve_whitespace: bool,
    /// Strip image tags from output
    pub strip_images: bool,
    /// Convert tables from Excel/spreadsheet data
    pub convert_tables: bool,
    /// Convert HTML tables to Markdown tables
    pub convert_html_tables: bool,
    /// Image save directory for pasted images
    pub image_save_dir: Option<String>,
}

impl Default for PasteOptions {
    fn default() -> Self {
        Self {
            preserve_whitespace: false,
            strip_images: false,
            convert_tables: true,
            convert_html_tables: true,
            image_save_dir: None,
        }
    }
}

/// Clipboard content type for format detection
#[derive(Debug, Clone, PartialEq)]
pub enum ClipboardFormat {
    /// Plain text content
    PlainText,
    /// HTML formatted content
    Html,
    /// Rich Text Format
    Rtf,
    /// Tab-separated values (Excel-like)
    TabSeparatedValues,
    /// Markdown text
    Markdown,
    /// Unknown or unsupported format
    Unknown,
}

/// Detect the clipboard content format
pub fn detect_clipboard_format(content: &str) -> ClipboardFormat {
    let trimmed = content.trim();

    // Check for empty content
    if trimmed.is_empty() {
        return ClipboardFormat::Unknown;
    }

    // Check for tab-separated values (Excel-style)
    if looks_like_tsv(trimmed) {
        return ClipboardFormat::TabSeparatedValues;
    }

    // Check for HTML content
    if is_html_content(trimmed) {
        // Check for Word-specific HTML
        if is_word_html(trimmed) {
            return ClipboardFormat::Html; // Word HTML is still HTML
        }
        return ClipboardFormat::Html;
    }

    // Check for RTF content
    if trimmed.starts_with("{\\rtf") || trimmed.starts_with("{\\rtf1") {
        return ClipboardFormat::Rtf;
    }

    // Check for Markdown content
    if looks_like_markdown(trimmed) {
        return ClipboardFormat::Markdown;
    }

    // Default to plain text
    ClipboardFormat::PlainText
}

/// Check if content looks like TSV (tab-separated values)
fn looks_like_tsv(content: &str) -> bool {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 2 {
        return false;
    }

    // Check if first line has tabs
    let first_line_tabs = lines[0].matches('\t').count();
    if first_line_tabs == 0 {
        return false;
    }

    // Check if most lines have similar tab counts
    let valid_lines = lines
        .iter()
        .filter(|line| {
            let tab_count = line.matches('\t').count();
            tab_count >= first_line_tabs.saturating_sub(1) && tab_count <= first_line_tabs + 1
        })
        .count();

    valid_lines as f64 / lines.len() as f64 > 0.7
}

/// Check if content is HTML
fn is_html_content(content: &str) -> bool {
    content.starts_with('<')
        && (content.contains("</")
            || content.contains("/>")
            || content.starts_with("<html")
            || content.starts_with("<body")
            || content.starts_with("<div")
            || content.starts_with("<p")
            || content.starts_with("<table")
            || content.starts_with("<ul")
            || content.starts_with("<ol")
            || content.starts_with("<h1")
            || content.starts_with("<h2")
            || content.starts_with("<h3"))
}

/// Check if content is from Microsoft Word
fn is_word_html(content: &str) -> bool {
    content.contains("msonormal")
        || content.contains("<o:p>")
        || content.contains("</o:p>")
        || content.contains("xml:lang")
        || content.contains("MsoNormal")
        || content.contains("<!--[if")
}

/// Check if content looks like Markdown
fn looks_like_markdown(content: &str) -> bool {
    let markdown_indicators = [
        "**",    // bold
        "__",    // bold alt
        "* ",    // italic or list
        "_",     // italic
        "`",     // code
        "```",   // code block
        "# ",    // heading
        "## ",   // heading
        "### ",  // heading
        "- [ ]", // task list unchecked
        "- [x]", // task list checked
        "> ",    // blockquote
        "| ",    // table
        "---",   // horizontal rule
    ];

    let match_count = markdown_indicators
        .iter()
        .filter(|ind| content.contains(*ind))
        .count();

    match_count >= 2
}

pub struct HtmlToMarkdownConverter {
    options: PasteOptions,
}

impl HtmlToMarkdownConverter {
    pub fn new() -> Self {
        Self {
            options: PasteOptions::default(),
        }
    }

    pub fn with_options(options: PasteOptions) -> Self {
        Self { options }
    }

    /// Check if text appears to be tab-separated table data (Excel-style)
    fn looks_like_excel_table(text: &str) -> bool {
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() < 2 {
            return false;
        }

        // Check if multiple lines contain tabs and have consistent column counts
        let first_line_tabs = lines[0].matches('\t').count();
        if first_line_tabs == 0 {
            return false;
        }

        lines.iter().all(|line| {
            let tab_count = line.matches('\t').count();
            // Allow 1-2 extra columns for header rows
            tab_count >= first_line_tabs.saturating_sub(1) && tab_count <= first_line_tabs + 1
        })
    }

    /// Convert tab-separated table data to Markdown table format
    fn convert_excel_table(&self, text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        if lines.is_empty() {
            return String::new();
        }

        let mut result = String::new();

        for (idx, line) in lines.iter().enumerate() {
            let cells: Vec<&str> = line.split('\t').collect();

            if cells.is_empty() {
                continue;
            }

            // Format as Markdown table row
            result.push('|');
            for cell in &cells {
                result.push(' ');
                result.push_str(cell.trim());
                result.push_str(" |");
            }
            result.push('\n');

            // Add separator row after header (first line)
            if idx == 0 {
                result.push('|');
                for _ in 0..cells.len() {
                    result.push_str("---|");
                }
                result.push('\n');
            }
        }

        result.trim().to_string()
    }

    /// Convert HTML table to Markdown table
    pub fn convert_html_table(&self, html: &str) -> Option<String> {
        let bytes = html.as_bytes();
        let len = bytes.len();

        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut current_row: Vec<String> = Vec::new();
        let mut current_cell = String::new();

        let mut i = 0;
        while i < len {
            // Check for closing table tag
            if i + 8 <= len {
                let slice = std::str::from_utf8(&bytes[i..]).ok()?;
                if slice.starts_with("</table>") {
                    // Push final cell
                    let cell_content = current_cell.trim();
                    if !cell_content.is_empty() {
                        current_row.push(cell_content.to_string());
                    }
                    current_cell.clear();
                    // Push final row
                    if !current_row.is_empty() {
                        rows.push(current_row.clone());
                    }
                    break;
                }
            }

            // Check for row end
            if i + 5 <= len {
                let slice = std::str::from_utf8(&bytes[i..]).ok()?;
                if slice.starts_with("</tr>") {
                    // Push current cell
                    let cell_content = current_cell.trim();
                    if !cell_content.is_empty() {
                        current_row.push(cell_content.to_string());
                    }
                    current_cell.clear();
                    // Push current row
                    if !current_row.is_empty() {
                        rows.push(current_row.clone());
                        current_row.clear();
                    }
                    i += 5;
                    continue;
                }
            }

            // Check for cell end tags
            if i + 5 <= len {
                let slice = std::str::from_utf8(&bytes[i..]).ok()?;
                if slice.starts_with("</td>") || slice.starts_with("</th>") {
                    // Push current cell
                    let cell_content = current_cell.trim();
                    if !cell_content.is_empty() {
                        current_row.push(cell_content.to_string());
                    }
                    current_cell.clear();
                    i += 5;
                    continue;
                }
            }

            // Skip all tags - find the end of any tag starting with <
            if bytes[i] == b'<' {
                // Skip to end of tag
                let mut j = i;
                while j < len && bytes[j] != b'>' {
                    j += 1;
                }
                if j < len {
                    i = j + 1;
                    continue;
                }
            }

            // Collect text content
            current_cell.push(bytes[i] as char);
            i += 1;
        }

        // Need at least 2 rows
        if rows.len() < 2 {
            return None;
        }

        let mut result = String::new();
        let num_cols = rows[0].len();

        for (idx, row) in rows.iter().enumerate() {
            result.push('|');
            for cell in row {
                result.push(' ');
                result.push_str(cell.trim());
                result.push_str(" |");
            }
            result.push('\n');

            if idx == 0 {
                result.push('|');
                for _ in 0..num_cols {
                    result.push_str("---|");
                }
                result.push('\n');
            }
        }

        Some(result.trim().to_string())
    }

    /// Extract alt text from img tag
    fn extract_alt_text(&self, bytes: &[u8], start: usize) -> Option<String> {
        let mut i = start;
        let len = bytes.len();

        // Look for alt="
        while i < len - 5 {
            if &bytes[i..i + 4] == b"alt=" {
                i += 4;
                let quote = bytes[i];
                if quote == b'"' || quote == b'\'' {
                    i += 1;
                    let alt_start = i;
                    while i < len && bytes[i] != quote {
                        i += 1;
                    }
                    return std::str::from_utf8(&bytes[alt_start..i])
                        .map(|s| s.to_string())
                        .ok();
                }
            }
            i += 1;
        }
        None
    }

    /// Generate Markdown image syntax for pasted image
    fn format_image_markdown(&self, alt: &str, src: Option<&str>) -> String {
        if let Some(path) = src {
            format!("![{}]({})", alt, path)
        } else if !alt.is_empty() {
            format!("![{}]()", alt)
        } else {
            String::new()
        }
    }

    /// Extract href attribute from an anchor tag
    fn extract_href(&self, bytes: &[u8], start: usize) -> Option<String> {
        let mut i = start;
        let len = bytes.len();

        // Look for href="
        while i < len - 6 {
            if &bytes[i..i + 5] == b"href=" {
                i += 5;
                let quote = bytes[i];
                if quote == b'"' || quote == b'\'' {
                    i += 1;
                    let href_start = i;
                    while i < len && bytes[i] != quote {
                        i += 1;
                    }
                    return std::str::from_utf8(&bytes[href_start..i])
                        .map(|s| s.to_string())
                        .ok();
                }
            }
            i += 1;
        }
        None
    }

    /// Extract src attribute from a tag
    fn extract_src(&self, bytes: &[u8], start: usize) -> Option<String> {
        let mut i = start;
        let len = bytes.len();

        // Look for src="
        while i < len - 5 {
            if &bytes[i..i + 4] == b"src=" {
                i += 4;
                let quote = bytes[i];
                if quote == b'"' || quote == b'\'' {
                    i += 1;
                    let src_start = i;
                    while i < len && bytes[i] != quote {
                        i += 1;
                    }
                    return std::str::from_utf8(&bytes[src_start..i])
                        .map(|s| s.to_string())
                        .ok();
                }
            }
            i += 1;
        }
        None
    }

    pub fn convert(&self, html: &str) -> String {
        if html.trim().is_empty() {
            return String::new();
        }

        // Check for Excel/tab-separated table data first
        if self.options.convert_tables && Self::looks_like_excel_table(html) {
            return self.convert_excel_table(html);
        }

        // Check for HTML table
        if self.options.convert_html_tables && html.contains("<table") {
            if let Some(table_md) = self.convert_html_table(html) {
                return table_md;
            }
        }

        let mut result = String::new();
        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut list_stack: Vec<char> = Vec::new();
        let mut current_href: Option<String> = None;

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
                    if let Some(href) = current_href.take() {
                        result.push_str(&format!("]({})", href));
                    } else {
                        result.push_str("]()");
                    }
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
                if remaining_len >= 5 && &bytes[i..i + 4] == b"<em>" {
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
                if remaining_len >= 9 && &bytes[i..i + 8] == b"<b class" {
                    result.push_str("**");
                    i += 8;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 7 && &bytes[i..i + 6] == b"<b " {
                    result.push_str("**");
                    i += 6;
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
                if remaining_len >= 9 && &bytes[i..i + 8] == b"<i class" {
                    result.push('*');
                    i += 8;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 7 && &bytes[i..i + 6] == b"<i " {
                    result.push('*');
                    i += 6;
                    while i < len && bytes[i] != b'>' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<a " {
                    // Extract href before consuming the opening tag
                    current_href = self.extract_href(bytes, i + 3);
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
                // Handle img tags - extract alt and src for markdown image
                if remaining_len >= 4 && &bytes[i..i + 3] == b"<im" {
                    if remaining_len >= 5 && (bytes[i + 3] == b'g' || bytes[i + 3] == b' ') {
                        if !self.options.strip_images {
                            // Try to extract alt text
                            let alt = self.extract_alt_text(bytes, i).unwrap_or_default();
                            // Try to extract src
                            let src = self.extract_src(bytes, i);
                            result.push_str(&self.format_image_markdown(&alt, src.as_deref()));
                        }
                        i += 3;
                        while i < len && bytes[i] != b'>' {
                            i += 1;
                        }
                        if i < len {
                            i += 1;
                        }
                        continue;
                    }
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

    /// Handle unknown format by falling back to plain text
    /// This ensures no crash on unrecognized clipboard formats
    pub fn convert_unknown(&self, content: &str) -> String {
        // If content appears to be plain text (no HTML-like tags), return as-is
        let is_likely_html = content.starts_with('<')
            || content.contains("</")
            || content.contains("/>")
            || content.contains("< ");

        if is_likely_html {
            // Try to convert as HTML
            self.convert(content)
        } else {
            // Return as plain text, stripping any accidental HTML artifacts
            let mut result = String::new();
            let mut in_tag = false;
            for ch in content.chars() {
                if ch == '<' {
                    in_tag = true;
                } else if ch == '>' {
                    in_tag = false;
                } else if !in_tag {
                    result.push(ch);
                }
            }
            result.trim().to_string()
        }
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
