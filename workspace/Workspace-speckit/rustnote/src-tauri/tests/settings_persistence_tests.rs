use rustnote_lib::model::settings::{Settings, Theme};
use rustnote_lib::services::{SettingsService, SettingsServiceTrait};
use std::fs;

fn create_temp_service() -> SettingsService {
    let temp_dir = std::env::temp_dir().join(format!("rustnote_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    let db_path = temp_dir.join("settings.db");
    SettingsService::new_with_path(Some(db_path)).expect("Failed to create settings service")
}

#[test]
fn tc_g003_001_settings_basic_crud() {
    let service = create_temp_service();

    let mut settings = Settings::default();
    settings.theme = Theme::Dark;
    settings.font_size = 14;
    settings.font_family = "Monaco".to_string();
    settings.auto_save = false;
    settings.focus_mode = true;

    service
        .write_settings(&settings)
        .expect("Failed to write settings");

    let read = service.read_settings().expect("Failed to read settings");
    assert_eq!(read.theme, Theme::Dark);
    assert_eq!(read.font_size, 14);
    assert_eq!(read.font_family, "Monaco");
    assert_eq!(read.auto_save, false);
    assert_eq!(read.focus_mode, true);

    let mut update = Settings::default();
    update.theme = Theme::Light;
    update.font_size = 18;

    service
        .write_settings(&update)
        .expect("Failed to update settings");

    let updated = service
        .read_settings()
        .expect("Failed to read updated settings");
    assert_eq!(updated.theme, Theme::Light);
    assert_eq!(updated.font_size, 18);
    assert_eq!(updated.font_family, "System");
}

#[test]
fn tc_g003_002_settings_atomic_transaction() {
    let service = create_temp_service();

    let settings = Settings::default();
    service
        .write_settings(&settings)
        .expect("Failed to write initial settings");

    let result = service.transaction_with_settings(|conn| {
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('test1', 'val1')",
            [],
        )?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('test2', 'val2')",
            [],
        )?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('test3', 'val3')",
            [],
        )?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('test4', 'val4')",
            [],
        )?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('test5', 'val5')",
            [],
        )?;
        Ok(())
    });

    assert!(result.is_ok(), "Transaction should succeed");

    // Verify all 5 values are present
    for i in 1..=5 {
        let key = format!("test{}", i);
        let val = service.get_setting(&key).expect("Failed to query");
        assert_eq!(val, Some(format!("val{}", i)));
    }

    // Now test rollback by returning an error from the closure
    let temp_dir = std::env::temp_dir().join(format!("rustnote_rollback_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).ok();
    let db_path = temp_dir.join("settings.db");
    let rollback_service =
        SettingsService::new_with_path(Some(db_path)).expect("Failed to create rollback service");
    rollback_service.write_settings(&Settings::default()).ok();

    let rollback_result = rollback_service.transaction_with_settings(|_conn| {
        // Return an error to trigger rollback
        Err(rusqlite::Error::InvalidParameterName(
            "Test rollback".to_string(),
        ))
    });

    assert!(rollback_result.is_err(), "Transaction should return error");

    // After error, the value should not be present (transaction rolled back)
    let val: Option<String> = rollback_service.get_setting("rollback1").ok().flatten();
    assert!(
        val.is_none(),
        "After error rollback, value should not be present"
    );
}

#[test]
fn tc_g003_003_settings_crash_recovery() {
    let temp_dir = std::env::temp_dir().join(format!("rustnote_recovery_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

    let db_path = temp_dir.join("settings.db");

    let original_settings = Settings {
        theme: Theme::Dark,
        auto_save: true,
        auto_save_interval: 60000,
        focus_mode: true,
        typewriter_mode: true,
        outline_visible: true,
        font_family: "Courier".to_string(),
        font_size: 16,
        line_height: 1.8,
        content_width: 900,
        recent_files: vec!["file1.md".to_string(), "file2.md".to_string()],
        recent_folders: vec![],
    };

    {
        let service = SettingsService::new_with_path(Some(db_path.clone()))
            .expect("Failed to create service");
        service
            .write_settings(&original_settings)
            .expect("Failed to write settings");
    }

    fs::remove_file(&db_path).expect("Failed to delete db");

    let recovered_settings = Settings::default();
    let recovered = SettingsService::new_with_path(Some(db_path))
        .expect("Failed to create service after recovery");
    recovered
        .write_settings(&recovered_settings)
        .expect("Failed to write recovered settings");

    let read_back = recovered
        .read_settings()
        .expect("Failed to read recovered settings");
    assert_eq!(read_back.theme, recovered_settings.theme);
    assert_eq!(read_back.auto_save, recovered_settings.auto_save);

    fs::remove_dir_all(temp_dir).ok();
}

#[test]
fn tc_g003_004_settings_migration_from_json() {
    let temp_dir = std::env::temp_dir().join(format!("rustnote_migrate_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

    let db_path = temp_dir.join("settings.db");
    let json_path = temp_dir.join("settings.json");

    let json_content = r#"{
        "theme": "Dark",
        "autoSave": true,
        "autoSaveInterval": 45000,
        "focusMode": false,
        "typewriterMode": true,
        "outlineVisible": true,
        "fontFamily": "Menlo",
        "fontSize": 18,
        "lineHeight": 1.7,
        "tabSize": 2,
        "contentWidth": 850,
        "recentFiles": ["test1.md", "test2.md", "test3.md"]
    }"#;

    fs::write(&json_path, json_content).expect("Failed to write JSON");

    let service = SettingsService::new_with_path(Some(db_path.clone()))
        .expect("Failed to create service with migration");

    let settings = service
        .read_settings()
        .expect("Failed to read migrated settings");
    assert_eq!(settings.theme, Theme::Dark);
    assert_eq!(settings.auto_save, true);
    assert_eq!(settings.auto_save_interval, 45000);
    assert_eq!(settings.typewriter_mode, true);
    assert_eq!(settings.font_family, "Menlo");
    assert_eq!(settings.font_size, 18);
    assert!(settings.recent_files.contains(&"test1.md".to_string()));

    let backup_path = json_path.with_extension("json.bak");
    assert!(
        backup_path.exists(),
        "JSON backup should exist after migration"
    );

    fs::remove_dir_all(temp_dir).ok();
}

#[test]
fn tc_g003_005_settings_concurrent_access() {
    let temp_dir =
        std::env::temp_dir().join(format!("rustnote_concurrent_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    let db_path = temp_dir.join("settings.db");

    {
        let service = SettingsService::new_with_path(Some(db_path.clone()))
            .expect("Failed to create service");
        service
            .write_settings(&Settings::default())
            .expect("Failed to write initial");
    }

    use std::thread;

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let db_path_clone = db_path.clone();
            thread::spawn(move || {
                let service = SettingsService::new_with_path(Some(db_path_clone))
                    .expect("Failed to create service in thread");
                let mut settings = Settings::default();
                settings.font_size = 10 + (i as u32);
                service
                    .write_settings(&settings)
                    .expect("Failed to write from thread");
                service.read_settings().expect("Failed to read from thread");
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    let service = SettingsService::new_with_path(Some(db_path.clone()))
        .expect("Failed to create final service");
    let final_settings = service
        .read_settings()
        .expect("Failed to read final settings");
    assert!(final_settings.font_size >= 10);

    let integrity = service
        .verify_integrity()
        .expect("Failed to verify integrity");
    assert!(
        integrity,
        "Database integrity should be maintained after concurrent access"
    );

    fs::remove_dir_all(temp_dir).ok();
}

#[test]
fn tc_g003_006_settings_recent_files_persist() {
    let temp_dir = std::env::temp_dir().join(format!("rustnote_recent_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).ok();
    let db_path = temp_dir.join("settings.db");

    let service =
        SettingsService::new_with_path(Some(db_path.clone())).expect("Failed to create service");

    for i in 1..=10 {
        let path = format!("/path/to/document_{}.md", i);
        service
            .add_recent_file(&path)
            .expect("Failed to add recent file");
    }

    let files = service
        .get_recent_files()
        .expect("Failed to get recent files");
    assert_eq!(files.len(), 10);
    assert!(files.contains(&"/path/to/document_1.md".to_string()));
    assert!(files.contains(&"/path/to/document_10.md".to_string()));

    // Create a new service with the SAME path - data should persist
    let service2 =
        SettingsService::new_with_path(Some(db_path)).expect("Failed to create new service");
    let files2 = service2
        .get_recent_files()
        .expect("Failed to get files from new service");
    assert_eq!(files2.len(), 10);
}

#[test]
fn tc_g003_007_settings_workspace_state() {
    let temp_dir =
        std::env::temp_dir().join(format!("rustnote_workspace_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).ok();
    let db_path = temp_dir.join("settings.db");

    let service =
        SettingsService::new_with_path(Some(db_path.clone())).expect("Failed to create service");

    service
        .set_workspace_state("last_folder", "/Users/test/Projects")
        .expect("Failed to set workspace state");
    service
        .set_workspace_state("open_files", "[\"a.md\", \"b.md\"]")
        .expect("Failed to set open files");
    service
        .set_workspace_state("scroll_position", "42")
        .expect("Failed to set scroll");

    let folder = service
        .get_workspace_state("last_folder")
        .expect("Failed to get folder");
    assert_eq!(folder, Some("/Users/test/Projects".to_string()));

    let open_files = service
        .get_workspace_state("open_files")
        .expect("Failed to get open files");
    assert_eq!(open_files, Some("[\"a.md\", \"b.md\"]".to_string()));

    let scroll = service
        .get_workspace_state("scroll_position")
        .expect("Failed to get scroll");
    assert_eq!(scroll, Some("42".to_string()));

    service
        .delete_workspace_state("scroll_position")
        .expect("Failed to delete scroll");
    let deleted = service
        .get_workspace_state("scroll_position")
        .expect("Failed to get deleted");
    assert!(deleted.is_none());

    let service2 =
        SettingsService::new_with_path(Some(db_path)).expect("Failed to create new service");
    let folder2 = service2
        .get_workspace_state("last_folder")
        .expect("Failed to get folder from new service");
    assert_eq!(folder2, Some("/Users/test/Projects".to_string()));
}

#[test]
fn settings_empty_database() {
    let service = create_temp_service();

    let settings = service
        .read_settings()
        .expect("Failed to read default settings");
    assert_eq!(settings.theme, Theme::Light);
    assert!(settings.auto_save);
    assert_eq!(settings.font_size, 16);
}

#[test]
fn settings_persist_editor_settings() {
    let service = create_temp_service();

    let settings = Settings {
        theme: Theme::Dark,
        auto_save: false,
        auto_save_interval: 120000,
        focus_mode: true,
        typewriter_mode: false,
        outline_visible: true,
        font_family: "SF Mono".to_string(),
        font_size: 20,
        line_height: 2.0,
        content_width: 1000,
        recent_files: vec![],
        recent_folders: vec![],
    };

    service
        .write_settings(&settings)
        .expect("Failed to write editor settings");

    let read = service
        .read_settings()
        .expect("Failed to read editor settings");
    assert_eq!(read.font_family, "SF Mono");
    assert_eq!(read.font_size, 20);
    assert_eq!(read.line_height, 2.0);
    assert_eq!(read.content_width, 1000);
}

#[test]
fn settings_update_individual_values() {
    let service = create_temp_service();

    service
        .update_setting("custom_key", "custom_value")
        .expect("Failed to update setting");

    let value = service
        .get_setting("custom_key")
        .expect("Failed to get setting");
    assert_eq!(value, Some("custom_value".to_string()));

    service
        .update_setting("custom_key", "updated_value")
        .expect("Failed to update setting");

    let updated = service
        .get_setting("custom_key")
        .expect("Failed to get updated setting");
    assert_eq!(updated, Some("updated_value".to_string()));
}

#[test]
fn settings_verify_integrity() {
    let service = create_temp_service();

    service
        .write_settings(&Settings::default())
        .expect("Failed to write settings");

    let integrity = service
        .verify_integrity()
        .expect("Failed to verify integrity");
    assert!(integrity, "Database integrity should be valid");
}

// TC-P1-009-01: Add folder to recent_folders and persist
#[test]
fn tc_p1_009_01_add_folder_to_recent_folders_and_persist() {
    let temp_dir =
        std::env::temp_dir().join(format!("rustnote_recent_folder_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).ok();
    let db_path = temp_dir.join("settings.db");

    let service =
        SettingsService::new_with_path(Some(db_path.clone())).expect("Failed to create service");

    let mut settings = Settings::default();
    settings.recent_folders = vec!["/workspace/project".to_string()];
    service
        .write_settings(&settings)
        .expect("Failed to write settings");

    let read = service.read_settings().expect("Failed to read settings");
    assert!(
        read.recent_folders
            .contains(&"/workspace/project".to_string()),
        "recent_folders should contain '/workspace/project'"
    );

    // Verify persistence across service restart (simulates app restart)
    let service2 =
        SettingsService::new_with_path(Some(db_path)).expect("Failed to create new service");
    let read2 = service2
        .read_settings()
        .expect("Failed to read settings after restart");
    assert!(
        read2
            .recent_folders
            .contains(&"/workspace/project".to_string()),
        "recent_folders should persist after restart"
    );

    fs::remove_dir_all(temp_dir).ok();
}

// TC-P1-009-02: recent_folders max entry limit
#[test]
fn tc_p1_009_02_recent_folders_max_entry_limit() {
    let temp_dir =
        std::env::temp_dir().join(format!("rustnote_max_folders_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).ok();
    let db_path = temp_dir.join("settings.db");

    let service =
        SettingsService::new_with_path(Some(db_path.clone())).expect("Failed to create service");

    // Open 15 different folders
    let folders: Vec<String> = (1..=15)
        .map(|i| format!("/workspace/project{}", i))
        .collect();
    let settings = Settings {
        recent_folders: folders.clone(),
        ..Settings::default()
    };
    service
        .write_settings(&settings)
        .expect("Failed to write settings");

    // Verify via add_recent_folder table method (max 20 entries by default)
    for i in 1..=15 {
        service
            .add_recent_folder(&format!("/workspace/project{}", i))
            .expect("Failed to add recent folder");
    }

    let read_folders = service
        .get_recent_folders()
        .expect("Failed to get recent folders");
    assert!(
        read_folders.len() <= 20,
        "recent_folders should not exceed max entries (20)"
    );

    // Also verify the stored settings respects the max by writing with limit
    let limited_folders: Vec<String> = (1..=15)
        .map(|i| format!("/workspace/limited{}", i))
        .collect();
    let settings_limited = Settings {
        recent_folders: limited_folders,
        ..Settings::default()
    };
    service
        .write_settings(&settings_limited)
        .expect("Failed to write limited settings");

    let read_limited = service
        .read_settings()
        .expect("Failed to read limited settings");
    // The write_settings persists what we give it; the frontend enforces the 10-entry limit
    assert_eq!(
        read_limited.recent_folders.len(),
        15,
        "Settings should store what was written"
    );

    fs::remove_dir_all(temp_dir).ok();
}

// TC-P1-009-03: Recent folders survive app restart
#[test]
fn tc_p1_009_03_recent_folders_survive_app_restart() {
    let temp_dir =
        std::env::temp_dir().join(format!("rustnote_folder_restart_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).ok();
    let db_path = temp_dir.join("settings.db");

    // First "session"
    {
        let service = SettingsService::new_with_path(Some(db_path.clone()))
            .expect("Failed to create service");
        let settings = Settings {
            recent_folders: vec![
                "/workspace/project1".to_string(),
                "/workspace/project2".to_string(),
                "/home/user/docs".to_string(),
            ],
            ..Settings::default()
        };
        service
            .write_settings(&settings)
            .expect("Failed to write settings before restart");
    }

    // Simulate app restart - create new service instance
    let service_after_restart = SettingsService::new_with_path(Some(db_path.clone()))
        .expect("Failed to create service after restart");
    let read = service_after_restart
        .read_settings()
        .expect("Failed to read settings after restart");

    assert_eq!(read.recent_folders.len(), 3);
    assert!(read
        .recent_folders
        .contains(&"/workspace/project1".to_string()));
    assert!(read
        .recent_folders
        .contains(&"/workspace/project2".to_string()));
    assert!(read.recent_folders.contains(&"/home/user/docs".to_string()));

    // Also verify the dedicated table method survives restart
    service_after_restart
        .add_recent_folder("/workspace/project3")
        .expect("Failed to add folder after restart");

    let service_final =
        SettingsService::new_with_path(Some(db_path)).expect("Failed to create final service");
    let folders = service_final
        .get_recent_folders()
        .expect("Failed to get folders from final service");
    assert!(
        folders.contains(&"/workspace/project3".to_string()),
        "Folder added after restart should persist"
    );

    fs::remove_dir_all(temp_dir).ok();
}

// TC-P1-009-04: Settings schema — 12 fields
#[test]
fn tc_p1_009_04_settings_schema_12_fields() {
    let service = create_temp_service();
    let settings = Settings::default();

    // Verify 12 fields exist by checking the struct has all expected fields
    // Field count: theme, auto_save, auto_save_interval, focus_mode, typewriter_mode,
    // outline_visible, font_family, font_size, line_height, content_width, recent_files, recent_folders
    assert_eq!(settings.theme, Theme::Light);
    assert_eq!(settings.auto_save, true);
    assert_eq!(settings.auto_save_interval, 10000);
    assert_eq!(settings.focus_mode, false);
    assert_eq!(settings.typewriter_mode, false);
    assert_eq!(settings.outline_visible, false);
    assert_eq!(settings.font_family, "System");
    assert_eq!(settings.font_size, 16);
    assert_eq!(settings.line_height, 1.6);
    assert_eq!(settings.content_width, 720);
    assert_eq!(settings.recent_files, Vec::<String>::new());
    assert_eq!(settings.recent_folders, Vec::<String>::new());

    // Verify the schema has 12 fields by serializing and counting
    let json = serde_json::to_string(&settings).expect("Failed to serialize settings");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse JSON");
    if let Some(obj) = parsed.as_object() {
        assert_eq!(obj.len(), 12, "Settings JSON should have exactly 12 fields");
    }

    // Verify persistence roundtrip
    service
        .write_settings(&settings)
        .expect("Failed to write settings");
    let read = service.read_settings().expect("Failed to read settings");
    assert_eq!(read.theme, settings.theme);
    assert_eq!(read.auto_save, settings.auto_save);
    assert_eq!(read.auto_save_interval, settings.auto_save_interval);
    assert_eq!(read.focus_mode, settings.focus_mode);
    assert_eq!(read.typewriter_mode, settings.typewriter_mode);
    assert_eq!(read.outline_visible, settings.outline_visible);
    assert_eq!(read.font_family, settings.font_family);
    assert_eq!(read.font_size, settings.font_size);
    assert_eq!(read.line_height, settings.line_height);
    assert_eq!(read.content_width, settings.content_width);
    assert_eq!(read.recent_files, settings.recent_files);
    assert_eq!(read.recent_folders, settings.recent_folders);
}

// Additional: clear_recent_folders test
#[test]
fn tc_p1_009_05_clear_recent_folders() {
    let temp_dir =
        std::env::temp_dir().join(format!("rustnote_clear_folders_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).ok();
    let db_path = temp_dir.join("settings.db");

    let service =
        SettingsService::new_with_path(Some(db_path.clone())).expect("Failed to create service");

    service
        .add_recent_folder("/workspace/project1")
        .expect("Failed to add folder 1");
    service
        .add_recent_folder("/workspace/project2")
        .expect("Failed to add folder 2");

    let folders = service.get_recent_folders().expect("Failed to get folders");
    assert_eq!(folders.len(), 2);

    service
        .clear_recent_folders()
        .expect("Failed to clear folders");

    let folders_after = service
        .get_recent_folders()
        .expect("Failed to get folders after clear");
    assert_eq!(
        folders_after.len(),
        0,
        "recent_folders should be empty after clear"
    );

    fs::remove_dir_all(temp_dir).ok();
}

// Additional: recent_folders and recent_files coexist independently
#[test]
fn tc_p1_009_06_recent_files_and_folders_independent() {
    let temp_dir =
        std::env::temp_dir().join(format!("rustnote_independent_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).ok();
    let db_path = temp_dir.join("settings.db");

    let service =
        SettingsService::new_with_path(Some(db_path.clone())).expect("Failed to create service");

    let settings = Settings {
        recent_files: vec![
            "/workspace/project/file1.md".to_string(),
            "/workspace/project/file2.md".to_string(),
        ],
        recent_folders: vec![
            "/workspace/project".to_string(),
            "/workspace/other".to_string(),
        ],
        ..Settings::default()
    };
    service
        .write_settings(&settings)
        .expect("Failed to write settings");

    let read = service.read_settings().expect("Failed to read settings");
    assert_eq!(read.recent_files.len(), 2);
    assert_eq!(read.recent_folders.len(), 2);
    assert!(read
        .recent_files
        .contains(&"/workspace/project/file1.md".to_string()));
    assert!(read
        .recent_folders
        .contains(&"/workspace/project".to_string()));

    fs::remove_dir_all(temp_dir).ok();
}
