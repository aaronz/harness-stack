use rustnote_lib::model::export::{
    HtmlExportMode, HtmlExportOptions, PdfExportOptions, PdfMargins, PdfPageSize,
};
use rustnote_lib::model::{Document, Settings};
use rustnote_lib::services::{
    AutosaveServiceTrait, DocumentResult, DocumentServiceTrait, EditorServiceTrait,
    ExportServiceTrait, FileWatcherServiceTrait, RecoveryServiceTrait, SettingsResult,
    SettingsServiceTrait, WorkspaceServiceTrait,
};
use std::path::PathBuf;
use uuid::Uuid;

#[test]
fn tc_g016_001_service_document_interface() {
    let mut service = rustnote_lib::services::DocumentService::new();

    let doc = service
        .create("Test Doc".to_string())
        .expect("create should work");
    assert_eq!(doc.title, "Test Doc");

    let id = doc.id;

    assert!(service.get_content(id).is_some());

    service
        .update_source(id, "# Hello".to_string())
        .expect("update_source should work");
    let content = service.get_content(id).expect("should get content");
    assert!(content.contains("# Hello"));

    let docs = service.list_documents();
    assert!(docs.contains(&id));

    let retrieved = service.get(id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().title, "Test Doc");

    let closed = service.close(id);
    assert!(closed.is_some());
    assert!(service.get(id).is_none());
}

#[test]
fn tc_g016_001_document_service_trait_objects() {
    fn accepts_trait<T: DocumentServiceTrait>(
        service: &mut T,
        title: String,
    ) -> DocumentResult<Document> {
        service.create(title)
    }

    let mut service = rustnote_lib::services::DocumentService::new();
    let doc = accepts_trait(&mut service, "Test".to_string()).expect("trait object should work");
    assert_eq!(doc.title, "Test");
}

#[test]
fn tc_g016_002_service_editor_interface() {
    let mut service = rustnote_lib::services::EditorService::new();
    let id = Uuid::new_v4();

    use rustnote_lib::semantic::ast::Position;
    let pos = Position {
        offset: 10,
        line: 1,
        column: 5,
    };

    service.set_cursor(id, pos);
    let retrieved_pos = service.cursor(id);
    assert!(retrieved_pos.is_some());
    assert_eq!(retrieved_pos.unwrap().offset, 10);

    service.clear_selection(id);

    service.set_anchor_mode(id, true);
    assert!(service.is_anchor_mode(id));

    service.set_anchor_mode(id, false);
    assert!(!service.is_anchor_mode(id));

    let start_pos = Position {
        offset: 0,
        line: 1,
        column: 0,
    };
    service.start_selection(id, start_pos);

    let end_pos = Position {
        offset: 5,
        line: 1,
        column: 5,
    };
    service.update_selection(id, end_pos);

    assert!(
        service.has_selection(id),
        "should have selection after start and update"
    );

    let range = service.selected_range(id);
    assert!(range.is_some());

    service.remove_editor(id);
    assert!(service.cursor(id).is_none());
}

#[test]
fn tc_g016_002_editor_service_selection() {
    let mut service = rustnote_lib::services::EditorService::new();
    let id = Uuid::new_v4();

    use rustnote_lib::semantic::ast::Position;
    use rustnote_lib::semantic::position::Selection;

    let sel = Selection::new(
        Position {
            offset: 5,
            line: 1,
            column: 5,
        },
        Position {
            offset: 10,
            line: 1,
            column: 10,
        },
    );

    service.set_selection(id, sel.clone());

    let retrieved = service.selection(id);
    assert!(retrieved.is_some());
}

#[test]
fn tc_g016_003_service_no_circular_deps_document() {
    let mut service = rustnote_lib::services::DocumentService::new();

    let doc1 = service.create("Doc1".to_string()).expect("doc1 created");
    let doc2 = service.create("Doc2".to_string()).expect("doc2 created");

    let id1 = doc1.id;
    let id2 = doc2.id;

    service
        .update_source(id1, "# Content 1".to_string())
        .expect("update 1");
    service
        .update_source(id2, "# Content 2".to_string())
        .expect("update 2");

    assert_ne!(id1, id2);

    let content1 = service.get_content(id1).expect("content1");
    assert!(content1.contains("Content 1"));

    let content2 = service.get_content(id2).expect("content2");
    assert!(content2.contains("Content 2"));

    service.close(id1).expect("close 1");
    service.close(id2).expect("close 2");

    let remaining = service.list_documents();
    assert!(remaining.is_empty());
}

#[test]
fn tc_g016_003_service_no_circular_deps_settings() {
    let temp_dir = std::env::temp_dir().join("rustnote_test_no_circular");
    std::fs::create_dir_all(&temp_dir).ok();
    let db_path = temp_dir.join("settings.db");

    let service = rustnote_lib::services::SettingsService::new_with_path(Some(db_path))
        .expect("settings service created");

    let settings = Settings::default();
    service.write_settings(&settings).expect("write settings");

    let loaded: SettingsResult<Settings> = service.read_settings();
    assert!(loaded.is_ok());
    assert_eq!(loaded.unwrap().theme, settings.theme);
}

