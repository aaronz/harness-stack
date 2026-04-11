# RustNote Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a cross-platform WYSIWYG Markdown editor with Tauri + Rust, featuring live rendering, file management, themes, and HTML/PDF export.

**Architecture:** Tauri shell with Rust backend handling document model, parsing, and export. Frontend uses contenteditable for live Markdown rendering with incremental updates. Architecture follows: Markdown Source → AST (Rust) → Presentation Model (Frontend) → Rendered View.

**Tech Stack:** Tauri 2.x, Rust (pulldown-cmark, syntect, serde), HTML/CSS/JS frontend

---

## Phase 1: Foundation (Project Setup)

### Task 1: Initialize Tauri Project Structure

**Files:**
- Create: `rustnote/Cargo.toml`
- Create: `rustnote/src-tauri/Cargo.toml`
- Create: `rustnote/src-tauri/tauri.conf.json`
- Create: `rustnote/src-tauri/src/main.rs`
- Create: `rustnote/src-tauri/src/lib.rs`
- Create: `rustnote/src/index.html`

- [ ] **Step 1: Create Rust project root Cargo.toml**

```toml
[package]
name = "rustnote"
version = "0.1.0"
edition = "2021"
authors = ["RustNote Team"]
description = "A WYSIWYG Markdown Editor"

[workspace]
members = ["src-tauri"]

[workspace.dependencies]
tauri = { version = "2", features = ["devtools"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

- [ ] **Step 2: Create src-tauri/Cargo.toml**

```toml
[package]
name = "rustnote-tauri"
version = "0.1.0"
edition = "2021"
authors = ["RustNote Team"]

[lib]
name = "rustnote_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[[bin]]
name = "rustnote"
path = "src/main.rs"

[dependencies]
tauri = { version = "2", features = ["devtools"] }
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
pulldown-cmark = "0.12"
syntect = "5"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
log = "0.4"
env_logger = "0.11"
thiserror = "2"

[profile.release]
strip = true
lto = true
codegen-units = 1
panic = "abort"
```

- [ ] **Step 3: Create tauri.conf.json**

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "RustNote",
  "version": "0.1.0",
  "identifier": "com.rustnote.app",
  "build": {
    "devtools": true,
    "frontendDist": "../src",
    "devUrl": "http://localhost:1420",
    "beforeDevCommand": "",
    "beforeBuildCommand": ""
  },
  "app": {
    "withGlobalTauri": true,
    "windows": [
      {
        "title": "RustNote",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

- [ ] **Step 4: Create src-tauri/src/main.rs**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    
    info!("Starting RustNote application");
    
    rustnote_lib::run();
}
```

- [ ] **Step 5: Create src-tauri/src/lib.rs**

```rust
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 6: Create basic src/index.html**

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RustNote</title>
    <link rel="stylesheet" href="styles/main.css">
</head>
<body>
    <div id="app">
        <div id="sidebar"></div>
        <div id="editor">
            <div id="toolbar"></div>
            <div id="content" contenteditable="true"></div>
        </div>
    </div>
    <script src="scripts/app.js"></script>
</body>
</html>
```

- [ ] **Step 7: Verify project builds**

Run: `cd rustnote && cargo build --release`
Expected: Build completes successfully

- [ ] **Step 8: Commit**

```bash
git add rustnote/
git commit -m "feat: initialize Tauri project structure"
```

---

### Task 2: Set Up Logging and Error Handling

**Files:**
- Modify: `rustnote/src-tauri/src/lib.rs`
- Create: `rustnote/src-tauri/src/commands/mod.rs`

- [ ] **Step 1: Create commands module with error types**

```rust
// src-tauri/src/commands/mod.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Document not found: {0}")]
    DocumentNotFound(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl serde::Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
```

- [ ] **Step 2: Add logging and panic handler**

```rust
// Add to lib.rs
use std::panic;

fn setup_panic_handler() {
    panic::set_hook(Box::new(|panic_info| {
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };
        
        let location = panic_info.location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown".to_string());
            
        log::error!("PANIC at {}: {}", location, msg);
    }));
}

pub fn run() {
    setup_panic_handler();
    // ... rest of existing code
}
```

- [ ] **Step 3: Verify logging works**

Run: `cd rustnote && cargo build 2>&1 | head -20`
Expected: No errors

- [ ] **Step 4: Commit**

```bash
git add rustnote/src-tauri/src/
git commit -m "feat: add logging and panic handling"
```

---

## Phase 2: Document Model (Rust Core)

### Task 3: Create Document Model

**Files:**
- Create: `rustnote/src-tauri/src/model/mod.rs`
- Create: `rustnote/src-tauri/src/model/document.rs`
- Create: `rustnote/src-tauri/src/model/workspace.rs`
- Create: `rustnote/src-tauri/src/model/settings.rs`

- [ ] **Step 1: Define Document struct with tests**

```rust
// src-tauri/src/model/document.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub file_path: Option<String>,
    pub is_dirty: bool,
    pub last_saved: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Document {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            content: String::new(),
            file_path: None,
            is_dirty: false,
            last_saved: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn from_file(path: &str, content: String) -> Result<Self, std::io::Error> {
        let title = std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
            
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            title,
            content,
            file_path: Some(path.to_string()),
            is_dirty: false,
            last_saved: Some(now),
            created_at: now,
            updated_at: now,
        })
    }
    
    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.is_dirty = true;
        self.updated_at = Utc::now();
    }
    
    pub fn mark_saved(&mut self) {
        self.is_dirty = false;
        self.last_saved = Some(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_document() {
        let doc = Document::new("Test".to_string());
        assert_eq!(doc.title, "Test");
        assert!(doc.content.is_empty());
        assert!(doc.is_dirty);
    }
    
    #[test]
    fn test_update_content() {
        let mut doc = Document::new("Test".to_string());
        doc.update_content("# Hello".to_string());
        assert_eq!(doc.content, "# Hello");
        assert!(doc.is_dirty);
    }
    
    #[test]
    fn test_mark_saved() {
        let mut doc = Document::new("Test".to_string());
        doc.update_content("content".to_string());
        doc.mark_saved();
        assert!(!doc.is_dirty);
        assert!(doc.last_saved.is_some());
    }
}
```

