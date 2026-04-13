use crate::parser::SyntaxHighlighter;
use crate::semantic::ast::SemanticDocument;
use crate::semantic::position::CursorMapping;

static SYNTAX_HIGHLIGHTER: once_cell::sync::Lazy<SyntaxHighlighter> =
    once_cell::sync::Lazy::new(SyntaxHighlighter::new);

#[tauri::command]
pub fn render_markdown(markdown: String) -> String {
    comrak::markdown_to_html(&markdown, &comrak::Options::default())
}

#[tauri::command]
pub fn highlight_code_block(code: String, language: String) -> String {
    SYNTAX_HIGHLIGHTER.highlight(&code, &language)
}

#[tauri::command]
pub fn get_highlighted_code_html(code: String, language: String) -> String {
    SYNTAX_HIGHLIGHTER.highlight_html(&code, &language)
}

#[tauri::command]
pub fn prehighlight_markdown(markdown: String) -> String {
    let mut result = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();

    for line in markdown.lines() {
        if line.starts_with("```") {
            if !in_code_block {
                in_code_block = true;
                code_lang = line.trim_start_matches("```").trim().to_string();
                code_content.clear();
            } else {
                in_code_block = false;
                let highlighted = SYNTAX_HIGHLIGHTER.highlight_html(&code_content, &code_lang);
                result.push_str(&highlighted);
                result.push('\n');
            }
        } else if in_code_block {
            if !code_content.is_empty() {
                code_content.push('\n');
            }
            code_content.push_str(line);
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

#[tauri::command]
pub fn parse_markdown_ast(markdown: String) -> String {
    let doc = SemanticDocument::parse(&markdown);
    doc.html().to_string()
}

#[tauri::command]
pub fn serialize_markdown(markdown: String) -> String {
    let doc = SemanticDocument::parse(&markdown);
    doc.serialize_to_commonmark()
}

#[tauri::command]
pub fn update_source(source: String) -> SourceUpdateResult {
    let doc = SemanticDocument::parse(&source);
    SourceUpdateResult {
        html: doc.html(),
        source: doc.source().to_string(),
        length: source.len(),
    }
}

#[tauri::command]
pub fn get_markdown_info(markdown: String) -> MarkdownInfo {
    let doc = SemanticDocument::parse(&markdown);
    MarkdownInfo {
        html: doc.html().to_string(),
        source: doc.source().to_string(),
        length: doc.source().len(),
    }
}

#[tauri::command]
pub fn render_for_editor(markdown: String, cursor_offset: usize) -> EditorRenderResult {
    let doc = SemanticDocument::parse(&markdown);
    let html = doc.html();
    let cursor_mapping = build_cursor_mapping(&markdown);
    let active_paragraph = doc.get_paragraph_at(cursor_offset);
    let headings = doc.get_headings_with_positions();

    EditorRenderResult {
        html,
        cursor_mapping,
        active_paragraph,
        headings,
    }
}

#[tauri::command]
pub fn render_for_editor_with_highlighting(
    markdown: String,
    cursor_offset: usize,
    include_highlighting: bool,
) -> EditorRenderResult {
    let doc = SemanticDocument::parse(&markdown);
    let html = if include_highlighting {
        highlight_code_fences(&markdown)
    } else {
        doc.html()
    };
    let cursor_mapping = build_cursor_mapping(&markdown);
    let active_paragraph = doc.get_paragraph_at(cursor_offset);
    let headings = doc.get_headings_with_positions();

    EditorRenderResult {
        html,
        cursor_mapping,
        active_paragraph,
        headings,
    }
}

fn highlight_code_fences(markdown: &str) -> String {
    let mut result = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();

    for line in markdown.lines() {
        if line.starts_with("```") {
            if !in_code_block {
                in_code_block = true;
                code_lang = line.trim_start_matches("```").trim().to_string();
                code_content.clear();
            } else {
                in_code_block = false;
                let highlighted = SYNTAX_HIGHLIGHTER.highlight_html(&code_content, &code_lang);
                result.push_str(&highlighted);
                result.push('\n');
            }
        } else if in_code_block {
            if !code_content.is_empty() {
                code_content.push('\n');
            }
            code_content.push_str(line);
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    comrak::markdown_to_html(&result, &comrak::Options::default())
}

/// AST-aware cursor mapping that accounts for HTML tag insertions during Markdown rendering.
///
/// This function builds a bidirectional mapping between source Markdown offsets and
/// DOM offsets in the rendered HTML. It handles:
/// - Bold formatting (`**text**` → `<strong>text</strong>`)
/// - Italic formatting (`*text*` → `<em>text</em>`)
/// - Inline code (`` `code` `` → `<code>code</code>`)
/// - Links (`[text](url)` → `<a href="url">text</a>`)
///
/// The mapping is built by walking through the source and tracking HTML insertions
/// based on Markdown formatting markers.
pub fn build_cursor_mapping(source: &str) -> Vec<CursorMapping> {
    let mut mappings = Vec::new();
    let bytes = source.as_bytes();
    let len = bytes.len();
    let mut pos: usize = 0;
    let mut line: u32 = 0;
    let mut column: u32 = 0;

    // Track accumulated HTML length
    let mut accumulated_html: usize = 0;

    while pos <= len {
        // Record mapping at current position
        mappings.push(CursorMapping::new(pos, accumulated_html, line, column));

        if pos >= len {
            break;
        }

        let ch = bytes[pos];

        if ch == b'\n' {
            line += 1;
            column = 0;
            pos += 1;
            accumulated_html += 1;
            continue;
        }

        // Check for formatting markers: **, *, `, [, (
        if ch == b'*' || ch == b'`' || ch == b'[' || ch == b'!' {
            // Look ahead to determine marker type and length
            let (marker_len, html_insertion, is_opening) = detect_html_insertion(bytes, pos);

            if marker_len > 0 {
                // Skip the marker characters in source
                pos += marker_len;

                // Account for HTML insertion
                if is_opening {
                    // Opening tag: add to accumulated immediately
                    accumulated_html += html_insertion;
                } else {
                    // Closing tag: add to accumulated after content (pending)
                    accumulated_html += html_insertion;
                }
                continue;
            }
        }

        // Regular character - advance
        column += 1;
        pos += 1;
        accumulated_html += 1;
    }

    mappings
}

/// Detects HTML insertion for Markdown formatting markers.
///
/// Returns (marker_length, html_insertion_length, is_opening) tuple.
/// marker_length is the number of source characters consumed.
/// html_insertion_length is the HTML characters added.
/// is_opening is true for opening tags, false for closing tags.
fn detect_html_insertion(bytes: &[u8], pos: usize) -> (usize, usize, bool) {
    let len = bytes.len();

    // Bold **...**
    if pos + 1 < len && bytes[pos] == b'*' && bytes[pos + 1] == b'*' {
        // Look for closing **
        let rest = &bytes[pos + 2..];
        if let Some(close_pos) = find_closing_marker(rest, b'*', b'*') {
            // Check if there's content between opening and closing
            if close_pos > 0 {
                // Valid bold: **content**
                return (2 + close_pos + 2, 17, false); // </strong> = 9 chars, but we add 8 for opening
            }
        }
        // Check if this is an opening **
        if pos + 2 < len {
            let next_ch = bytes[pos + 2];
            if !is_marker_char(next_ch) {
                return (2, 8, true); // <strong> = 8 chars
            }
        }
    }

    // Italic *...*
    if bytes[pos] == b'*' {
        // Look for closing *
        let rest = &bytes[pos + 1..];
        if let Some(close_pos) = find_closing_marker(rest, b'*', 0) {
            if close_pos > 0 {
                // Valid italic: *content*
                return (1 + close_pos + 1, 9, false); // </em> = 5 chars, but we add 4 for opening
            }
        }
        // Check if this is an opening *
        if pos + 1 < len {
            let next_ch = bytes[pos + 1];
            if !is_marker_char(next_ch) {
                return (1, 4, true); // <em> = 4 chars
            }
        }
    }

    // Inline code `...`
    if bytes[pos] == b'`' {
        // Look for closing `
        let rest = &bytes[pos + 1..];
        if let Some(close_pos) = find_closing_marker(rest, b'`', 0) {
            if close_pos > 0 {
                // Valid code: `content`
                return (1 + close_pos + 1, 13, false); // </code> = 7 chars, but we add 6 for opening
            }
        }
    }

    // Link [text](url) or image ![alt](url)
    if bytes[pos] == b'[' || (pos > 0 && bytes[pos] == b'!' && bytes[pos - 1] == b'[') {
        // Simple link detection - look for ](
        let rest = &bytes[pos..];
        if let Some(close_bracket) = find_marker(rest, b']') {
            if close_bracket + 1 < rest.len() && rest[close_bracket + 1] == b'(' {
                // Found link
                if let Some(close_paren) = find_marker(&rest[close_bracket + 2..], b')') {
                    let total_len = close_bracket + 2 + close_paren + 1;
                    // <a href="url">text</a> approximation
                    let text_len = if bytes[pos] == b'!' {
                        close_bracket - 1
                    } else {
                        close_bracket
                    };
                    let url_len = close_paren;
                    let html_len = 3 + 4 + url_len + 3 + text_len + 4; // <a href=""> + </a>
                    return (total_len, html_len, false);
                }
            }
        }
    }

    (0, 0, false) // No marker detected
}

/// Finds a closing marker, returning the offset to content start
fn find_closing_marker(bytes: &[u8], first: u8, second: u8) -> Option<usize> {
    let mut i = 0;
    while i < bytes.len() {
        if second != 0 && i + 1 < bytes.len() && bytes[i] == first && bytes[i + 1] == second {
            return Some(i);
        }
        if second == 0 && bytes[i] == first && (i == 0 || !is_marker_char(bytes[i - 1])) {
            // Single char marker not preceded by same char
            if i + 1 >= bytes.len() || bytes[i + 1] != first {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

/// Finds a marker character
fn find_marker(bytes: &[u8], marker: u8) -> Option<usize> {
    bytes.iter().position(|&b| b == marker)
}

/// Checks if a character is a Markdown marker
fn is_marker_char(ch: u8) -> bool {
    ch == b'*' || ch == b'`' || ch == b'[' || ch == b']' || ch == b'(' || ch == b')' || ch == b'!'
}

#[derive(serde::Serialize)]
pub struct MarkdownInfo {
    pub html: String,
    pub source: String,
    pub length: usize,
}

#[derive(serde::Serialize)]
pub struct SourceUpdateResult {
    pub html: String,
    pub source: String,
    pub length: usize,
}

#[derive(serde::Serialize)]
pub struct EditorRenderResult {
    pub html: String,
    pub cursor_mapping: Vec<CursorMapping>,
    pub active_paragraph: usize,
    pub headings: Vec<crate::semantic::ast::HeadingInfo>,
}