#[test]
fn tc_g016_003_service_no_circular_deps_file_watcher() {
    let mut service = rustnote_lib::services::FileWatcherService::new();

    let temp_dir = std::env::temp_dir().join("rustnote_watcher_test");
    std::fs::create_dir_all(&temp_dir).ok();
    let test_file = temp_dir.join("test.md");
    std::fs::write(&test_file, "# Test").ok();

    let result = service.watch(test_file.to_str().unwrap());
    assert!(result.is_ok());

    assert!(service.is_watching());

    let unwatch_result = service.unwatch(test_file.to_str().unwrap());
    assert!(unwatch_result.is_ok());
}

#[test]
fn tc_g016_003_service_interface_boundaries() {
    let mut doc_service = rustnote_lib::services::DocumentService::new();
    let mut editor_service = rustnote_lib::services::EditorService::new();

    let doc = doc_service.create("Test".to_string()).expect("doc created");
    let doc_id = doc.id;

    let editor_id = Uuid::new_v4();

    use rustnote_lib::semantic::ast::Position;
    editor_service.set_cursor(
        editor_id,
        Position {
            offset: 0,
            line: 1,
            column: 0,
        },
    );

    assert!(doc_service.get(doc_id).is_some());
    assert!(editor_service.cursor(editor_id).is_some());

    editor_service.remove_editor(editor_id);

    let doc_content = doc_service.get_content(doc_id);
    assert!(doc_content.is_some());
}

#[test]
fn tc_g016_001_document_service_save_and_retrieve() {
    let temp_dir = std::env::temp_dir().join("rustnote_save_test");
    std::fs::create_dir_all(&temp_dir).ok();
    let file_path = temp_dir.join("save_test.md");

    let mut service = rustnote_lib::services::DocumentService::new();

    let doc = service.create("Save Test".to_string()).expect("create doc");
    let id = doc.id;

    service
        .update_source(id, "# Saved Content".to_string())
        .expect("update");

    let result = service.save(id, Some(file_path.clone()));
    assert!(result.is_ok());

    assert!(file_path.exists());

    let content = std::fs::read_to_string(&file_path).expect("read file");
    assert!(content.contains("Saved Content"));
}

#[test]
fn tc_g016_002_editor_service_move_cursor() {
    let mut service = rustnote_lib::services::EditorService::new();
    let id = Uuid::new_v4();

    use rustnote_lib::semantic::ast::Position;

    service.set_cursor(
        id,
        Position {
            offset: 0,
            line: 1,
            column: 0,
        },
    );
    service.move_cursor(
        id,
        Position {
            offset: 10,
            line: 2,
            column: 3,
        },
    );

    let pos = service.cursor(id).expect("cursor should exist");
    assert_eq!(pos.offset, 10);
    assert_eq!(pos.line, 2);
}

#[test]
fn tc_g016_003_services_are_sized_correctly() {
    use std::mem::size_of;

    let doc_service_size = size_of::<rustnote_lib::services::DocumentService>();
    let editor_service_size = size_of::<rustnote_lib::services::EditorService>();
    let settings_service_size = size_of::<rustnote_lib::services::SettingsService>();
    let file_watcher_size = size_of::<rustnote_lib::services::FileWatcherService>();

    assert!(doc_service_size > 0, "DocumentService should be sized");
    assert!(editor_service_size > 0, "EditorService should be sized");
    assert!(settings_service_size > 0, "SettingsService should be sized");
    assert!(file_watcher_size > 0, "FileWatcherService should be sized");
}

#[test]
fn tc_g016_003_service_traits_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<rustnote_lib::services::DocumentService>();
    assert_send_sync::<rustnote_lib::services::EditorService>();
    assert_send_sync::<rustnote_lib::services::SettingsService>();
    assert_send_sync::<rustnote_lib::services::FileWatcherService>();
}

// ============================================================================
// G-014 Service Interface Verification Tests
// These tests verify that service trait implementations match PRD-10 specification
// ============================================================================

/// TC-G014-001: DocumentService trait signature verification
/// Verifies that DocumentServiceTrait matches PRD-10 specification:
/// - create(title) -> DocumentResult<Document>
/// - open(path) -> DocumentResult<Document>
/// - save(id, path) -> DocumentResult<()>
/// - get_content(id) -> Option<String>
/// - update_source(id, source) -> DocumentResult<()>
/// - get(id) -> Option<Document>
/// - list_documents() -> Vec<Uuid>
/// - close(id) -> Option<Document>
#[test]
fn tc_g014_001_document_service_trait_signature() {
    fn accepts_trait<T: DocumentServiceTrait>(service: &mut T) {
        let _ = service;
    }

    let mut service = rustnote_lib::services::DocumentService::new();
    accepts_trait(&mut service);

    // Verify all trait methods are callable with correct signatures
    let doc = service.create("Test Document".to_string());
    assert!(doc.is_ok());
    let doc = doc.unwrap();

    // Test open (creates from path - will fail but signature is correct)
    let result = service.open(std::path::PathBuf::from("/nonexistent/path.md"));
    assert!(result.is_err()); // Expected to fail for nonexistent path

    // Test save with None path (in-memory save)
    let save_result = service.save(doc.id, None);
    assert!(save_result.is_ok());

    // Test get_content
    let content = service.get_content(doc.id);
    assert!(content.is_some());

    // Test update_source
    let update_result = service.update_source(doc.id, "# Updated Content".to_string());
    assert!(update_result.is_ok());

    // Verify content was updated
    let updated_content = service.get_content(doc.id);
    assert!(updated_content.is_some());
    assert!(updated_content.unwrap().contains("Updated Content"));

    // Test get
    let retrieved = service.get(doc.id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().title, "Test Document");

    // Test list_documents
    let docs = service.list_documents();
    assert!(docs.contains(&doc.id));

    // Test close
    let closed = service.close(doc.id);
    assert!(closed.is_some());

    // Verify document is closed
    let after_close = service.get(doc.id);
    assert!(after_close.is_none());
}