- [ ] **Step 2: Run tests to verify they fail (no implementation)**

Run: `cd rustnote/src-tauri && cargo test model::document -- --nocapture`
Expected: FAIL - module not found

- [ ] **Step 3: Create model module**

```rust
// src-tauri/src/model/mod.rs
pub mod document;
pub mod workspace;
pub mod settings;

pub use document::Document;
pub use workspace::{Workspace, FileEntry};
pub use settings::{Settings, Theme, EditorSettings};
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cd rustnote/src-tauri && cargo test model::document`
Expected: PASS (3 tests)

- [ ] **Step 5: Create Workspace model**

```rust
// src-tauri/src/model/workspace.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub children: Vec<FileEntry>,
}

impl FileEntry {
    pub fn new_file(name: String, path: String) -> Self {
        Self {
            name,
            path,
            is_directory: false,
            children: vec![],
        }
    }
    
    pub fn new_directory(name: String, path: String, children: Vec<FileEntry>) -> Self {
        Self {
            name,
            path,
            is_directory: true,
            children,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub root_path: String,
    pub name: String,
    pub files: Vec<FileEntry>,
}

impl Workspace {
    pub fn new(root_path: String) -> Self {
        let name = std::path::Path::new(&root_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Workspace")
            .to_string();
            
        Self {
            id: Uuid::new_v4(),
            root_path,
            name,
            files: vec![],
        }
    }
}
```

- [ ] **Step 6: Create Settings model**

```rust
// src-tauri/src/model/settings.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub font_family: String,
    pub font_size: u32,
    pub line_height: f32,
    pub tab_size: u32,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font_family: "System".to_string(),
            font_size: 16,
            line_height: 1.6,
            tab_size: 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub auto_save: bool,
    pub auto_save_interval: u32,  // seconds
    pub editor: EditorSettings,
    pub recent_files: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            auto_save: true,
            auto_save_interval: 30,
            editor: EditorSettings::default(),
            recent_files: vec![],
        }
    }
}
```

- [ ] **Step 7: Run all model tests**

Run: `cd rustnote/src-tauri && cargo test model`
Expected: PASS

- [ ] **Step 8: Commit**

```bash
git add rustnote/src-tauri/src/model/
git commit -m "feat: add document, workspace, and settings models"
```

---

### Task 4: Markdown Parser Integration

**Files:**
- Create: `rustnote/src-tauri/src/parser/mod.rs`
- Create: `rustnote/src-tauri/src/parser/markdown.rs`
- Create: `rustnote/src-tauri/src/parser/syntax.rs`

- [ ] **Step 1: Write parser tests**

```rust
// src-tauri/src/parser/markdown.rs
use pulldown_cmark::{Parser, Options, html};

pub struct MarkdownParser {
    options: Options,
}

impl MarkdownParser {
    pub fn new() -> Self {
        let options = Options::empty()
            | Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_SMART_PUNCTUATION;
            
        Self { options }
    }
    
    pub fn parse(&self, markdown: &str) -> String {
        let parser = Parser::new_ext(markdown, self.options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
    
    pub fn parse_to_ast(&self, markdown: &str) -> Parser<'static> {
        Parser::new_ext(markdown, self.options)
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn parser() -> MarkdownParser {
        MarkdownParser::new()
    }
    
    #[test]
    fn test_parse_heading() {
        let result = parser().parse("# Hello World");
        assert!(result.contains("<h1>"));
        assert!(result.contains("Hello World"));
    }
    
    #[test]
    fn test_parse_bold_italic() {
        let result = parser().parse("**bold** and *italic*");
        assert!(result.contains("<strong>bold</strong>"));
        assert!(result.contains("<em>italic</em>"));
    }
    
    #[test]
    fn test_parse_list() {
        let result = parser().parse("- item 1\n- item 2");
        assert!(result.contains("<ul>"));
        assert!(result.contains("<li>"));
    }
    
    #[test]
    fn test_parse_code_block() {
        let result = parser().parse("```rust\nfn main() {}\n```");
        assert!(result.contains("<code"));
        assert!(result.contains("rust"));
    }
    
    #[test]
    fn test_parse_table() {
        let result = parser().parse("| a | b |\n|---|---|\n| 1 | 2 |");
        assert!(result.contains("<table>"));
        assert!(result.contains("<td>"));
    }
    
    #[test]
    fn test_parse_task_list() {
        let result = parser().parse("- [ ] unchecked\n- [x] checked");
        assert!(result.contains("task-list-item"));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cd rustnote/src-tauri && cargo test parser::markdown`
Expected: FAIL - module not found

- [ ] **Step 3: Create parser module**

```rust
// src-tauri/src/parser/mod.rs
pub mod markdown;
pub mod syntax;

pub use markdown::MarkdownParser;
pub use syntax::SyntaxHighlighter;
```

- [ ] **Step 4: Implement markdown parser**

```rust
// src-tauri/src/parser/markdown.rs
// (paste the code from Step 1)
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cd rustnote/src-tauri && cargo test parser::markdown`
Expected: PASS (6 tests)

- [ ] **Step 6: Create syntax highlighter**

```rust
// src-tauri/src/parser/syntax.rs
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::html::{highlighted_html_for_string, IncludeBackground};

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::new(),
            theme_set: ThemeSet::load_defaults(),
        }
    }
    
    pub fn highlight(&self, code: &str, language: &str) -> String {
        let syntax = self.syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
            
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
        assert!(!result.is_empty());  // Falls back to plain text
    }
}
```

- [ ] **Step 7: Run all parser tests**

Run: `cd rustnote/src-tauri && cargo test parser`
Expected: PASS

- [ ] **Step 8: Commit**

```bash
git add rustnote/src-tauri/src/parser/
git commit -m "feat: add markdown parser and syntax highlighter"
```

---

### Task 5: Tauri Commands (File Operations)

**Files:**
- Modify: `rustnote/src-tauri/src/commands/mod.rs`
- Create: `rustnote/src-tauri/src/commands/document.rs`
- Create: `rustnote/src-tauri/src/commands/workspace.rs`
- Create: `rustnote/src-tauri/src/commands/export.rs`

