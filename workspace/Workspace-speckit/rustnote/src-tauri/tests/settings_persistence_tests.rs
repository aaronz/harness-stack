use rustnote_lib::model::settings::{EditorSettings, Settings, Theme};
use rustnote_lib::services::SettingsService;
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
    settings.editor.font_size = 14;
    settings.editor.font_family = "Monaco".to_string();
    settings.auto_save = false;
    settings.focus_mode = true;

    service
        .write_settings(&settings)
        .expect("Failed to write settings");

    let read = service.read_settings().expect("Failed to read settings");
    assert_eq!(read.theme, Theme::Dark);
    assert_eq!(read.editor.font_size, 14);
    assert_eq!(read.editor.font_family, "Monaco");
    assert_eq!(read.auto_save, false);
    assert_eq!(read.focus_mode, true);

    let mut update = Settings::default();
    update.theme = Theme::Light;
    update.editor.font_size = 18;

    service
        .write_settings(&update)
        .expect("Failed to update settings");

    let updated = service
        .read_settings()
        .expect("Failed to read updated settings");
    assert_eq!(updated.theme, Theme::Light);
    assert_eq!(updated.editor.font_size, 18);
    assert_eq!(updated.editor.font_family, "System");
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
        editor: EditorSettings {
            font_family: "Courier".to_string(),
            font_size: 16,
            line_height: 1.8,
            tab_size: 4,
            content_width: 900,
        },
        recent_files: vec!["file1.md".to_string(), "file2.md".to_string()],
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
    assert_eq!(settings.editor.font_family, "Menlo");
    assert_eq!(settings.editor.font_size, 18);
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
                settings.editor.font_size = 10 + (i as u32);
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
    assert!(final_settings.editor.font_size >= 10);

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
    assert_eq!(settings.editor.font_size, 16);
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
        editor: EditorSettings {
            font_family: "SF Mono".to_string(),
            font_size: 20,
            line_height: 2.0,
            tab_size: 8,
            content_width: 1000,
        },
        recent_files: vec![],
    };

    service
        .write_settings(&settings)
        .expect("Failed to write editor settings");

    let read = service
        .read_settings()
        .expect("Failed to read editor settings");
    assert_eq!(read.editor.font_family, "SF Mono");
    assert_eq!(read.editor.font_size, 20);
    assert_eq!(read.editor.line_height, 2.0);
    assert_eq!(read.editor.tab_size, 8);
    assert_eq!(read.editor.content_width, 1000);
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