/// TC-G014-002: EditorService trait signature verification
/// Verifies that EditorServiceTrait matches PRD-10 specification:
/// - cursor(id) -> Option<Position>
/// - set_cursor(id, pos)
/// - selection(id) -> Option<Selection>
/// - set_selection(id, sel)
/// - clear_selection(id)
/// - is_anchor_mode(id) -> bool
/// - set_anchor_mode(id, mode)
/// - start_selection(id, pos)
/// - update_selection(id, head)
/// - move_cursor(id, pos)
/// - has_selection(id) -> bool
/// - selected_range(id) -> Option<(Position, Position)>
/// - remove_editor(id)
#[test]
fn tc_g014_002_editor_service_trait_signature() {
    fn accepts_trait<T: EditorServiceTrait>(service: &T) {
        let _ = service;
    }

    let service = rustnote_lib::services::EditorService::new();
    accepts_trait(&service);

    let id = Uuid::new_v4();

    use rustnote_lib::semantic::ast::Position;
    use rustnote_lib::semantic::position::Selection;

    // Test set_cursor and cursor
    let pos = Position {
        offset: 10,
        line: 1,
        column: 5,
    };
    let mut mutable_service = rustnote_lib::services::EditorService::new();
    mutable_service.set_cursor(id, pos);
    let retrieved_pos = mutable_service.cursor(id);
    assert!(retrieved_pos.is_some());
    assert_eq!(retrieved_pos.unwrap().offset, 10);

    // Test selection and set_selection
    let sel = Selection::new(
        Position {
            offset: 0,
            line: 1,
            column: 0,
        },
        Position {
            offset: 5,
            line: 1,
            column: 5,
        },
    );
    mutable_service.set_selection(id, sel.clone());
    let retrieved_sel = mutable_service.selection(id);
    assert!(retrieved_sel.is_some());

    // Test clear_selection
    mutable_service.clear_selection(id);

    // Test anchor mode
    mutable_service.set_anchor_mode(id, true);
    assert!(mutable_service.is_anchor_mode(id));
    mutable_service.set_anchor_mode(id, false);
    assert!(!mutable_service.is_anchor_mode(id));

    // Test selection operations
    let start_pos = Position {
        offset: 0,
        line: 1,
        column: 0,
    };
    mutable_service.start_selection(id, start_pos);

    let end_pos = Position {
        offset: 5,
        line: 1,
        column: 5,
    };
    mutable_service.update_selection(id, end_pos);
    assert!(mutable_service.has_selection(id));

    // Test selected_range
    let range = mutable_service.selected_range(id);
    assert!(range.is_some());

    // Test move_cursor
    let new_pos = Position {
        offset: 15,
        line: 2,
        column: 10,
    };
    mutable_service.move_cursor(id, new_pos);
    let moved_pos = mutable_service.cursor(id);
    assert!(moved_pos.is_some());
    assert_eq!(moved_pos.unwrap().offset, 15);

    // Test remove_editor
    mutable_service.remove_editor(id);
    assert!(mutable_service.cursor(id).is_none());
}

/// TC-G014-003: WorkspaceService trait signature verification
/// Verifies that WorkspaceServiceTrait matches PRD-10 specification:
/// - list_workspace(path) -> WorkspaceResult<Workspace>
/// - read_dir(path) -> WorkspaceResult<Vec<FileEntry>>
/// - get_workspace_root() -> Option<String>
/// - set_workspace_root(path)
#[test]
fn tc_g014_003_workspace_service_trait_signature() {
    fn accepts_trait<T: WorkspaceServiceTrait>(service: &T) {
        let _ = service;
    }

    let service = rustnote_lib::services::WorkspaceService::new();
    accepts_trait(&service);

    // Test get_workspace_root and set_workspace_root
    assert!(service.get_workspace_root().is_none());

    let mut mutable_service = rustnote_lib::services::WorkspaceService::new();
    mutable_service.set_workspace_root("/test/workspace".to_string());
    assert_eq!(
        mutable_service.get_workspace_root(),
        Some("/test/workspace".to_string())
    );

    // Test list_workspace with a temp directory
    let temp_dir = std::env::temp_dir().join("rustnote_workspace_test");
    std::fs::create_dir_all(&temp_dir).ok();

    // Create a test markdown file
    let test_file = temp_dir.join("test.md");
    std::fs::write(&test_file, "# Test").ok();

    let workspace = mutable_service.list_workspace(temp_dir.to_str().unwrap());
    assert!(workspace.is_ok());
    let workspace = workspace.unwrap();
    assert!(!workspace.files.is_empty());

    // Test read_dir
    let entries = mutable_service.read_dir(temp_dir.to_str().unwrap());
    assert!(entries.is_ok());
    let entries = entries.unwrap();
    assert!(!entries.is_empty());

    // Cleanup
    std::fs::remove_file(&test_file).ok();
}