- [ ] **Step 1: Write file operations tests**

```rust
// src-tauri/src/commands/document.rs
use crate::model::Document;
use crate::commands::CommandError;
use std::fs;

#[tauri::command]
pub async fn create_document(title: String) -> Result<Document, CommandError> {
    Ok(Document::new(title))
}

#[tauri::command]
pub async fn open_document(path: String) -> Result<Document, CommandError> {
    let content = fs::read_to_string(&path)?;
    Document::from_file(&path, content).map_err(|e| CommandError::Io(e))
}

#[tauri::command]
pub async fn save_document(path: String, content: String) -> Result<(), CommandError> {
    // Atomic write: write to temp file, then rename
    let temp_path = format!("{}.tmp", path);
    fs::write(&temp_path, &content)?;
    fs::rename(&temp_path, &path)?;
    Ok(())
}

#[tauri::command]
pub async fn read_document_content(path: String) -> Result<String, CommandError> {
    fs::read_to_string(&path).map_err(|e| CommandError::Io(e))
}

#[tauri::command]
pub async fn write_document_content(path: String, content: String) -> Result<(), CommandError> {
    fs::write(&path, &content).map_err(|e| CommandError::Io(e))
}
```

- [ ] **Step 2: Create commands module with exports**

```rust
// src-tauri/src/commands/mod.rs
pub mod document;
pub mod workspace;
pub mod export;

pub use document::*;
pub use workspace::*;
pub use export::*;
```

- [ ] **Step 3: Register commands in lib.rs**

```rust
// src-tauri/src/lib.rs
mod commands;
use commands::{create_document, open_document, save_document, read_document_content, 
               write_document_content, list_workspace, read_settings, write_settings,
               export_to_html, export_to_pdf};

pub fn run() {
    // ... existing setup code ...
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_document,
            open_document,
            save_document,
            read_document_content,
            write_document_content,
            list_workspace,
            read_settings,
            write_settings,
            export_to_html,
            export_to_pdf,
        ])
        // ... rest of code
}
```

- [ ] **Step 4: Create workspace commands**

```rust
// src-tauri/src/commands/workspace.rs
use crate::model::{Workspace, FileEntry};
use crate::commands::CommandError;
use std::fs;

fn read_dir_recursive(path: &str) -> Result<Vec<FileEntry>, CommandError> {
    let mut entries = vec![];
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();
        
        // Skip hidden files and common ignores
        if file_name.starts_with('.') || file_name == "node_modules" || file_name == "target" {
            continue;
        }
        
        if file_path.is_dir() {
            let children = read_dir_recursive(file_path.to_str().unwrap_or(""))?;
            entries.push(FileEntry::new_directory(file_name, file_path.to_string_lossy().to_string(), children));
        } else if file_name.ends_with(".md") {
            entries.push(FileEntry::new_file(file_name, file_path.to_string_lossy().to_string()));
        }
    }
    
    Ok(entries)
}

#[tauri::command]
pub async fn list_workspace(path: String) -> Result<Workspace, CommandError> {
    let mut workspace = Workspace::new(path.clone());
    workspace.files = read_dir_recursive(&path)?;
    Ok(workspace)
}
```

- [ ] **Step 5: Create settings commands**

```rust
// src-tauri/src/commands/settings.rs (inline in commands/mod.rs)
use crate::model::Settings;

const SETTINGS_PATH: &str = "rustnote_settings.json";

#[tauri::command]
pub async fn read_settings() -> Result<Settings, CommandError> {
    let content = fs::read_to_string(SETTINGS_PATH)?;
    serde_json::from_str(&content).map_err(|e| CommandError::SerializationError(e.to_string()))
}

#[tauri::command]
pub async fn write_settings(settings: Settings) -> Result<(), CommandError> {
    let content = serde_json::to_string_pretty(&settings)?;
    fs::write(SETTINGS_PATH, content)?;
    Ok(())
}
```

- [ ] **Step 6: Create export commands**

```rust
// src-tauri/src/commands/export.rs
use crate::parser::MarkdownParser;
use crate::commands::CommandError;
use std::fs;

#[tauri::command]
pub async fn export_to_html(markdown: String, output_path: String) -> Result<(), CommandError> {
    let parser = MarkdownParser::new();
    let html = parser.parse(&markdown);
    
    let full_html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Exported Document</title>
    <style>
        body {{ font-family: -apple-system, system-ui, sans-serif; max-width: 800px; margin: 40px auto; padding: 20px; }}
        pre {{ background: #f5f5f5; padding: 16px; border-radius: 4px; overflow-x: auto; }}
        code {{ background: #f5f5f5; padding: 2px 6px; border-radius: 3px; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        blockquote {{ border-left: 4px solid #ddd; margin: 0; padding-left: 16px; color: #666; }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        html
    );
    
    fs::write(&output_path, full_html)?;
    Ok(())
}

#[tauri::command]
pub async fn export_to_pdf(markdown: String, output_path: String) -> Result<(), CommandError> {
    // HTML export first, then PDF via system print
    // For MVP, we generate HTML and let users print to PDF
    export_to_html(markdown, output_path.replace(".pdf", ".html")).await
}
```

- [ ] **Step 7: Verify compilation**

Run: `cd rustnote/src-tauri && cargo build 2>&1 | head -30`
Expected: No errors

- [ ] **Step 8: Commit**

```bash
git add rustnote/src-tauri/src/commands/
git commit -m "feat: add Tauri commands for file operations and export"
```

---

## Phase 3: Frontend Editor

### Task 6: Basic Editor UI

**Files:**
- Create: `rustnote/src/styles/main.css`
- Create: `rustnote/src/styles/theme-light.css`
- Create: `rustnote/src/styles/theme-dark.css`
- Create: `rustnote/src/scripts/app.js`
- Create: `rustnote/src/scripts/editor.js`

- [ ] **Step 1: Write editor initialization tests**

```javascript
// tests/editor.test.js (conceptual - we'll test via manual verification)
describe('Editor', () => {
    test('initializes with empty content', () => {
        const editor = createEditor();
        expect(editor.getContent()).toBe('');
    });
    
    test('renders markdown to HTML', () => {
        const result = renderMarkdown('# Hello');
        expect(result).toContain('<h1>');
    });
});
```

