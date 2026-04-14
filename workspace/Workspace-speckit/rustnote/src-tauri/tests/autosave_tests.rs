use rustnote_lib::model::recovery::RecoverySnapshot;
use rustnote_lib::services::autosave::{compute_content_hash, AutosaveConfig, AutosaveService};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use uuid::Uuid;

fn create_temp_app_data_dir() -> PathBuf {
    let temp_dir = TempDir::new().unwrap();
    temp_dir.into_path()
}

#[test]
fn test_tc_g017_001_autosave_interval() {
    let app_data_dir = create_temp_app_data_dir();
    let mut service = AutosaveService::new(app_data_dir.clone());

    service.set_config(AutosaveConfig {
        enabled: true,
        interval_ms: 30000,
        debounce_ms: 2000,
    });

    let doc_id = Uuid::new_v4();
    let content = "# Test Document\n\nSome content here";
    let content_hash = compute_content_hash(content);

    service.mark_dirty(doc_id, content_hash);

    assert!(
        service.needs_autosave(doc_id),
        "Should need autosave immediately after dirty mark"
    );

    std::thread::sleep(Duration::from_millis(2500));

    assert!(
        service.can_autosave(doc_id),
        "Should be able to autosave after debounce"
    );

    let snapshot_id = {
        let doc =
            rustnote_lib::model::document::Document::from_file("test.md", content.to_string())
                .unwrap();

        service.create_recovery_snapshot(&doc, 10).unwrap()
    };

    service.record_autosave(doc_id, content_hash);

    assert!(
        !service.can_autosave(doc_id),
        "Should not trigger another autosave immediately"
    );

    let snapshots = RecoverySnapshot::list(&app_data_dir).unwrap();
    assert!(
        !snapshots.is_empty(),
        "Should have saved at least one snapshot"
    );

    let latest = snapshots.first().unwrap();
    assert_eq!(latest.id, snapshot_id);
    assert_eq!(latest.content, content);
}

#[test]
fn test_tc_g017_002_autosave_crash_recovery() {
    let app_data_dir = create_temp_app_data_dir();
    let service = AutosaveService::new(app_data_dir.clone());

    let content = "# Crash Recovery Test\n\nThis content should be recovered";
    let title = "Crash Test";
    let cursor_offset = 25;
    let file_path = Some("/test/path/crash_test.md".to_string());

    let snapshot = RecoverySnapshot::new(
        file_path.clone(),
        content.to_string(),
        cursor_offset,
        title.to_string(),
    );

    let snapshot_id = snapshot.id.clone();
    snapshot.save(&app_data_dir).unwrap();

    let snapshots = RecoverySnapshot::list(&app_data_dir).unwrap();
    let found = snapshots.iter().find(|s| s.id == snapshot_id);

    assert!(found.is_some(), "Snapshot should exist for crash recovery");

    let recovered = found.unwrap().restore();
    assert_eq!(recovered.content, content);
    assert_eq!(recovered.cursor_offset, cursor_offset);
    assert_eq!(recovered.title, title);
    assert_eq!(recovered.file_path, file_path);
}

#[test]
fn test_tc_g017_003_autosave_debounce() {
    let app_data_dir = create_temp_app_data_dir();
    let mut service = AutosaveService::new(app_data_dir.clone());

    service.set_config(AutosaveConfig {
        enabled: true,
        interval_ms: 30000,
        debounce_ms: 500,
    });

    let doc_id = Uuid::new_v4();

    let content1 = "# Initial Content";
    let hash1 = compute_content_hash(content1);

    service.mark_dirty(doc_id, hash1);

    assert!(
        !service.can_autosave(doc_id),
        "Should not autosave immediately after first edit"
    );

    std::thread::sleep(Duration::from_millis(100));

    let content2 = "# Initial Content Updated";
    let hash2 = compute_content_hash(content2);

    service.mark_dirty(doc_id, hash2);

    assert!(
        !service.can_autosave(doc_id),
        "Should not autosave after rapid second edit"
    );

    std::thread::sleep(Duration::from_millis(600));

    assert!(
        service.can_autosave(doc_id),
        "Should be able to autosave after debounce period with no new edits"
    );
}