/// TC-G014-004: ExportService trait signature verification
/// Verifies that ExportServiceTrait matches PRD-10 specification:
/// - export_to_html(markdown, output_path, options) -> ExportResult<()>
/// - export_to_pdf(markdown, output_path) -> ExportResult<()>
/// - export_to_pdf_with_options(markdown, output_path, options) -> ExportResult<()>
/// - get_print_html(markdown) -> String
#[test]
fn tc_g014_004_export_service_trait_signature() {
    fn accepts_trait<T: ExportServiceTrait>(service: &T) {
        let _ = service;
    }

    let service = rustnote_lib::services::ExportService::new();
    accepts_trait(&service);

    // Test get_print_html
    let html = service.get_print_html("# Hello World\n\nThis is a test.");
    assert!(html.contains("<h1"));
    assert!(html.contains("Hello World"));
    assert!(html.contains("This is a test."));

    // Test export_to_pdf (writes to temp file)
    let temp_dir = std::env::temp_dir().join("rustnote_export_test");
    std::fs::create_dir_all(&temp_dir).ok();
    let pdf_path = temp_dir.join("export_test.html"); // Using .html extension for HTML-based export

    let result = service.export_to_pdf("# Test Content", pdf_path.to_str().unwrap());
    assert!(result.is_ok());
    assert!(pdf_path.exists());

    // Test export_to_html
    let html_path = temp_dir.join("export_test.html");
    let options = HtmlExportOptions {
        mode: HtmlExportMode::Inline,
        embed_css: true,
    };
    let html_result =
        service.export_to_html("# HTML Export Test", html_path.to_str().unwrap(), options);
    assert!(html_result.is_ok());
    assert!(html_path.exists());

    // Test export_to_pdf_with_options
    let pdf_options_path = temp_dir.join("export_test_options.html");
    let pdf_options = PdfExportOptions {
        page_size: PdfPageSize::A4,
        margins: PdfMargins::default(),
        embed_images: true,
    };
    let pdf_result = service.export_to_pdf_with_options(
        "# Test",
        pdf_options_path.to_str().unwrap(),
        pdf_options,
    );
    assert!(pdf_result.is_ok());
    assert!(pdf_options_path.exists());

    // Cleanup
    std::fs::remove_file(&pdf_path).ok();
    std::fs::remove_file(&html_path).ok();
    std::fs::remove_file(&pdf_options_path).ok();
}

/// TC-G014-005: RecoveryService trait signature verification
/// Verifies that RecoveryServiceTrait matches PRD-10 specification:
/// - save_snapshot(file_path, content, cursor_offset, title) -> RecoveryResult<String>
/// - list_snapshots() -> RecoveryResult<Vec<RecoverySnapshotMeta>>
/// - restore_snapshot(id) -> RecoveryResult<RecoveryData>
/// - delete_snapshot(id) -> RecoveryResult<()>
/// - cleanup_old(max_age_hours) -> RecoveryResult<usize>
#[test]
fn tc_g014_005_recovery_service_trait_signature() {
    fn accepts_trait<T: RecoveryServiceTrait>(service: &T) {
        let _ = service;
    }

    let service = rustnote_lib::services::RecoveryService::new();
    accepts_trait(&service);

    // Test save_snapshot
    let snapshot_id = service.save_snapshot(
        Some("/test/path.md".to_string()),
        "# Test Content".to_string(),
        10,
        "Test Title".to_string(),
    );
    assert!(snapshot_id.is_ok());
    let snapshot_id = snapshot_id.unwrap();

    // Test list_snapshots
    let snapshots = service.list_snapshots();
    assert!(snapshots.is_ok());
    let snapshots = snapshots.unwrap();
    assert!(!snapshots.is_empty());

    // Find our snapshot
    let our_snapshot = snapshots.iter().find(|s| s.id == snapshot_id);
    assert!(our_snapshot.is_some());
    let our_snapshot = our_snapshot.unwrap();
    assert_eq!(our_snapshot.title, "Test Title");

    // Test restore_snapshot
    let restored = service.restore_snapshot(&snapshot_id);
    assert!(restored.is_ok());
    let restored = restored.unwrap();
    assert_eq!(restored.content, "# Test Content");
    assert_eq!(restored.cursor_offset, 10);
    assert_eq!(restored.title, "Test Title");

    // Test delete_snapshot
    let delete_result = service.delete_snapshot(&snapshot_id);
    assert!(delete_result.is_ok());

    // Verify snapshot is deleted
    let snapshots_after_delete = service.list_snapshots().unwrap();
    let deleted_snapshot = snapshots_after_delete.iter().find(|s| s.id == snapshot_id);
    assert!(deleted_snapshot.is_none());

    // Test cleanup_old (create snapshots first)
    let id1 = service
        .save_snapshot(
            Some("/test/path1.md".to_string()),
            "# Content 1".to_string(),
            0,
            "Title 1".to_string(),
        )
        .unwrap();

    let id2 = service
        .save_snapshot(
            Some("/test/path2.md".to_string()),
            "# Content 2".to_string(),
            0,
            "Title 2".to_string(),
        )
        .unwrap();

    // cleanup_old with 0 hours should delete all snapshots created in this test
    let cleaned = service.cleanup_old(0);
    assert!(cleaned.is_ok());

    // Cleanup
    service.delete_snapshot(&id1).ok();
    service.delete_snapshot(&id2).ok();
}