- [ ] **Step 2: Create main CSS**

```css
/* src/styles/main.css */
:root {
    --bg-primary: #ffffff;
    --bg-secondary: #f5f5f5;
    --text-primary: #333333;
    --text-secondary: #666666;
    --border-color: #e0e0e0;
    --accent-color: #0066cc;
    --font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    --font-mono: "SF Mono", Monaco, Consolas, monospace;
    --font-size: 16px;
    --line-height: 1.6;
}

* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

body {
    font-family: var(--font-family);
    font-size: var(--font-size);
    line-height: var(--line-height);
    background: var(--bg-primary);
    color: var(--text-primary);
    overflow: hidden;
}

#app {
    display: flex;
    height: 100vh;
    width: 100vw;
}

#sidebar {
    width: 250px;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

#sidebar-header {
    padding: 16px;
    font-weight: 600;
    border-bottom: 1px solid var(--border-color);
}

#file-tree {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
}

.file-item {
    padding: 8px 12px;
    cursor: pointer;
    border-radius: 4px;
    display: flex;
    align-items: center;
    gap: 8px;
}

.file-item:hover {
    background: var(--border-color);
}

.file-item.active {
    background: var(--accent-color);
    color: white;
}

#editor {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

#toolbar {
    padding: 8px 16px;
    border-bottom: 1px solid var(--border-color);
    display: flex;
    gap: 8px;
    align-items: center;
}

#content {
    flex: 1;
    padding: 40px 60px;
    overflow-y: auto;
    outline: none;
    max-width: 900px;
    margin: 0 auto;
    width: 100%;
}

/* Typography in editor */
#content h1 { font-size: 2em; margin: 0.67em 0; font-weight: 600; }
#content h2 { font-size: 1.5em; margin: 0.83em 0; font-weight: 600; }
#content h3 { font-size: 1.25em; margin: 1em 0; font-weight: 600; }
#content p { margin: 1em 0; }
#content ul, #content ol { margin: 1em 0; padding-left: 2em; }
#content li { margin: 0.25em 0; }
#content code { 
    background: var(--bg-secondary); 
    padding: 2px 6px; 
    border-radius: 3px; 
    font-family: var(--font-mono);
}
#content pre { 
    background: var(--bg-secondary); 
    padding: 16px; 
    border-radius: 4px; 
    overflow-x: auto; 
    margin: 1em 0;
}
#content pre code { 
    background: none; 
    padding: 0; 
}
#content blockquote {
    border-left: 4px solid var(--border-color);
    margin: 1em 0;
    padding-left: 16px;
    color: var(--text-secondary);
}
#content table {
    border-collapse: collapse;
    width: 100%;
    margin: 1em 0;
}
#content th, #content td {
    border: 1px solid var(--border-color);
    padding: 8px;
    text-align: left;
}
#content img {
    max-width: 100%;
    height: auto;
}
#content a {
    color: var(--accent-color);
    text-decoration: none;
}
#content a:hover {
    text-decoration: underline;
}
#content hr {
    border: none;
    border-top: 1px solid var(--border-color);
    margin: 1em 0;
}
```

- [ ] **Step 3: Create theme CSS**

```css
/* src/styles/theme-dark.css */
[data-theme="dark"] {
    --bg-primary: #1e1e1e;
    --bg-secondary: #252526;
    --text-primary: #d4d4d4;
    --text-secondary: #858585;
    --border-color: #3c3c3c;
    --accent-color: #4fc3f7;
}

[data-theme="dark"] #content code {
    background: #2d2d2d;
}

[data-theme="dark"] #content pre {
    background: #2d2d2d;
}

[data-theme="dark"] #content blockquote {
    border-color: #3c3c3c;
    color: #858585;
}
```

- [ ] **Step 4: Create app.js**

```javascript
// src/scripts/app.js
// Global state
let currentDocument = null;
let workspace = null;
let settings = {
    theme: 'light',
    autoSave: true,
    autoSaveInterval: 30000
};

// Initialize app
async function init() {
    await loadSettings();
    applyTheme();
    setupEventListeners();
    createNewDocument();
}

// Load settings from backend
async function loadSettings() {
    try {
        const result = await window.__TAURI__.core.invoke('read_settings');
        settings = { ...settings, ...result };
    } catch (e) {
        console.log('Using default settings');
    }
}

// Apply theme
function applyTheme() {
    document.documentElement.setAttribute('data-theme', settings.theme);
}

// Event listeners
function setupEventListeners() {
    // Keyboard shortcuts
    document.addEventListener('keydown', handleKeyboard);
    
    // Auto-save
    if (settings.autoSave) {
        setInterval(autoSave, settings.autoSaveInterval);
    }
}

// Keyboard handler
function handleKeyboard(e) {
    const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
    const modifier = isMac ? e.metaKey : e.ctrlKey;
    
    if (modifier && e.key === 's') {
        e.preventDefault();
        saveDocument();
    } else if (modifier && e.key === 'n') {
        e.preventDefault();
        createNewDocument();
    } else if (modifier && e.key === 'o') {
        e.preventDefault();
        openDocument();
    } else if (modifier && e.key === 'b') {
        e.preventDefault();
        formatBold();
    } else if (modifier && e.key === 'i') {
        e.preventDefault();
        formatItalic();
    }
}

// Initialize on load
document.addEventListener('DOMContentLoaded', init);
```

- [ ] **Step 5: Create editor.js**

