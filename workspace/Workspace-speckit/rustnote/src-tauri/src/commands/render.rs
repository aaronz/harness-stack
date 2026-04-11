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

fn build_cursor_mapping(source: &str) -> Vec<CursorMapping> {
    let mut mappings = Vec::new();
    let mut line: u32 = 0;
    let mut column: u32 = 0;
    let mut char_index: usize = 0;
    let mut dom_offset: usize = 0;

    for ch in source.chars() {
        mappings.push(CursorMapping::new(char_index, dom_offset, line, column));

        if ch == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
        char_index += ch.len_utf8();
        dom_offset += 1;
    }

    mappings.push(CursorMapping::new(char_index, dom_offset, line, column));
    mappings
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