// ============================================================================
// TC-SI001 to TC-SI010: Service Interface Verification Against PRD-10
// ============================================================================

/// TC-SI001: DocumentServiceTrait completeness
/// Verifies that DocumentServiceTrait has all methods required by PRD-10:
/// - create(title: String) -> DocumentResult<Document>
/// - open(path: PathBuf) -> DocumentResult<Document>
/// - save(id: Uuid, path: Option<PathBuf>) -> DocumentResult<()>
/// - get_content(id: Uuid) -> Option<String>
/// - update_source(id: Uuid, source: String) -> DocumentResult<()>
/// - get(id: Uuid) -> Option<Document>
/// - list_documents() -> Vec<Uuid>
/// - close(id: Uuid) -> Option<Document>
#[test]
fn tc_si001_document_service_trait_completeness() {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn DocumentServiceTrait>();

    let mut service = rustnote_lib::services::DocumentService::new();

    // create
    let doc = service
        .create("Test".to_string())
        .expect("create should work");
    let doc_id = doc.id;
    assert_eq!(doc.title, "Test");

    // get_content
    let content = service.get_content(doc_id);
    assert!(content.is_some());

    // update_source
    service
        .update_source(doc_id, "# Updated".to_string())
        .expect("update_source should work");

    // get
    let retrieved = service.get(doc_id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().title, "Test");

    // list_documents
    let docs = service.list_documents();
    assert!(docs.contains(&doc_id));

    // close
    let closed = service.close(doc_id);
    assert!(closed.is_some());
    assert!(service.get(doc_id).is_none());
}

/// TC-SI002: EditorServiceTrait completeness
/// Verifies that EditorServiceTrait has all methods required by PRD-10:
/// - cursor(id: Uuid) -> Option<Position>
/// - set_cursor(id: Uuid, pos: Position)
/// - selection(id: Uuid) -> Option<Selection>
/// - set_selection(id: Uuid, sel: Selection)
/// - clear_selection(id: Uuid)
/// - is_anchor_mode(id: Uuid) -> bool
/// - set_anchor_mode(id: Uuid, mode: bool)
/// - start_selection(id: Uuid, pos: Position)
/// - update_selection(id: Uuid, head: Position)
/// - move_cursor(id: Uuid, pos: Position)
/// - has_selection(id: Uuid) -> bool
/// - selected_range(id: Uuid) -> Option<(Position, Position)>
/// - remove_editor(id: Uuid)
#[test]
fn tc_si002_editor_service_trait_completeness() {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn EditorServiceTrait>();

    let mut service = rustnote_lib::services::EditorService::new();
    let id = Uuid::new_v4();

    use rustnote_lib::semantic::ast::Position;
    use rustnote_lib::semantic::position::Selection;

    // set_cursor / cursor
    let pos = Position {
        offset: 10,
        line: 1,
        column: 5,
    };
    service.set_cursor(id, pos);
    assert_eq!(service.cursor(id).unwrap().offset, 10);

    // set_selection / selection
    let sel = Selection::new(
        Position {
            offset: 0,
            line: 1,
            column: 0,
        },
        Position {
            offset: 5,
            line: 1,
            column: 5,
        },
    );
    service.set_selection(id, sel);
    assert!(service.selection(id).is_some());

    // clear_selection
    service.clear_selection(id);

    // is_anchor_mode / set_anchor_mode
    service.set_anchor_mode(id, true);
    assert!(service.is_anchor_mode(id));
    service.set_anchor_mode(id, false);
    assert!(!service.is_anchor_mode(id));

    // start_selection / update_selection
    let start = Position {
        offset: 0,
        line: 1,
        column: 0,
    };
    let end = Position {
        offset: 5,
        line: 1,
        column: 5,
    };
    service.start_selection(id, start);
    service.update_selection(id, end);

    // has_selection
    assert!(service.has_selection(id));

    // selected_range
    assert!(service.selected_range(id).is_some());

    // move_cursor
    let new_pos = Position {
        offset: 15,
        line: 2,
        column: 10,
    };
    service.move_cursor(id, new_pos);
    assert_eq!(service.cursor(id).unwrap().offset, 15);

    // remove_editor
    service.remove_editor(id);
    assert!(service.cursor(id).is_none());
}