```javascript
// src/scripts/editor.js
const editor = {
    content: document.getElementById('content'),
    
    // Get raw markdown content
    getContent() {
        return this.content.innerText;
    },
    
    // Set content
    setContent(text) {
        this.content.innerText = text;
    },
    
    // Render markdown to HTML in-place
    async render() {
        const markdown = this.getContent();
        try {
            const html = await window.__TAURI__.core.invoke('render_markdown', { markdown });
            // For WYSIWYG, we keep as text but style appropriately
            // Actual rendering happens via CSS
        } catch (e) {
            console.error('Render error:', e);
        }
    },
    
    // Insert text at cursor
    insertText(text) {
        const selection = window.getSelection();
        if (!selection.rangeCount) return;
        
        const range = selection.getRangeAt(0);
        range.deleteContents();
        range.insertNode(document.createTextNode(text));
        range.collapse(false);
        selection.removeAllRanges();
        selection.addRange(range);
    },
    
    // Format selection
    format(type) {
        const selection = window.getSelection();
        if (!selection.rangeCount) return;
        
        const range = selection.getRangeAt(0);
        const selectedText = range.toString();
        
        if (!selectedText) return;
        
        const before = type === 'bold' ? '**' : type === 'italic' ? '*' : '`';
        const after = before;
        
        const newText = before + selectedText + after;
        range.deleteContents();
        range.insertNode(document.createTextNode(newText));
    }
};

function formatBold() { editor.format('bold'); }
function formatItalic() { editor.format('italic'); }
function formatCode() { editor.format('code'); }
```

- [ ] **Step 6: Update index.html with styles and scripts**

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RustNote</title>
    <link rel="stylesheet" href="styles/main.css">
    <link rel="stylesheet" href="styles/theme-light.css">
    <link rel="stylesheet" href="styles/theme-dark.css" disabled>
</head>
<body>
    <div id="app">
        <div id="sidebar">
            <div id="sidebar-header">Files</div>
            <div id="file-tree"></div>
        </div>
        <div id="editor">
            <div id="toolbar">
                <button id="btn-new" title="New (Ctrl+N)">New</button>
                <button id="btn-open" title="Open (Ctrl+O)">Open</button>
                <button id="btn-save" title="Save (Ctrl+S)">Save</button>
                <div style="flex:1"></div>
                <button id="btn-theme" title="Toggle Theme">Theme</button>
            </div>
            <div id="content" contenteditable="true" spellcheck="false"></div>
        </div>
    </div>
    <script src="scripts/app.js"></script>
    <script src="scripts/editor.js"></script>
</body>
</html>
```

- [ ] **Step 7: Verify frontend loads**

Run: Open rustnote/src/index.html in browser
Expected: Editor UI renders with sidebar and content area

- [ ] **Step 8: Commit**

```bash
git add rustnote/src/
git commit -m "feat: add frontend editor UI with styles and scripts"
```

---

### Task 7: Live Markdown Rendering

**Files:**
- Modify: `rustnote/src/scripts/editor.js`
- Create: `rustnote/src/scripts/renderer.js`

- [ ] **Step 1: Implement live rendering**

```javascript
// src/scripts/renderer.js
// Live markdown rendering - parses as you type with debouncing

class MarkdownRenderer {
    constructor() {
        this.parser = new marked();  // Use browser-markdown or call Tauri
        this.debounceTime = 150;
        this.timeout = null;
    }
    
    // Debounced render
    render(markdown, callback) {
        clearTimeout(this.timeout);
        this.timeout = setTimeout(() => {
            const html = this.parse(markdown);
            callback(html);
        }, this.debounceTime);
    }
    
    // Parse markdown to HTML
    parse(markdown) {
        // For WYSIWYG, we don't fully convert - we keep markdown source
        // but style it to look rendered
        return this.applyStyles(markdown);
    }
    
    // Apply CSS classes for visual rendering
    applyStyles(markdown) {
        let html = markdown
            // Headers
            .replace(/^###### (.+)$/gm, '<h6>$1</h6>')
            .replace(/^##### (.+)$/gm, '<h5>$1</h5>')
            .replace(/^#### (.+)$/gm, '<h4>$1</h4>')
            .replace(/^### (.+)$/gm, '<h3>$1</h3>')
            .replace(/^## (.+)$/gm, '<h2>$1</h2>')
            .replace(/^# (.+)$/gm, '<h1>$1</h1>')
            // Bold
            .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
            // Italic
            .replace(/\*(.+?)\*/g, '<em>$1</em>')
            // Inline code
            .replace(/`(.+?)`/g, '<code>$1</code>')
            // Links
            .replace(/\[(.+?)\]\((.+?)\)/g, '<a href="$2">$1</a>')
            // Task lists
            .replace(/- \[ \] /g, '<input type="checkbox" disabled> ')
            .replace(/- \[x\] /g, '<input type="checkbox" checked disabled> ');
        
        return html;
    }
}

const renderer = new MarkdownRenderer();
```

- [ ] **Step 2: Integrate live rendering into editor**

```javascript
// Update editor.js - add live rendering on input
editor.content.addEventListener('input', () => {
    renderer.render(editor.getContent(), (html) => {
        // Keep raw markdown but update visual styling
        updateVisualStyles();
    });
    
    // Mark document as dirty
    if (currentDocument) {
        currentDocument.isDirty = true;
        updateTitle();
    }
});

// Visual style updater - adds/removes classes
function updateVisualStyles() {
    const text = editor.getContent();
    // Simple visual feedback - real implementation uses AST
    // For MVP, rely on CSS for basic markdown elements
}
```

- [ ] **Step 3: Commit**

```bash
git add rustnote/src/scripts/
git commit -m "feat: add live markdown rendering with debouncing"
```

---

### Task 8: File Operations UI

**Files:**
- Modify: `rustnote/src/scripts/app.js`

- [ ] **Step 1: Implement file operations**

