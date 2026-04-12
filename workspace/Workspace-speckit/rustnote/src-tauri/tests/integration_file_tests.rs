use rustnote_lib::model::document::Document;
use rustnote_lib::model::export::{PdfExportOptions, PdfMargins, PdfPageSize};
use rustnote_lib::model::recovery::RecoverySnapshot;
use rustnote_lib::model::workspace::{FileEntry, Workspace};

#[test]
fn test_file_entry_new_file() {
    let entry = FileEntry::new_file("test.md".to_string(), "/path/test.md".to_string());
    assert_eq!(entry.name, "test.md");
    assert_eq!(entry.path, "/path/test.md");
    assert!(!entry.is_directory);
    assert!(entry.children.is_empty());
}

#[test]
fn test_file_entry_new_directory() {
    let children = vec![
        FileEntry::new_file("a.md".to_string(), "/path/a.md".to_string()),
        FileEntry::new_file("b.md".to_string(), "/path/b.md".to_string()),
    ];
    let entry = FileEntry::new_directory("docs".to_string(), "/path/docs".to_string(), children);
    assert_eq!(entry.name, "docs");
    assert!(entry.is_directory);
    assert_eq!(entry.children.len(), 2);
}

#[test]
fn test_workspace_new() {
    let ws = Workspace::new("/Users/test/Documents".to_string());
    assert_eq!(ws.name, "Documents");
    assert_eq!(ws.root_path, "/Users/test/Documents");
    assert!(ws.files.is_empty());
}

#[test]
fn test_workspace_with_files() {
    let mut ws = Workspace::new("/Users/test/project".to_string());
    ws.files.push(FileEntry::new_file(
        "readme.md".to_string(),
        "/Users/test/project/readme.md".to_string(),
    ));
    ws.files.push(FileEntry::new_directory(
        "src".to_string(),
        "/Users/test/project/src".to_string(),
        vec![FileEntry::new_file(
            "index.md".to_string(),
            "/Users/test/project/src/index.md".to_string(),
        )],
    ));
    assert_eq!(ws.files.len(), 2);
    assert_eq!(ws.files[0].name, "readme.md");
    assert!(ws.files[1].is_directory);
}

#[test]
fn test_workspace_id_unique() {
    let ws1 = Workspace::new("/path/a".to_string());
    let ws2 = Workspace::new("/path/b".to_string());
    assert_ne!(ws1.id, ws2.id);
}

#[test]
fn test_document_save_and_load_cycle() {
    let mut doc = Document::new("Test".to_string());
    doc.update_content("# Hello\n\nWorld".to_string());
    doc.mark_saved();

    let content = doc.content.clone();
    let is_dirty = doc.is_dirty;
    let headings_count = doc.headings.len();

    assert_eq!(content, "# Hello\n\nWorld");
    assert!(!is_dirty);
    assert_eq!(headings_count, 1);
}

#[test]
fn test_document_with_frontmatter_save() {
    let mut doc = Document::new("Frontmatter Test".to_string());
    doc.update_content("---\ntitle: Test\n---\n\n# Content".to_string());
    assert!(doc.headings.len() >= 1);
}

#[test]
fn test_multiple_documents_independent() {
    let mut doc1 = Document::new("Doc1".to_string());
    let mut doc2 = Document::new("Doc2".to_string());

    doc1.update_content("# Doc1 Content".to_string());
    doc2.update_content("# Doc2 Content\n\nMore content".to_string());

    assert_ne!(doc1.id, doc2.id);
    assert_ne!(doc1.content, doc2.content);
    assert_eq!(doc1.headings.len(), 1);
    assert_eq!(doc2.headings.len(), 1);
}

#[test]
fn test_pdf_export_options_default() {
    let options = PdfExportOptions::default();
    assert_eq!(options.page_size.dimensions(), (210.0, 297.0));
    assert!(options.embed_images);
    assert_eq!(options.margins.top_mm, 20.0);
}

#[test]
fn test_pdf_page_size_dimensions() {
    assert_eq!(PdfPageSize::A4.dimensions(), (210.0, 297.0));
    assert_eq!(PdfPageSize::Letter.dimensions(), (215.9, 279.4));
    assert_eq!(PdfPageSize::Legal.dimensions(), (215.9, 355.6));
    assert_eq!(
        PdfPageSize::Custom {
            width_mm: 100.0,
            height_mm: 200.0
        }
        .dimensions(),
        (100.0, 200.0)
    );
}

#[test]
fn test_pdf_margins_default() {
    let margins = PdfMargins::default();
    assert_eq!(margins.as_tuple(), (20.0, 20.0, 20.0, 20.0));
}

#[test]
fn test_pdf_margins_custom() {
    let margins = PdfMargins {
        top_mm: 15.0,
        right_mm: 25.0,
        bottom_mm: 15.0,
        left_mm: 25.0,
    };
    assert_eq!(margins.as_tuple(), (15.0, 25.0, 15.0, 25.0));
}