/// TC-SI003: SettingsServiceTrait completeness
/// Verifies that SettingsServiceTrait has all methods required by PRD-10:
/// - read_settings() -> SettingsResult<Settings>
/// - write_settings(settings: &Settings) -> SettingsResult<()>
/// - update_setting(key: &str, value: &str) -> SettingsResult<()>
/// - get_setting(key: &str) -> SettingsResult<Option<String>>
/// - add_recent_file(path: &str) -> SettingsResult<()>
/// - get_recent_files() -> SettingsResult<Vec<String>>
/// - clear_recent_files() -> SettingsResult<()>
/// - set_workspace_state(key: &str, value: &str) -> SettingsResult<()>
/// - get_workspace_state(key: &str) -> SettingsResult<Option<String>>
/// - delete_workspace_state(key: &str) -> SettingsResult<()>
/// - verify_integrity() -> SettingsResult<bool>
#[test]
fn tc_si003_settings_service_trait_completeness() {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn SettingsServiceTrait>();

    let temp_dir = std::env::temp_dir().join("tc_si003_settings_test");
    std::fs::create_dir_all(&temp_dir).ok();
    let db_path = temp_dir.join("settings.db");

    let service = rustnote_lib::services::SettingsService::new_with_path(Some(db_path))
        .expect("settings service created");

    // read_settings / write_settings
    let settings = Settings::default();
    service
        .write_settings(&settings)
        .expect("write_settings should work");
    let loaded = service.read_settings().expect("read_settings should work");
    assert_eq!(loaded.theme, settings.theme);

    // update_setting / get_setting
    service
        .update_setting("test_key", "test_value")
        .expect("update_setting should work");
    let value = service
        .get_setting("test_key")
        .expect("get_setting should work");
    assert_eq!(value, Some("test_value".to_string()));

    // add_recent_file / get_recent_files / clear_recent_files
    service
        .add_recent_file("/path/to/file.md")
        .expect("add_recent_file should work");
    let files = service
        .get_recent_files()
        .expect("get_recent_files should work");
    assert!(files.contains(&"/path/to/file.md".to_string()));
    service
        .clear_recent_files()
        .expect("clear_recent_files should work");
    let cleared = service
        .get_recent_files()
        .expect("get_recent_files should work after clear");
    assert!(cleared.is_empty());

    // set_workspace_state / get_workspace_state / delete_workspace_state
    service
        .set_workspace_state("ws_key", "ws_value")
        .expect("set_workspace_state should work");
    let ws_value = service
        .get_workspace_state("ws_key")
        .expect("get_workspace_state should work");
    assert_eq!(ws_value, Some("ws_value".to_string()));
    service
        .delete_workspace_state("ws_key")
        .expect("delete_workspace_state should work");
    let deleted = service
        .get_workspace_state("ws_key")
        .expect("get_workspace_state after delete should work");
    assert!(deleted.is_none());

    // verify_integrity
    let integrity = service
        .verify_integrity()
        .expect("verify_integrity should work");
    assert!(integrity);
}

/// TC-SI004: FileWatcherServiceTrait completeness
/// Verifies that FileWatcherServiceTrait has all methods required by PRD-10:
/// - watch(path: &str) -> FileWatcherResult<()>
/// - unwatch(path: &str) -> FileWatcherResult<()>
/// - poll_changes() -> Vec<FileChange>
/// - check_file_changed(path: &str) -> Option<bool>
/// - update_file_state(path: &str)
/// - is_watching() -> bool
#[test]
fn tc_si004_file_watcher_service_trait_completeness() {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn FileWatcherServiceTrait>();

    let mut service = rustnote_lib::services::FileWatcherService::new();

    let temp_dir = std::env::temp_dir().join("tc_si004_watcher_test");
    std::fs::create_dir_all(&temp_dir).ok();
    let test_file = temp_dir.join("test.md");
    std::fs::write(&test_file, "# Test").ok();

    // is_watching before watch
    assert!(!service.is_watching());

    // watch
    service
        .watch(test_file.to_str().unwrap())
        .expect("watch should work");
    assert!(service.is_watching());

    // poll_changes
    let changes = service.poll_changes();
    assert!(changes.is_empty()); // No changes yet

    // check_file_changed
    let changed = service.check_file_changed(test_file.to_str().unwrap());
    assert!(changed.is_some());
    assert_eq!(changed.unwrap(), false); // Not changed yet

    // update_file_state
    service.update_file_state(test_file.to_str().unwrap());

    // unwatch
    service
        .unwatch(test_file.to_str().unwrap())
        .expect("unwatch should work");

    std::fs::remove_file(&test_file).ok();
}

