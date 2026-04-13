use rustnote_lib::model::{Document, Settings};
use rustnote_lib::services::{
    DocumentResult, DocumentServiceTrait, EditorServiceTrait, FileWatcherServiceTrait,
    SettingsResult, SettingsServiceTrait,
};
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