#[test]
fn test_pdf_export_options_custom() {
    let options = PdfExportOptions {
        page_size: PdfPageSize::Custom {
            width_mm: 150.0,
            height_mm: 250.0,
        },
        margins: PdfMargins {
            top_mm: 10.0,
            right_mm: 10.0,
            bottom_mm: 10.0,
            left_mm: 10.0,
        },
        embed_images: false,
    };
    assert!(!options.embed_images);
    let (w, h) = options.page_size.dimensions();
    assert_eq!(w, 150.0);
    assert_eq!(h, 250.0);
}

#[test]
fn test_recovery_snapshot_integration() {
    let mut doc = Document::new("Recovery Test".to_string());
    doc.update_content("# Long Content\n\nWith multiple paragraphs".to_string());

    let snapshot = RecoverySnapshot::new(
        doc.file_path.clone(),
        doc.content.clone(),
        10,
        doc.title.clone(),
    );

    let recovered = snapshot.restore();
    assert_eq!(recovered.content, doc.content);
    assert_eq!(recovered.cursor_offset, 10);
    assert_eq!(recovered.title, doc.title);
}

#[test]
fn test_workspace_serialization() {
    let ws = Workspace::new("/test/path".to_string());
    let json = serde_json::to_string(&ws).unwrap();
    let deserialized: Workspace = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, ws.name);
    assert_eq!(deserialized.root_path, ws.root_path);
}

#[test]
fn test_document_serialization() {
    let mut doc = Document::new("Serialize Test".to_string());
    doc.update_content("# Title".to_string());

    let json = serde_json::to_string(&doc).unwrap();
    let deserialized: Document = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.title, doc.title);
    assert_eq!(deserialized.content, doc.content);
}

#[test]
fn test_file_entry_serialization() {
    let entry = FileEntry::new_file("test.md".to_string(), "/path/test.md".to_string());
    let json = serde_json::to_string(&entry).unwrap();
    let deserialized: FileEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, entry.name);
    assert_eq!(deserialized.path, entry.path);
}

#[test]
fn test_pdf_export_serialization() {
    let options = PdfExportOptions::default();
    let json = serde_json::to_string(&options).unwrap();
    let deserialized: PdfExportOptions = serde_json::from_str(&json).unwrap();
    assert!(deserialized.embed_images);
}

#[test]
fn test_workspace_nested_tree() {
    let mut ws = Workspace::new("/project".to_string());
    ws.files.push(FileEntry::new_directory(
        "docs".to_string(),
        "/project/docs".to_string(),
        vec![
            FileEntry::new_directory(
                "api".to_string(),
                "/project/docs/api".to_string(),
                vec![FileEntry::new_file(
                    "ref.md".to_string(),
                    "/project/docs/api/ref.md".to_string(),
                )],
            ),
            FileEntry::new_file("index.md".to_string(), "/project/docs/index.md".to_string()),
        ],
    ));

    let docs = &ws.files[0];
    assert!(docs.is_directory);
    assert_eq!(docs.children.len(), 2);

    let api = &docs.children[0];
    assert!(api.is_directory);
    assert_eq!(api.children.len(), 1);
}

#[test]
fn test_document_title_from_path() {
    let doc = Document::from_file("/path/to/My Document.md", "# Content".to_string()).unwrap();
    assert_eq!(doc.title, "My Document.md");
}

#[test]
fn test_document_dirty_after_edit() {
    let mut doc = Document::new("Test".to_string());
    assert!(doc.is_dirty);

    doc.mark_saved();
    assert!(!doc.is_dirty);

    doc.update_content("edit".to_string());
    assert!(doc.is_dirty);
}

#[test]
fn test_document_headings_update_on_content_change() {
    let mut doc = Document::new("Test".to_string());
    doc.update_content("# H1\n\n## H2".to_string());
    assert_eq!(doc.headings.len(), 2);

    doc.update_content("# New H1\n\n## H2\n\n### H3".to_string());
    assert_eq!(doc.headings.len(), 3);
    assert_eq!(doc.headings[0].text, "New H1");
}

#[test]
fn test_recovery_data_with_file_path() {
    let snapshot = RecoverySnapshot::new(
        Some("/path/to/file.md".to_string()),
        "# Content".to_string(),
        5,
        "File".to_string(),
    );

    let data = snapshot.restore();
    assert!(data.file_path.is_some());
    assert_eq!(data.file_path.unwrap(), "/path/to/file.md");
}

#[test]
fn test_empty_workspace() {
    let ws = Workspace::new("/empty".to_string());
    assert!(ws.files.is_empty());
    assert_eq!(ws.name, "empty");
}

#[test]
fn test_heading_extraction_accuracy() {
    let content = "# Main Title\n\n## Section 1\n\nContent\n\n### Subsection 1.1\n\n## Section 2";
    let headings = Document::extract_headings_from_content(content);
    assert_eq!(headings.len(), 4);
    assert_eq!(headings[0].text, "Main Title");
    assert_eq!(headings[1].text, "Section 1");
    assert_eq!(headings[2].text, "Subsection 1.1");
    assert_eq!(headings[3].text, "Section 2");
}