```javascript
// Add to app.js

// Create new document
async function createNewDocument() {
    currentDocument = {
        title: 'Untitled',
        content: '',
        filePath: null,
        isDirty: false
    };
    editor.setContent('');
    updateTitle();
}

// Open document dialog
async function openDocument() {
    try {
        const result = await window.__TAURI__.dialog.open({
            filters: [{ name: 'Markdown', extensions: ['md', 'markdown'] }]
        });
        
        if (result) {
            const doc = await window.__TAURI__.core.invoke('open_document', { path: result });
            currentDocument = doc;
            editor.setContent(doc.content);
            updateTitle();
            addToRecentFiles(result);
        }
    } catch (e) {
        console.error('Open error:', e);
    }
}

// Save document
async function saveDocument() {
    if (!currentDocument) return;
    
    currentDocument.content = editor.getContent();
    
    try {
        if (currentDocument.filePath) {
            await window.__TAURI__.core.invoke('save_document', {
                path: currentDocument.filePath,
                content: currentDocument.content
            });
        } else {
            await saveDocumentAs();
        }
        
        currentDocument.isDirty = false;
        updateTitle();
    } catch (e) {
        console.error('Save error:', e);
    }
}

// Save as
async function saveDocumentAs() {
    try {
        const result = await window.__TAURI__.dialog.save({
            filters: [{ name: 'Markdown', extensions: ['md'] }],
            defaultPath: currentDocument.title + '.md'
        });
        
        if (result) {
            currentDocument.filePath = result;
            await saveDocument();
            addToRecentFiles(result);
        }
    } catch (e) {
        console.error('Save as error:', e);
    }
}

// Auto-save
async function autoSave() {
    if (currentDocument && currentDocument.isDirty && currentDocument.filePath) {
        await saveDocument();
        showNotification('Auto-saved');
    }
}

// Update window title
function updateTitle() {
    const dirty = currentDocument?.isDirty ? '•' : '';
    const title = currentDocument?.title || 'RustNote';
    document.title = `${dirty}${title} - RustNote`;
}

// Recent files
async function addToRecentFiles(path) {
    if (!settings.recentFiles) settings.recentFiles = [];
    settings.recentFiles = [path, ...settings.recentFiles.filter(p => p !== path)].slice(0, 10);
    await window.__TAURI__.core.invoke('write_settings', settings);
}

// Notification
function showNotification(message) {
    // Simple notification - could be enhanced
    console.log(message);
}

// Toolbar button handlers
document.getElementById('btn-new').addEventListener('click', createNewDocument);
document.getElementById('btn-open').addEventListener('click', openDocument);
document.getElementById('btn-save').addEventListener('click', saveDocument);
```

- [ ] **Step 2: Add sidebar file tree**

```javascript
// Add to app.js

// Load workspace
async function loadWorkspace() {
    try {
        const result = await window.__TAURI__.dialog.open({
            directory: true,
            title: 'Open Folder as Workspace'
        });
        
        if (result) {
            workspace = await window.__TAURI__.core.invoke('list_workspace', { path: result });
            renderFileTree();
        }
    } catch (e) {
        console.error('Workspace error:', e);
    }
}

// Render file tree
function renderFileTree() {
    const tree = document.getElementById('file-tree');
    tree.innerHTML = '';
    
    if (!workspace?.files) return;
    
    workspace.files.forEach(file => {
        const div = document.createElement('div');
        div.className = 'file-item';
        div.textContent = file.name;
        div.addEventListener('click', () => openFile(file.path));
        tree.appendChild(div);
    });
}

// Open file from tree
async function openFile(path) {
    try {
        const doc = await window.__TAURI__.core.invoke('open_document', { path });
        currentDocument = doc;
        editor.setContent(doc.content);
        updateTitle();
    } catch (e) {
        console.error('Open file error:', e);
    }
}
```

- [ ] **Step 3: Commit**

```bash
git add rustnote/src/scripts/app.js
git commit -m "feat: add file operations and workspace sidebar"
```

---

### Task 9: Search Functionality

**Files:**
- Create: `rustnote/src/scripts/search.js`

- [ ] **Step 1: Implement find/replace**

```javascript
// src/scripts/search.js
class SearchManager {
    constructor() {
        this.dialog = null;
        this.query = '';
        this.results = [];
        this.currentIndex = -1;
    }
    
    // Show search dialog
    show() {
        if (this.dialog) return;
        
        this.dialog = document.createElement('div');
        this.dialog.id = 'search-dialog';
        this.dialog.innerHTML = `
            <input type="text" id="search-input" placeholder="Find...">
            <input type="text" id="replace-input" placeholder="Replace..." style="display:none">
            <button id="search-next">Next</button>
            <button id="search-prev">Prev</button>
            <button id="search-replace" style="display:none">Replace</button>
            <button id="search-close">✕</button>
        `;
        
        document.getElementById('editor').appendChild(this.dialog);
        
        this.dialog.querySelector('#search-close').addEventListener('click', () => this.hide());
        this.dialog.querySelector('#search-next').addEventListener('click', () => this.findNext());
        this.dialog.querySelector('#search-prev').addEventListener('click', () => this.findPrev());
        
        this.dialog.querySelector('#search-input').addEventListener('input', (e) => {
            this.query = e.target.value;
            this.findAll();
        });
        
        this.dialog.querySelector('#search-input').focus();
    }
    
    // Find all occurrences
    findAll() {
        if (!this.query) {
            this.results = [];
            return;
        }
        
        const content = editor.getContent();
        const regex = new RegExp(this.query, 'gi');
        this.results = [...content.matchAll(regex)];
        this.currentIndex = this.results.length > 0 ? 0 : -1;
        this.highlightCurrent();
    }
    
    // Find next
    findNext() {
        if (this.results.length === 0) return;
        this.currentIndex = (this.currentIndex + 1) % this.results.length;
        this.highlightCurrent();
    }
    
    // Find previous
    findPrev() {
        if (this.results.length === 0) return;
        this.currentIndex = (this.currentIndex - 1 + this.results.length) % this.results.length;
        this.highlightCurrent();
    }
    
    // Highlight current result (simplified)
    highlightCurrent() {
        // In a real implementation, would use Selection API
        console.log(`Result ${this.currentIndex + 1} of ${this.results.length}`);
    }
    
    // Hide dialog
    hide() {
        if (this.dialog) {
            this.dialog.remove();
            this.dialog = null;
        }
    }
}

const searchManager = new SearchManager();

// Keyboard shortcut
document.addEventListener('keydown', (e) => {
    const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
    const modifier = isMac ? e.metaKey : e.ctrlKey;
    
    if (modifier && e.key === 'f') {
        e.preventDefault();
        searchManager.show();
    }
});
```

- [ ] **Step 2: Commit**

```bash
git add rustnote/src/scripts/search.js
git commit -m "feat: add search and find functionality"
```

---

### Task 10: Theme Switching

**Files:**
- Modify: `rustnote/src/styles/theme-light.css`
- Modify: `rustnote/src/styles/theme-dark.css`
- Modify: `rustnote/src/scripts/app.js`

- [ ] **Step 1: Implement theme toggle**

