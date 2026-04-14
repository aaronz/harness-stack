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

/// Maps a source offset to DOM offset using the cursor mappings.
/// This is used for source→DOM conversion.
pub fn source_to_dom(mappings: &[CursorMapping], source_offset: usize) -> usize {
    if mappings.is_empty() {
        return source_offset;
    }

    // Find the largest mapping with source_offset <= target
    let mut result = mappings[0].dom_offset;
    for mapping in mappings {
        if mapping.source_offset <= source_offset {
            // Calculate the delta between source and dom for this mapping
            let delta = source_offset - mapping.source_offset;
            result = mapping.dom_offset + delta;
        } else {
            break;
        }
    }
    result
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
/// The mapping records the DOM offset BEFORE any HTML insertions for each source position.
pub fn build_cursor_mapping(source: &str) -> Vec<CursorMapping> {
    let mut mappings = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();
    let mut pos: usize = 0;
    let mut line: u32 = 0;
    let mut column: u32 = 0;

    // Track accumulated HTML length - this is the DOM offset
    let mut dom_offset: usize = 0;

    // Stack to track open formatting tags (for nested handling)
    let mut format_stack: Vec<(usize, &'static str)> = Vec::new();

    while pos <= len {
        // Record mapping at current position BEFORE processing
        mappings.push(CursorMapping::new(pos, dom_offset, line, column));

        if pos >= len {
            break;
        }

        let ch = chars[pos];

        // Handle CRLF line endings
        if ch == '\r' {
            // Skip CR if followed by LF
            if pos + 1 < len && chars[pos + 1] == '\n' {
                // Record mapping before the LF
                pos += 2;
                line += 1;
                column = 0;
                dom_offset += 1; // Only count LF in DOM
                continue;
            }
            // Bare CR - count as line ending
            pos += 1;
            line += 1;
            column = 0;
            dom_offset += 1;
            continue;
        }

        if ch == '\n' {
            pos += 1;
            line += 1;
            column = 0;
            dom_offset += 1;
            continue;
        }

        // Handle formatting markers
        let marker_result = detect_and_process_marker(&chars, pos, &mut format_stack);

        if let Some((chars_consumed, html_inserted)) = marker_result {
            dom_offset += html_inserted;
            pos += chars_consumed;
            column += chars_consumed as u32;
            continue;
        }

        // Regular character - advance
        column += 1;
        pos += 1;
        dom_offset += 1;
    }

    mappings
}

/// Detects and processes Markdown formatting markers.
///
/// Returns (chars_consumed, html_inserted) if a marker was processed, None otherwise.
/// The html_inserted is the net HTML length added (positive for opening, negative for closing
/// if we were to track it that way, but we add on both since we track the offset BEFORE).
fn detect_and_process_marker(
    chars: &[char],
    pos: usize,
    format_stack: &mut Vec<(usize, &'static str)>,
) -> Option<(usize, usize)> {
    let len = chars.len();
    if pos >= len {
        return None;
    }

    let ch = chars[pos];

    // Bold **...**
    if ch == '*' && pos + 1 < len && chars[pos + 1] == '*' {
        // Check if closing **
        if let Some(close_pos) = find_closing_double_marker(chars, pos + 2, '*') {
            // Valid bold: **content**
            let opening_len = 2;
            let closing_len = 2;
            let content_len = close_pos - (pos + 2);
            let total_source_len = opening_len + content_len + closing_len;
            // <strong> = 8, </strong> = 9
            return Some((total_source_len, 17)); // 8 + 9
        }
        // Check if opening **
        if pos + 2 < len {
            let next_ch = chars[pos + 2];
            if !is_marker_char(next_ch) {
                format_stack.push((pos, "strong"));
                return Some((2, 8)); // <strong> = 8 chars
            }
        }
    }

    // Italic *...* (single asterisk)
    if ch == '*' {
        // Look for closing *
        if let Some(close_pos) = find_closing_single_marker(chars, pos + 1, '*') {
            // Valid italic: *content*
            let opening_len = 1;
            let closing_len = 1;
            let content_len = close_pos - (pos + 1);
            let total_source_len = opening_len + content_len + closing_len;
            // <em> = 4, </em> = 5
            return Some((total_source_len, 9)); // 4 + 5
        }
        // Check if opening *
        if pos + 1 < len {
            let next_ch = chars[pos + 1];
            if !is_marker_char(next_ch) {
                format_stack.push((pos, "em"));
                return Some((1, 4)); // <em> = 4 chars
            }
        }
    }

    // Inline code `...`
    if ch == '`' {
        // Look for closing `
        if let Some(close_pos) = find_closing_single_marker(chars, pos + 1, '`') {
            // Valid code: `content`
            let opening_len = 1;
            let closing_len = 1;
            let content_len = close_pos - (pos + 1);
            let total_source_len = opening_len + content_len + closing_len;
            // <code> = 6, </code> = 7
            return Some((total_source_len, 13)); // 6 + 7
        }
    }

    // Link [text](url) or image ![alt](url)
    if ch == '[' || (pos > 0 && chars[pos - 1] == '[' && ch == '!') {
        // Look for ](url)
        if let Some(close_bracket) = find_char(chars, pos + 1, ']') {
            if close_bracket + 1 < chars.len() && chars[close_bracket + 1] == '(' {
                if let Some(close_paren) = find_char(chars, close_bracket + 2, ')') {
                    let url_start = close_bracket + 2;
                    let url_len = close_paren - url_start;
                    let text_start = if ch == '!' { pos + 1 } else { pos };
                    let text_len = close_bracket - text_start;
                    let total_source_len = if ch == '!' {
                        close_paren - pos + 1
                    } else {
                        close_paren - pos + 1
                    };
                    // <a href="url">text</a>
                    let html_len = 3 + 4 + url_len + 3 + text_len + 4; // <a href=""> + </a>
                    return Some((total_source_len, html_len));
                }
            }
        }
    }

    None
}

/// Finds a closing ** marker, returning the position after the closing **
fn find_closing_double_marker(chars: &[char], start: usize, marker: char) -> Option<usize> {
    let mut i = start;
    while i < chars.len() {
        if chars[i] == marker && i + 1 < chars.len() && chars[i + 1] == marker {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Finds a closing single marker, returning the position of the closing marker
fn find_closing_single_marker(chars: &[char], start: usize, marker: char) -> Option<usize> {
    let mut i = start;
    while i < chars.len() {
        if chars[i] == marker {
            return Some(i);
        }
        // Check if this char is itself a marker that shouldn't be crossed
        if chars[i] == '*' || chars[i] == '`' || chars[i] == '[' || chars[i] == ']' {
            // Don't cross other formatting
            break;
        }
        i += 1;
    }
    None
}

/// Finds a character in the slice
fn find_char(chars: &[char], start: usize, target: char) -> Option<usize> {
    chars[start..]
        .iter()
        .position(|&c| c == target)
        .map(|p| p + start)
}

/// Checks if a character is a Markdown marker
fn is_marker_char(ch: char) -> bool {
    ch == '*' || ch == '`' || ch == '[' || ch == ']' || ch == '(' || ch == ')' || ch == '!'
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
