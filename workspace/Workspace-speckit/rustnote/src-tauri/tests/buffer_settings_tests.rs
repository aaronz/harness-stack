use rustnote_lib::model::document::Document;
use rustnote_lib::model::recovery::{RecoveryData, RecoverySnapshot};
use rustnote_lib::model::settings::{Settings, Theme};
use std::path::PathBuf;

#[test]
fn test_document_new() {
    let doc = Document::new("Test".to_string());
    assert_eq!(doc.title, "Test");
    assert!(doc.content.is_empty());
    assert!(doc.is_dirty);
    assert!(doc.file_path.is_none());
}

#[test]
fn test_document_update_content() {
    let mut doc = Document::new("Test".to_string());
    doc.update_content("# Hello World".to_string());
    assert_eq!(doc.content, "# Hello World");
    assert!(doc.is_dirty);
    assert_eq!(doc.headings.len(), 1);
    assert_eq!(doc.headings[0].level, 1);
    assert_eq!(doc.headings[0].text, "Hello World");
}

#[test]
fn test_document_mark_saved() {
    let mut doc = Document::new("Test".to_string());
    doc.update_content("content".to_string());
    assert!(doc.is_dirty);
    doc.mark_saved();
    assert!(!doc.is_dirty);
    assert!(doc.last_saved.is_some());
}

#[test]
fn test_document_extract_headings() {
    let content = "# Title\n\n## Section 1\n\n### Subsection\n\n## Section 2";
    let headings = Document::extract_headings_from_content(content);
    assert_eq!(headings.len(), 4);
    assert_eq!(headings[0].level, 1);
    assert_eq!(headings[0].text, "Title");
    assert_eq!(headings[1].level, 2);
    assert_eq!(headings[1].text, "Section 1");
}

#[test]
fn test_document_headings_with_no_headings() {
    let content = "Just a paragraph with no headings";
    let headings = Document::extract_headings_from_content(content);
    assert!(headings.is_empty());
}

#[test]
fn test_document_from_file() {
    let content = "# File Title\n\nSome content".to_string();
    let mut doc = Document::from_file("test.md", content).unwrap();
    assert_eq!(doc.title, "test.md");
    assert_eq!(doc.content, "# File Title\n\nSome content");
    assert!(!doc.is_dirty);
    assert!(doc.file_path.is_some());
}

#[test]
fn test_document_is_dirty_flag() {
    let mut doc = Document::new("Test".to_string());
    assert!(doc.is_dirty);
    doc.mark_saved();
    assert!(!doc.is_dirty);
    doc.update_content("new".to_string());
    assert!(doc.is_dirty);
}

#[test]
fn test_settings_default() {
    let settings = Settings::default();
    assert_eq!(settings.theme, Theme::Light);
    assert!(settings.auto_save);
    assert_eq!(settings.auto_save_interval, 30000);
    assert!(!settings.focus_mode);
    assert!(!settings.typewriter_mode);
    assert!(!settings.outline_visible);
    assert_eq!(settings.font_size, 16);
    assert_eq!(settings.tab_size, 4);
}

#[test]
fn test_settings_theme_toggle() {
    let mut settings = Settings::default();
    assert_eq!(settings.theme, Theme::Light);
    settings.theme = Theme::Dark;
    assert_eq!(settings.theme, Theme::Dark);
}

#[test]
fn test_editor_settings_default() {
    let settings = Settings::default();
    assert_eq!(settings.font_family, "System");
    assert_eq!(settings.font_size, 16);
    assert_eq!(settings.line_height, 1.6);
    assert_eq!(settings.tab_size, 4);
}

#[test]
fn test_editor_settings_custom() {
    let settings = Settings {
        font_family: "Menlo".to_string(),
        font_size: 14,
        line_height: 1.8,
        tab_size: 2,
        content_width: 800,
        ..Settings::default()
    };
    assert_eq!(settings.font_family, "Menlo");
    assert_eq!(settings.font_size, 14);
    assert_eq!(settings.tab_size, 2);
}

#[test]
fn test_recent_files_tracking() {
    let mut settings = Settings::default();
    settings.recent_files.push("file1.md".to_string());
    settings.recent_files.push("file2.md".to_string());
    assert_eq!(settings.recent_files.len(), 2);
    assert_eq!(settings.recent_files[0], "file1.md");
}

#[test]
fn test_recovery_snapshot_new() {
    let snapshot = RecoverySnapshot::new(
        Some("test.md".to_string()),
        "# Content".to_string(),
        10,
        "Test".to_string(),
    );
    assert_eq!(snapshot.content, "# Content");
    assert_eq!(snapshot.cursor_offset, 10);
    assert_eq!(snapshot.title, "Test");
    assert!(snapshot.file_path.is_some());
}

#[test]
fn test_recovery_snapshot_restore() {
    let snapshot = RecoverySnapshot::new(
        Some("test.md".to_string()),
        "# Restored Content".to_string(),
        5,
        "Restored".to_string(),
    );
    let data = snapshot.restore();
    assert_eq!(data.content, "# Restored Content");
    assert_eq!(data.cursor_offset, 5);
    assert_eq!(data.title, "Restored");
}