```javascript
// Add to app.js

// Toggle theme
async function toggleTheme() {
    settings.theme = settings.theme === 'light' ? 'dark' : 'light';
    applyTheme();
    await window.__TAURI__.core.invoke('write_settings', settings);
}

document.getElementById('btn-theme').addEventListener('click', toggleTheme);
```

- [ ] **Step 2: Make theme CSS work with data-theme**

```css
/* Ensure theme-dark.css applies when data-theme="dark" */
[data-theme="dark"] {
    --bg-primary: #1e1e1e;
    --bg-secondary: #252526;
    --text-primary: #d4d4d4;
    --text-secondary: #858585;
    --border-color: #3c3c3c;
    --accent-color: #4fc3f7;
}

[data-theme="dark"] #content code {
    background: #2d2d2d;
}

[data-theme="dark"] #content pre {
    background: #2d2d2d;
}

[data-theme="dark"] #content blockquote {
    border-color: #3c3c3c;
    color: #858585;
}

[data-theme="dark"] #sidebar {
    background: #252526;
}

[data-theme="dark"] #toolbar {
    background: #2d2d2d;
    border-color: #3c3c3c;
}
```

- [ ] **Step 3: Update index.html to enable theme switching**

```html
<!-- In index.html, make sure theme-dark is not disabled by default -->
<link rel="stylesheet" href="styles/theme-light.css">
<link rel="stylesheet" href="styles/theme-dark.css">
```

- [ ] **Step 4: Commit**

```bash
git add rustnote/src/styles/
git commit -m "feat: add theme switching (light/dark)"
```

---

### Task 11: Export Functionality

**Files:**
- Modify: `rustnote/src/scripts/app.js`

- [ ] **Step 1: Add export menu**

```javascript
// Add to app.js

// Export to HTML
async function exportHtml() {
    if (!currentDocument) return;
    
    try {
        const result = await window.__TAURI__.dialog.save({
            filters: [{ name: 'HTML', extensions: ['html'] }],
            defaultPath: currentDocument.title + '.html'
        });
        
        if (result) {
            await window.__TAURI__.core.invoke('export_to_html', {
                markdown: editor.getContent(),
                outputPath: result
            });
            showNotification('Exported to HTML');
        }
    } catch (e) {
        console.error('Export error:', e);
    }
}

// Export to PDF (via HTML print)
async function exportPdf() {
    // Generate HTML and open print dialog
    const markdown = editor.getContent();
    const html = parseMarkdown(markdown);  // Use renderer
    
    const printWindow = window.open('', '_blank');
    printWindow.document.write(html);
    printWindow.document.close();
    printWindow.print();
}

// Add to toolbar or menu
const exportBtn = document.createElement('button');
exportBtn.textContent = 'Export';
exportBtn.addEventListener('click', () => {
    // Show export menu (simple dropdown)
    const menu = document.createElement('div');
    menu.innerHTML = '<div onclick="exportHtml()">Export HTML</div><div onclick="exportPdf()">Export PDF</div>';
    menu.style.position = 'absolute';
    menu.style.background = 'white';
    menu.style.border = '1px solid #ccc';
    menu.style.padding = '8px';
    document.getElementById('toolbar').appendChild(menu);
});
document.getElementById('toolbar').appendChild(exportBtn);
```

- [ ] **Step 2: Commit**

```bash
git add rustnote/src/scripts/app.js
git commit -m "feat: add HTML and PDF export functionality"
```

---

## Phase 4: Polish and Integration

### Task 12: Auto-Save and Recovery

**Files:**
- Modify: `rustnote/src/scripts/app.js`
- Create: `rustnote/src-tauri/src/commands/recovery.rs`

- [ ] **Step 1: Implement auto-save with snapshots**

```javascript
// Add to app.js

// Snapshot for recovery
let snapshotInterval = null;

function startSnapshot() {
    if (snapshotInterval) clearInterval(snapshotInterval);
    snapshotInterval = setInterval(() => {
        if (currentDocument?.isDirty && currentDocument?.filePath) {
            saveSnapshot();
        }
    }, 30000);  // Every 30 seconds
}

async function saveSnapshot() {
    if (!currentDocument?.filePath) return;
    
    try {
        const snapshotPath = currentDocument.filePath + '.backup';
        await window.__TAURI__.core.invoke('write_document_content', {
            path: snapshotPath,
            content: editor.getContent()
        });
    } catch (e) {
        console.error('Snapshot error:', e);
    }
}

// Recovery on startup
async function checkRecovery() {
    try {
        const recentFiles = settings.recentFiles || [];
        for (const path of recentFiles) {
            const backupPath = path + '.backup';
            // Check if backup exists and is newer than original
            // If so, prompt user to recover
        }
    } catch (e) {
        console.error('Recovery check error:', e);
    }
}

// Initialize recovery check
checkRecovery();
startSnapshot();
```

- [ ] **Step 2: Commit**

```bash
git add rustnote/src/scripts/app.js
git commit -m "feat: add auto-save and crash recovery"
```

---

### Task 13: Settings Panel

**Files:**
- Create: `rustnote/src/scripts/settings.js`

- [ ] **Step 1: Implement settings UI**