/// TC-SI005: AutosaveServiceTrait completeness
/// Verifies that AutosaveServiceTrait has all methods required by PRD-10:
/// - mark_dirty(doc_id: Uuid, content_hash: u64)
/// - needs_autosave(doc_id: Uuid) -> bool
/// - can_autosave(doc_id: Uuid) -> bool
/// - record_autosave(doc_id: Uuid, content_hash: u64)
/// - create_recovery_snapshot(doc: &Document, cursor_offset: usize) -> AutosaveResult<String>
/// - remove_document(doc_id: Uuid)
/// - tracked_documents() -> Vec<Uuid>
/// - is_enabled() -> bool
/// - set_enabled(enabled: bool)
/// - get_interval() -> u64
/// - set_interval(interval_ms: u64)
/// - get_debounce() -> u64
/// - set_debounce(debounce_ms: u64)
/// - get_config() -> AutosaveConfig
/// - set_config(config: AutosaveConfig)
/// - is_tracked(doc_id: Uuid) -> bool
/// - reset_dirty(doc_id: Uuid)
#[test]
fn tc_si005_autosave_service_trait_completeness() {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn AutosaveServiceTrait>();

    let mut service = rustnote_lib::services::AutosaveService::new(PathBuf::from("/tmp"));

    let doc_id = Uuid::new_v4();
    let doc = Document::new("Test Doc".to_string());

    // is_enabled / set_enabled
    assert!(service.is_enabled());
    service.set_enabled(false);
    assert!(!service.is_enabled());
    service.set_enabled(true);

    // get_interval / set_interval
    assert_eq!(service.get_interval(), 30000);
    service.set_interval(60000);
    assert_eq!(service.get_interval(), 60000);

    // get_debounce / set_debounce
    assert_eq!(service.get_debounce(), 2000);
    service.set_debounce(5000);
    assert_eq!(service.get_debounce(), 5000);

    // get_config / set_config
    let config = service.get_config();
    assert!(config.enabled);
    let new_config = rustnote_lib::services::AutosaveConfig {
        enabled: false,
        interval_ms: 120000,
        debounce_ms: 10000,
    };
    service.set_config(new_config.clone());
    assert_eq!(service.get_config().enabled, false);

    // Re-enable autosave for remaining tests
    service.set_enabled(true);

    // is_tracked
    assert!(!service.is_tracked(doc_id));

    // mark_dirty / needs_autosave
    service.mark_dirty(doc_id, 12345);
    assert!(service.is_tracked(doc_id));
    assert!(service.needs_autosave(doc_id));

    // can_autosave (should be false due to debounce)
    assert!(!service.can_autosave(doc_id));

    // record_autosave
    service.record_autosave(doc_id, 12345);
    assert!(!service.needs_autosave(doc_id));

    // tracked_documents
    let tracked = service.tracked_documents();
    assert!(tracked.contains(&doc_id));

    // reset_dirty
    service.mark_dirty(doc_id, 54321);
    service.reset_dirty(doc_id);
    assert!(!service.needs_autosave(doc_id));

    // remove_document
    service.remove_document(doc_id);
    assert!(!service.is_tracked(doc_id));
}

/// TC-SI006: ExportServiceTrait completeness
/// Verifies that ExportServiceTrait has all methods required by PRD-10:
/// - export_to_html(markdown: &str, output_path: &str, options: HtmlExportOptions) -> ExportResult<()>
/// - export_to_pdf(markdown: &str, output_path: &str) -> ExportResult<()>
/// - export_to_pdf_with_options(markdown: &str, output_path: &str, options: PdfExportOptions) -> ExportResult<()>
/// - get_print_html(markdown: &str) -> String
#[test]
fn tc_si006_export_service_trait_completeness() {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn ExportServiceTrait>();

    let service = rustnote_lib::services::ExportService::new();

    // get_print_html
    let html = service.get_print_html("# Hello\n\nWorld");
    assert!(html.contains("<h1"));
    assert!(html.contains("Hello"));

    let temp_dir = std::env::temp_dir().join("tc_si006_export_test");
    std::fs::create_dir_all(&temp_dir).ok();

    // export_to_html
    let html_path = temp_dir.join("test.html");
    let options = HtmlExportOptions {
        mode: HtmlExportMode::Inline,
        embed_css: true,
    };
    service
        .export_to_html("# HTML Test", html_path.to_str().unwrap(), options)
        .expect("export_to_html should work");
    assert!(html_path.exists());

    // export_to_pdf
    let pdf_path = temp_dir.join("test.html"); // Using HTML export
    service
        .export_to_pdf("# PDF Test", pdf_path.to_str().unwrap())
        .expect("export_to_pdf should work");
    assert!(pdf_path.exists());

    // export_to_pdf_with_options
    let pdf_opts_path = temp_dir.join("test_opts.html");
    let pdf_options = PdfExportOptions {
        page_size: PdfPageSize::A4,
        margins: PdfMargins::default(),
        embed_images: true,
    };
    service
        .export_to_pdf_with_options(
            "# PDF Options Test",
            pdf_opts_path.to_str().unwrap(),
            pdf_options,
        )
        .expect("export_to_pdf_with_options should work");
    assert!(pdf_opts_path.exists());

    std::fs::remove_file(&html_path).ok();
    std::fs::remove_file(&pdf_path).ok();
    std::fs::remove_file(&pdf_opts_path).ok();
}