#[test]
fn test_recovery_snapshot_meta() {
    let snapshot = RecoverySnapshot::new(
        Some("test.md".to_string()),
        "Short content".to_string(),
        0,
        "Test".to_string(),
    );
    let meta = rustnote_lib::model::recovery::RecoverySnapshotMeta::from(&snapshot);
    assert_eq!(meta.title, "Test");
    assert_eq!(meta.content_preview, "Short content");
}

#[test]
fn test_recovery_snapshot_long_content_preview() {
    let long_content = "a".repeat(200);
    let snapshot = RecoverySnapshot::new(None, long_content.clone(), 0, "Long".to_string());
    let meta = rustnote_lib::model::recovery::RecoverySnapshotMeta::from(&snapshot);
    assert!(meta.content_preview.ends_with("..."));
    assert_eq!(meta.content_preview.len(), 103);
}

#[test]
fn test_document_large_heading_count() {
    let mut content = String::new();
    for i in 0..100 {
        content.push_str(&format!("# Heading {}\n\nParagraph {}\n\n", i, i));
    }
    let headings = Document::extract_headings_from_content(&content);
    assert_eq!(headings.len(), 100);
}

#[test]
fn test_document_unicode_content() {
    let content = "# 标题\n\n内容 with émojis: 🎉 👍";
    let headings = Document::extract_headings_from_content(content);
    assert_eq!(headings.len(), 1);
    assert_eq!(headings[0].text, "标题");
}

#[test]
fn test_document_unicode_in_headings() {
    let content = "# Japanese: こんにちは\n# Korean: 안녕하세요\n# Chinese: 你好";
    let headings = Document::extract_headings_from_content(content);
    assert_eq!(headings.len(), 3);
}

#[test]
fn test_settings_auto_save_toggle() {
    let mut settings = Settings::default();
    assert!(settings.auto_save);
    settings.auto_save = false;
    assert!(!settings.auto_save);
}

#[test]
fn test_settings_auto_save_interval() {
    let mut settings = Settings::default();
    settings.auto_save_interval = 60000;
    assert_eq!(settings.auto_save_interval, 60000);
}

#[test]
fn test_settings_focus_mode() {
    let mut settings = Settings::default();
    assert!(!settings.focus_mode);
    settings.focus_mode = true;
    assert!(settings.focus_mode);
}

#[test]
fn test_settings_typewriter_mode() {
    let mut settings = Settings::default();
    assert!(!settings.typewriter_mode);
    settings.typewriter_mode = true;
    assert!(settings.typewriter_mode);
}

#[test]
fn test_settings_outline_visible() {
    let mut settings = Settings::default();
    assert!(!settings.outline_visible);
    settings.outline_visible = true;
    assert!(settings.outline_visible);
}

#[test]
fn test_document_multiple_updates() {
    let mut doc = Document::new("Test".to_string());
    doc.update_content("# First".to_string());
    let first_headings = doc.headings.len();
    doc.update_content("# First\n\n## Second".to_string());
    assert_eq!(doc.headings.len(), first_headings + 1);
    doc.mark_saved();
    assert!(!doc.is_dirty);
}

#[test]
fn test_recovery_data_fields() {
    let data = RecoveryData {
        content: "Test content".to_string(),
        cursor_offset: 5,
        file_path: Some("test.md".to_string()),
        title: "Test".to_string(),
    };
    assert_eq!(data.content, "Test content");
    assert_eq!(data.cursor_offset, 5);
    assert_eq!(data.title, "Test");
}

#[test]
fn test_document_id_is_unique() {
    let doc1 = Document::new("Test1".to_string());
    let doc2 = Document::new("Test2".to_string());
    assert_ne!(doc1.id, doc2.id);
}

#[test]
fn test_document_timestamps() {
    let doc = Document::new("Test".to_string());
    assert!(doc.created_at <= doc.updated_at);
}

#[test]
fn test_snapshot_id_is_unique() {
    let snap1 = RecoverySnapshot::new(None, "a".to_string(), 0, "A".to_string());
    let snap2 = RecoverySnapshot::new(None, "b".to_string(), 0, "B".to_string());
    assert_ne!(snap1.id, snap2.id);
}

#[test]
fn test_document_nested_headings() {
    let content = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
    let headings = Document::extract_headings_from_content(content);
    assert_eq!(headings.len(), 6);
    assert_eq!(headings[0].level, 1);
    assert_eq!(headings[5].level, 6);
}

#[test]
fn test_settings_serialize() {
    let settings = Settings::default();
    let json = serde_json::to_string(&settings).unwrap();
    assert!(json.contains("\"theme\":\"Light\""));
    assert!(json.contains("\"autoSave\":true"));
}

#[test]
fn test_settings_deserialize() {
    let json = r#"{"theme":"Dark","autoSave":false,"autoSaveInterval":60000,"focusMode":true,"typewriterMode":false,"outlineVisible":true,"fontFamily":"Menlo","fontSize":14,"lineHeight":1.5,"tabSize":2,"contentWidth":800,"recentFiles":["a.md","b.md"]}"#;
    let settings: Settings = serde_json::from_str(json).unwrap();
    assert_eq!(settings.theme, Theme::Dark);
    assert!(!settings.auto_save);
    assert_eq!(settings.auto_save_interval, 60000);
    assert!(settings.focus_mode);
    assert_eq!(settings.font_family, "Menlo");
    assert_eq!(settings.font_size, 14);
    assert_eq!(settings.content_width, 800);
}