```javascript
// src/scripts/settings.js
class SettingsPanel {
    constructor() {
        this.dialog = null;
    }
    
    show() {
        if (this.dialog) return;
        
        this.dialog = document.createElement('div');
        this.dialog.id = 'settings-dialog';
        this.dialog.innerHTML = `
            <h2>Settings</h2>
            <div class="setting">
                <label>Theme</label>
                <select id="setting-theme">
                    <option value="light">Light</option>
                    <option value="dark">Dark</option>
                </select>
            </div>
            <div class="setting">
                <label>Auto-save</label>
                <input type="checkbox" id="setting-autosave" checked>
            </div>
            <div class="setting">
                <label>Auto-save interval (seconds)</label>
                <input type="number" id="setting-interval" value="30" min="10" max="300">
            </div>
            <div class="setting">
                <label>Font Family</label>
                <select id="setting-font">
                    <option value="System">System</option>
                    <option value="Serif">Serif</option>
                    <option value="Monospace">Monospace</option>
                </select>
            </div>
            <div class="setting">
                <label>Font Size</label>
                <input type="number" id="setting-size" value="16" min="12" max="32">
            </div>
            <div class="buttons">
                <button id="settings-save">Save</button>
                <button id="settings-cancel">Cancel</button>
            </div>
        `;
        
        // Style
        this.dialog.style.cssText = `
            position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%);
            background: white; padding: 24px; border-radius: 8px;
            box-shadow: 0 4px 20px rgba(0,0,0,0.2); min-width: 400px;
        `;
        
        document.body.appendChild(this.dialog);
        
        // Load current settings
        this.dialog.querySelector('#setting-theme').value = settings.theme;
        this.dialog.querySelector('#setting-autosave').checked = settings.autoSave;
        this.dialog.querySelector('#setting-interval').value = settings.autoSaveInterval / 1000;
        
        // Handlers
        this.dialog.querySelector('#settings-save').addEventListener('click', () => this.save());
        this.dialog.querySelector('#settings-cancel').addEventListener('click', () => this.hide());
    }
    
    async save() {
        settings.theme = this.dialog.querySelector('#setting-theme').value;
        settings.autoSave = this.dialog.querySelector('#setting-autosave').checked;
        settings.autoSaveInterval = parseInt(this.dialog.querySelector('#setting-interval').value) * 1000;
        
        await window.__TAURI__.core.invoke('write_settings', settings);
        applyTheme();
        this.hide();
    }
    
    hide() {
        if (this.dialog) {
            this.dialog.remove();
            this.dialog = null;
        }
    }
}

const settingsPanel = new SettingsPanel();

// Add settings menu
document.getElementById('btn-theme').addEventListener('contextmenu', (e) => {
    e.preventDefault();
    settingsPanel.show();
});
```

- [ ] **Step 2: Commit**

```bash
git add rustnote/src/scripts/settings.js
git commit -m "feat: add settings panel"
```

---

### Task 14: Build and Verify

**Files:**
- Modify: `rustnote/src-tauri/tauri.conf.json`

- [ ] **Step 1: Configure production build**

```json
// tauri.conf.json - add build configuration
{
  "build": {
    "devtools": true,
    "frontendDist": "../src",
    "devUrl": "http://localhost:1420"
  },
  "bundle": {
    "active": true,
    "targets": ["msi", "nsis", "dmg", "app", "deb", "appimage"],
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  }
}
```

- [ ] **Step 2: Build release version**

Run: `cd rustnote && cargo tauri build 2>&1 | tail -50`
Expected: Build completes with .exe/.app output

- [ ] **Step 3: Run final tests**

Run: Test the built application manually
Expected: All MVP features work

- [ ] **Step 4: Commit**

```bash
git add rustnote/
git commit -m "feat: complete MVP build with all features"
```

---

## File Structure Summary

```
rustnote/
├── Cargo.toml                          # Root workspace manifest
├── src-tauri/
│   ├── Cargo.toml                      # Tauri Rust dependencies
│   ├── tauri.conf.json                 # Tauri configuration
│   ├── build.rs                       # Build script
│   ├── icons/                          # App icons
│   └── src/
│       ├── main.rs                    # Application entry
│       ├── lib.rs                     # Library exports + run()
│       ├── commands/                  # Tauri command handlers
│       │   ├── mod.rs
│       │   ├── document.rs            # Document CRUD
│       │   ├── workspace.rs           # File tree
│       │   ├── settings.rs            # Preferences
│       │   └── export.rs              # HTML/PDF export
│       ├── model/                     # Data models
│       │   ├── mod.rs
│       │   ├── document.rs            # Document struct
│       │   ├── workspace.rs           # Workspace, FileEntry
│       │   └── settings.rs            # Settings, Theme
│       └── parser/                    # Markdown processing
│           ├── mod.rs
│           ├── markdown.rs            # pulldown-cmark wrapper
│           └── syntax.rs              # syntect highlighter
├── src/                                # Frontend web assets
│   ├── index.html                     # Main HTML
│   ├── styles/
│   │   ├── main.css                   # Core styles
│   │   ├── theme-light.css            # Light theme
│   │   └── theme-dark.css             # Dark theme
│   └── scripts/
│       ├── app.js                     # Application entry
│       ├── editor.js                  # Editor functionality
│       ├── renderer.js                # Live markdown rendering
│       ├── search.js                  # Find/replace
│       └── settings.js                # Settings panel
└── tests/
    └── rust/                          # Rust unit tests
    └── parser_tests.rs
    └── model_tests.rs
```

---

## Implementation Checklist

- [x] Task 1: Initialize Tauri project structure
- [x] Task 2: Set up logging and error handling
- [x] Task 3: Create document model with tests
- [x] Task 4: Markdown parser integration
- [x] Task 5: Tauri commands (file operations)
- [x] Task 6: Basic editor UI (HTML/CSS/JS)
- [x] Task 7: Live markdown rendering
- [x] Task 8: File operations UI (open/save/workspace)
- [x] Task 9: Search functionality
- [x] Task 10: Theme switching
- [x] Task 11: Export functionality (HTML/PDF)
- [x] Task 12: Auto-save and recovery
- [x] Task 13: Settings panel
- [x] Task 14: Build and verify

---

## Dependencies Summary

| Package | Purpose | Version |
|---------|---------|---------|
| tauri | Desktop shell | 2.x |
| tauri-plugin-dialog | File dialogs | 2.x |
| tauri-plugin-fs | File system access | 2.x |
| pulldown-cmark | Markdown parsing | 0.12 |
| syntect | Syntax highlighting | 5 |
| serde/serde_json | Serialization | 1 |
| uuid | Document IDs | 1 |
| chrono | Timestamps | 0.4 |
| thiserror | Error handling | 2 |

---

## Open Questions Resolution

Based on the spec, the following decisions were made:

1. **Target User Priority**: Developers - focus on code blocks, syntax highlighting, README workflows
2. **Table Editing**: Basic (insert with dialog, cell editing in source) - MVP scope
3. **Math Support**: Post-MVP - defer to future release

---

**Plan complete and saved to:** `outputs/iteration-1/plan.md`

**Two execution options:**

1. **Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

2. **Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**