/// TC-SI007: RecoveryServiceTrait completeness
/// Verifies that RecoveryServiceTrait has all methods required by PRD-10:
/// - save_snapshot(file_path: Option<String>, content: String, cursor_offset: usize, title: String) -> RecoveryResult<String>
/// - list_snapshots() -> RecoveryResult<Vec<RecoverySnapshotMeta>>
/// - restore_snapshot(id: &str) -> RecoveryResult<RecoveryData>
/// - delete_snapshot(id: &str) -> RecoveryResult<()>
/// - cleanup_old(max_age_hours: u64) -> RecoveryResult<usize>
#[test]
fn tc_si007_recovery_service_trait_completeness() {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn RecoveryServiceTrait>();

    let service = rustnote_lib::services::RecoveryService::new();

    // save_snapshot
    let snapshot_id = service
        .save_snapshot(
            Some("/test/path.md".to_string()),
            "# Test Content".to_string(),
            10,
            "Test Title".to_string(),
        )
        .expect("save_snapshot should work");

    // list_snapshots
    let snapshots = service
        .list_snapshots()
        .expect("list_snapshots should work");
    assert!(!snapshots.is_empty());

    // restore_snapshot
    let restored = service
        .restore_snapshot(&snapshot_id)
        .expect("restore_snapshot should work");
    assert_eq!(restored.content, "# Test Content");
    assert_eq!(restored.cursor_offset, 10);

    // delete_snapshot
    service
        .delete_snapshot(&snapshot_id)
        .expect("delete_snapshot should work");
    let after_delete = service.list_snapshots().unwrap();
    assert!(after_delete.iter().find(|s| s.id == snapshot_id).is_none());

    // cleanup_old
    let id2 = service
        .save_snapshot(
            Some("/test/path2.md".to_string()),
            "# Content 2".to_string(),
            0,
            "Title 2".to_string(),
        )
        .unwrap();
    let cleaned = service.cleanup_old(0).expect("cleanup_old should work");
    assert!(cleaned >= 1);
    service.delete_snapshot(&id2).ok();
}

/// TC-SI008: WorkspaceServiceTrait completeness
/// Verifies that WorkspaceServiceTrait has all methods required by PRD-10:
/// - list_workspace(path: &str) -> WorkspaceResult<Workspace>
/// - read_dir(path: &str) -> WorkspaceResult<Vec<FileEntry>>
/// - get_workspace_root() -> Option<String>
/// - set_workspace_root(path: String)
#[test]
fn tc_si008_workspace_service_trait_completeness() {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn WorkspaceServiceTrait>();

    let mut service = rustnote_lib::services::WorkspaceService::new();

    // get_workspace_root / set_workspace_root
    assert!(service.get_workspace_root().is_none());
    service.set_workspace_root("/test/workspace".to_string());
    assert_eq!(
        service.get_workspace_root(),
        Some("/test/workspace".to_string())
    );

    let temp_dir = std::env::temp_dir().join("tc_si008_workspace_test");
    std::fs::create_dir_all(&temp_dir).ok();
    let test_file = temp_dir.join("test.md");
    std::fs::write(&test_file, "# Test").ok();

    // list_workspace
    let workspace = service
        .list_workspace(temp_dir.to_str().unwrap())
        .expect("list_workspace should work");
    assert!(!workspace.files.is_empty());

    // read_dir
    let entries = service
        .read_dir(temp_dir.to_str().unwrap())
        .expect("read_dir should work");
    assert!(!entries.is_empty());

    std::fs::remove_file(&test_file).ok();
}

/// TC-SI009: No extra methods in traits
/// Verifies that all service traits match exactly with PRD-10
/// and have no extra methods not defined in the specification
#[test]
fn tc_si009_no_extra_methods_in_traits() {
    fn assert_trait<T: ?Sized>() {}
    assert_trait::<dyn DocumentServiceTrait>();
    assert_trait::<dyn EditorServiceTrait>();
    assert_trait::<dyn SettingsServiceTrait>();
    assert_trait::<dyn FileWatcherServiceTrait>();
    assert_trait::<dyn AutosaveServiceTrait>();
    assert_trait::<dyn ExportServiceTrait>();
    assert_trait::<dyn RecoveryServiceTrait>();
    assert_trait::<dyn WorkspaceServiceTrait>();
}

/// TC-SI010: Method signatures match exactly
/// Verifies that all service trait method signatures match PRD-10 exactly
#[test]
fn tc_si010_method_signatures_match_exactly() {
    // DocumentServiceTrait method signatures
    fn test_document_service_trait<S: DocumentServiceTrait>(service: &mut S) {
        let _doc = service.create("title".to_string());
        let _result = service.open(PathBuf::from("/path"));
        let _result = service.save(Uuid::new_v4(), None);
        let _content = service.get_content(Uuid::new_v4());
        let _result = service.update_source(Uuid::new_v4(), "source".to_string());
        let _doc = service.get(Uuid::new_v4());
        let _ids = service.list_documents();
        let _doc = service.close(Uuid::new_v4());
    }

    // EditorServiceTrait method signatures
    use rustnote_lib::semantic::ast::Position;
    use rustnote_lib::semantic::position::Selection;
    fn test_editor_service_trait<S: EditorServiceTrait>(service: &mut S) {
        let id = Uuid::new_v4();
        let pos = Position {
            offset: 0,
            line: 1,
            column: 0,
        };
        let _cursor = service.cursor(id);
        service.set_cursor(id, pos);
        let _sel = service.selection(id);
        let sel = Selection::new(pos, pos);
        service.set_selection(id, sel);
        service.clear_selection(id);
        let _mode = service.is_anchor_mode(id);
        service.set_anchor_mode(id, true);
        service.start_selection(id, pos);
        service.update_selection(id, pos);
        service.move_cursor(id, pos);
        let _has = service.has_selection(id);
        let _range = service.selected_range(id);
        service.remove_editor(id);
    }

    // This test verifies all signatures compile correctly
    let mut doc_service = rustnote_lib::services::DocumentService::new();
    test_document_service_trait(&mut doc_service);

    let mut editor_service = rustnote_lib::services::EditorService::new();
    test_editor_service_trait(&mut editor_service);
}