#[test]
fn test_edge_case_rapid_edits() {
    let app_data_dir = create_temp_app_data_dir();
    let mut service = AutosaveService::new(app_data_dir.clone());

    service.set_config(AutosaveConfig {
        enabled: true,
        interval_ms: 30000,
        debounce_ms: 100,
    });

    let doc_id = Uuid::new_v4();
    let base_hash = compute_content_hash("# Start");

    for i in 0..10 {
        let content = format!("# Edit {}", i);
        let hash = compute_content_hash(&content);
        service.mark_dirty(doc_id, hash);

        if i < 9 {
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    std::thread::sleep(Duration::from_millis(200));

    assert!(
        service.can_autosave(doc_id),
        "Should eventually autosave after rapid edits stop"
    );

    let final_hash = compute_content_hash("# Edit 9");
    service.record_autosave(doc_id, final_hash);

    assert!(
        !service.needs_autosave(doc_id),
        "Should not need another save immediately"
    );
}

#[test]
fn test_edge_case_crash_during_save() {
    let app_data_dir = create_temp_app_data_dir();
    let service = AutosaveService::new(app_data_dir.clone());

    let content = "# Content Before Crash";
    let title = "Crash During Save";

    let snapshot = RecoverySnapshot::new(
        Some("/test/crash.md".to_string()),
        content.to_string(),
        10,
        title.to_string(),
    );

    let snapshot_id = snapshot.id.clone();

    let result = snapshot.save(&app_data_dir);
    assert!(result.is_ok(), "Snapshot save should succeed");

    let path = result.unwrap();
    assert!(path.exists(), "Snapshot file should exist after save");

    let json = std::fs::read_to_string(&path).unwrap();
    let loaded: RecoverySnapshot = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.id, snapshot_id);
    assert_eq!(loaded.content, content);
}

#[test]
fn test_edge_case_network_files() {
    let app_data_dir = create_temp_app_data_dir();
    let mut service = AutosaveService::new(app_data_dir.clone());

    service.set_config(AutosaveConfig {
        enabled: true,
        interval_ms: 30000,
        debounce_ms: 2000,
    });

    let doc_id = Uuid::new_v4();
    let UNC_path = "//server/share/network_file.md";

    let content = "# Network File Content\n\nStored via network path";
    let hash = compute_content_hash(content);

    service.mark_dirty(doc_id, hash);

    let doc =
        rustnote_lib::model::document::Document::from_file(UNC_path, content.to_string()).unwrap();

    let result = service.create_recovery_snapshot(&doc, 5);

    assert!(result.is_ok(), "Should handle network paths in file_path");
}

#[test]
fn test_autosave_disabled() {
    let app_data_dir = create_temp_app_data_dir();
    let mut service = AutosaveService::new(app_data_dir.clone());

    service.set_enabled(false);

    let doc_id = Uuid::new_v4();
    let hash = compute_content_hash("# Test");

    service.mark_dirty(doc_id, hash);

    assert!(
        !service.can_autosave(doc_id),
        "Should not autosave when disabled"
    );
}

#[test]
fn test_autosave_tracked_documents_management() {
    let app_data_dir = create_temp_app_data_dir();
    let mut service = AutosaveService::new(app_data_dir.clone());

    let doc1 = Uuid::new_v4();
    let doc2 = Uuid::new_v4();

    service.mark_dirty(doc1, compute_content_hash("doc1"));
    service.mark_dirty(doc2, compute_content_hash("doc2"));

    let tracked = service.tracked_documents();
    assert_eq!(tracked.len(), 2);
    assert!(tracked.contains(&doc1));
    assert!(tracked.contains(&doc2));

    service.remove_document(doc1);

    let tracked = service.tracked_documents();
    assert_eq!(tracked.len(), 1);
    assert!(tracked.contains(&doc2));
    assert!(!tracked.contains(&doc1));
}

#[test]
fn test_autosave_content_hash_collision() {
    let hash1 = compute_content_hash("abc");
    let hash2 = compute_content_hash("abc");
    let hash3 = compute_content_hash("xyz");

    assert_eq!(hash1, hash2);
    assert_ne!(hash1, hash3);

    let same_hash_different_content = "def";
    let hash4 = compute_content_hash(same_hash_different_content);
    assert_ne!(hash1, hash4);
}

#[test]
fn test_recovery_snapshot_list_sorted_by_timestamp() {
    let app_data_dir = create_temp_app_data_dir();

    let snapshot1 = RecoverySnapshot::new(
        Some("file1.md".to_string()),
        "Content 1".to_string(),
        0,
        "First".to_string(),
    );
    snapshot1.save(&app_data_dir).unwrap();

    std::thread::sleep(Duration::from_millis(50));

    let snapshot2 = RecoverySnapshot::new(
        Some("file2.md".to_string()),
        "Content 2".to_string(),
        0,
        "Second".to_string(),
    );
    snapshot2.save(&app_data_dir).unwrap();

    let snapshots = RecoverySnapshot::list(&app_data_dir).unwrap();

    assert!(snapshots.len() >= 2);
    assert_eq!(snapshots[0].title, "Second");
    assert_eq!(snapshots[1].title, "First");
}

#[test]
fn test_recovery_snapshot_delete() {
    let app_data_dir = create_temp_app_data_dir();

    let snapshot = RecoverySnapshot::new(
        None,
        "To be deleted".to_string(),
        0,
        "Delete Me".to_string(),
    );

    let snapshot_id = snapshot.id.clone();
    snapshot.save(&app_data_dir).unwrap();

    let snapshots = RecoverySnapshot::list(&app_data_dir).unwrap();
    assert!(snapshots.iter().any(|s| s.id == snapshot_id));

    RecoverySnapshot::delete(&snapshot_id, &app_data_dir).unwrap();

    let snapshots = RecoverySnapshot::list(&app_data_dir).unwrap();
    assert!(!snapshots.iter().any(|s| s.id == snapshot_id));
}

#[test]
fn test_autosave_multiple_documents() {
    let app_data_dir = create_temp_app_data_dir();
    let mut service = AutosaveService::new(app_data_dir.clone());

    let doc1 = Uuid::new_v4();
    let doc2 = Uuid::new_v4();
    let doc3 = Uuid::new_v4();

    service.mark_dirty(doc1, compute_content_hash("Doc 1 content"));
    service.mark_dirty(doc2, compute_content_hash("Doc 2 content"));

    assert!(service.needs_autosave(doc1));
    assert!(service.needs_autosave(doc2));
    assert!(!service.needs_autosave(doc3));

    service.record_autosave(doc1, compute_content_hash("Doc 1 content"));

    assert!(!service.needs_autosave(doc1));
    assert!(service.needs_autosave(doc2));
}

// =============================================================================
// TC-G015-001: Rust autosave triggers
// Category: integration
// Input: Edit document -> wait for debounce
// Expected: File saved automatically after debounce period
// =============================================================================
#[test]
fn test_tc_g015_001_rust_autosave_triggers() {
    let app_data_dir = create_temp_app_data_dir();
    let mut service = AutosaveService::new(app_data_dir.clone());

    // Configure autosave with 500ms debounce for faster testing
    service.set_config(AutosaveConfig {
        enabled: true,
        interval_ms: 30000,
        debounce_ms: 500,
    });

    let doc_id = Uuid::new_v4();
    let content =
        "# Test Document for Autosave\n\nThis content should be autosaved after debounce.";

    // Initially, document is marked dirty
    service.mark_dirty(doc_id, compute_content_hash(content));

    // Verify autosave is needed but not yet allowed (debounce not elapsed)
    assert!(
        service.needs_autosave(doc_id),
        "Document should need autosave immediately after edit"
    );
    assert!(
        !service.can_autosave(doc_id),
        "Should NOT autosave immediately - debounce not elapsed"
    );

    // Wait for debounce period to elapse
    std::thread::sleep(Duration::from_millis(600));

    // Now autosave should be allowed
    assert!(
        service.can_autosave(doc_id),
        "Autosave should be allowed after debounce period"
    );

    // Simulate saving - create recovery snapshot
    let doc =
        rustnote_lib::model::document::Document::from_file("test_autosave.md", content.to_string())
            .unwrap();

    let snapshot_id = service
        .create_recovery_snapshot(&doc, 20)
        .expect("Should create recovery snapshot");

    // Record that autosave was performed
    service.record_autosave(doc_id, compute_content_hash(content));

    // Verify autosave state is reset
    assert!(
        !service.can_autosave(doc_id),
        "Should not trigger another autosave immediately after save"
    );
    assert!(
        !service.needs_autosave(doc_id),
        "Should not need autosave after recording save"
    );

    // Verify snapshot was saved
    let snapshots = RecoverySnapshot::list(&app_data_dir).expect("Should list snapshots");
    let found = snapshots.iter().find(|s| s.id == snapshot_id);
    assert!(
        found.is_some(),
        "Recovery snapshot should exist after autosave"
    );
    assert_eq!(
        found.unwrap().content,
        content,
        "Snapshot content should match original"
    );
}

// =============================================================================
// TC-G015-002: Crash recovery
// Category: integration
// Input: Edit -> crash before frontend autosave
// Expected: Edited content recoverable from crash
// =============================================================================
#[test]
fn test_tc_g015_002_crash_recovery() {
    let app_data_dir = create_temp_app_data_dir();
    let service = AutosaveService::new(app_data_dir.clone());

    // Scenario: User edits document but app crashes before frontend timer fires
    // The Rust-side recovery system should still have the content

    let original_content = "# Original Document\n\nInitial content.";
    let crash_content =
        "# Original Document\n\nInitial content.\n\nThis was added just before the crash!";
    let cursor_offset = 65; // Position after "crash!"

    // Simulate crash scenario: save recovery snapshot from Rust side
    // (frontend never got a chance to trigger its timer)
    let snapshot = RecoverySnapshot::new(
        Some("/test/crash_test.md".to_string()),
        crash_content.to_string(),
        cursor_offset,
        "Crash Test".to_string(),
    );

    let snapshot_id = snapshot.id.clone();
    snapshot
        .save(&app_data_dir)
        .expect("Snapshot save should succeed even in crash scenario");

    // Verify snapshot was saved to disk
    let snapshots_dir = app_data_dir.join("snapshots");
    let snapshot_path = snapshots_dir.join(format!("{}.json", snapshot_id));
    assert!(
        snapshot_path.exists(),
        "Snapshot file should exist on disk for crash recovery"
    );

    // Simulate recovery after app restart
    let snapshots = RecoverySnapshot::list(&app_data_dir).expect("Should list snapshots");
    let recovered_snapshot = snapshots
        .iter()
        .find(|s| s.id == snapshot_id)
        .expect("Should find the crash recovery snapshot");

    let recovered = recovered_snapshot.restore();

    // Verify recovered content matches what was saved before crash
    assert_eq!(
        recovered.content, crash_content,
        "Recovered content should match what was saved before crash"
    );
    assert_eq!(
        recovered.cursor_offset, cursor_offset,
        "Cursor position should be recovered"
    );
    assert_eq!(
        recovered.file_path,
        Some("/test/crash_test.md".to_string()),
        "File path should be recovered"
    );
    assert_eq!(recovered.title, "Crash Test", "Title should be recovered");

    // Verify the content is different from original (proving edit was captured)
    assert_ne!(
        recovered.content, original_content,
        "Recovered content should include edits made before crash"
    );
}

// =============================================================================
// TC-G015-003: Autosave doesn't interfere
// Category: integration
// Input: Continuous typing with autosave enabled
// Expected: No lag or interruption from autosave
// =============================================================================
#[test]
fn test_tc_g015_003_autosave_doesnt_interfere() {
    let app_data_dir = create_temp_app_data_dir();
    let mut service = AutosaveService::new(app_data_dir.clone());

    // Configure with very short debounce for testing
    service.set_config(AutosaveConfig {
        enabled: true,
        interval_ms: 10000,
        debounce_ms: 50, // Very short debounce
    });

    let doc_id = Uuid::new_v4();
    let base_content = "# Continuous Typing Test";

    // Mark initial content
    service.mark_dirty(doc_id, compute_content_hash(base_content));

    // Simulate continuous rapid typing (10 edits in quick succession)
    let edit_count = 10;
    let mut last_was_able_to_save = false;

    for i in 0..edit_count {
        let content = format!("# Continuous Typing Test\n\nEdit number {}", i + 1);
        let hash = compute_content_hash(&content);

        // Mark dirty with new content
        service.mark_dirty(doc_id, hash);

        // Check if autosave became possible at any point during rapid edits
        let can_save_now = service.can_autosave(doc_id);

        // Record save if possible (simulating what would happen)
        if can_save_now && !last_was_able_to_save {
            service.record_autosave(doc_id, hash);
        }
        last_was_able_to_save = can_save_now;

        // Small delay between edits (simulating typing speed)
        std::thread::sleep(Duration::from_millis(10));
    }

    // After rapid edits stop, verify service is still responsive
    let final_content = "# Continuous Typing Test\n\nEdit number 10";
    service.mark_dirty(doc_id, compute_content_hash(final_content));

    // Service should still be tracking the document
    assert!(
        service.is_tracked(doc_id),
        "Document should still be tracked after rapid edits"
    );
    assert!(
        service.needs_autosave(doc_id),
        "Should need autosave after final edit"
    );

    // Wait for debounce
    std::thread::sleep(Duration::from_millis(100));

    // Should be able to autosave
    assert!(
        service.can_autosave(doc_id),
        "Should be able to autosave after debounce following rapid edits"
    );

    // Verify no state corruption - can still create snapshots
    let doc = rustnote_lib::model::document::Document::from_file(
        "rapid_edits.md",
        final_content.to_string(),
    )
    .unwrap();

    let result = service.create_recovery_snapshot(&doc, 30);
    assert!(
        result.is_ok(),
        "Should be able to create recovery snapshot after rapid edits - no interference"
    );
}

// =============================================================================
// Edge case: crash_during_edit
// Test that crash during edit still allows recovery of the last saved state
// =============================================================================
#[test]
fn test_edge_case_crash_during_edit() {
    let app_data_dir = create_temp_app_data_dir();
    let service = AutosaveService::new(app_data_dir.clone());

    // Scenario: User is in middle of editing, app crashes
    // Recovery should provide best-effort content

    let content_before_crash = "# Document\n\nContent before crash.";
    let content_during_crash = "# Document\n\nContent before crash.\n\nNew paragraph that";

    // Save snapshot representing state during crash
    let snapshot = RecoverySnapshot::new(
        Some("/test/crash_during_edit.md".to_string()),
        content_during_crash.to_string(),
        55, // Cursor in middle of incomplete sentence
        "Crash During Edit".to_string(),
    );

    let snapshot_id = snapshot.id.clone();
    snapshot
        .save(&app_data_dir)
        .expect("Should save snapshot even with incomplete content");

    // Simulate app restart and recovery
    let snapshots = RecoverySnapshot::list(&app_data_dir).expect("Should list snapshots");
    let found = snapshots.iter().find(|s| s.id == snapshot_id);

    assert!(
        found.is_some(),
        "Snapshot should exist even for incomplete edit"
    );

    let recovered = found.unwrap().restore();

    // Content is recovered (even if incomplete)
    assert_eq!(
        recovered.content, content_during_crash,
        "Should recover content that existed at time of crash"
    );
    assert_eq!(
        recovered.cursor_offset, 55,
        "Should recover cursor position at time of crash"
    );

    // Verify file_path is preserved
    assert_eq!(
        recovered.file_path,
        Some("/test/crash_during_edit.md".to_string()),
        "File path should be preserved for recovery"
    );
}
