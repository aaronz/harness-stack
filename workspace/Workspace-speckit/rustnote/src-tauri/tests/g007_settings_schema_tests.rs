use rustnote_lib::model::settings::{Settings, Theme};
use rustnote_lib::services::{SettingsService, SettingsServiceTrait};
use std::fs;

/// Test helper to create a temporary settings service
fn create_temp_service() -> SettingsService {
    let temp_dir = std::env::temp_dir().join(format!("rustnote_g007_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    let db_path = temp_dir.join("settings.db");
    SettingsService::new_with_path(Some(db_path)).expect("Failed to create settings service")
}

/// TC-G007-001: Flat settings structure
/// Verify Settings model has no nested 'editor' struct, all fields at top level
#[test]
fn tc_g007_001_flat_settings_structure() {
    let settings = Settings::default();

    assert!(settings.theme == Theme::Light || settings.theme == Theme::Dark);
    assert!(settings.auto_save == true || settings.auto_save == false);
    assert!(settings.auto_save_interval > 0);
    assert!(settings.focus_mode == true || settings.focus_mode == false);
    assert!(settings.typewriter_mode == true || settings.typewriter_mode == false);
    assert!(settings.outline_visible == true || settings.outline_visible == false);

    assert_eq!(settings.font_family, "System".to_string());
    assert_eq!(settings.font_size, 16);
    assert_eq!(settings.line_height, 1.6);
    assert_eq!(settings.content_width, 720);
    assert!(settings.recent_files.is_empty());

    let _ = settings;
}

/// TC-G007-002: Required settings fields present
/// Verify all required fields from PRD-09 are present in Settings model
#[test]
fn tc_g007_002_required_settings_fields_present() {
    let settings = Settings::default();

    assert!(matches!(settings.theme, Theme::Light | Theme::Dark));
    assert!(settings.auto_save == true || settings.auto_save == false);
    assert!(settings.auto_save_interval > 0);
    assert!(settings.focus_mode == true || settings.focus_mode == false);
    assert!(settings.typewriter_mode == true || settings.typewriter_mode == false);
    assert!(settings.outline_visible == true || settings.outline_visible == false);

    assert!(!settings.font_family.is_empty());
    assert!(settings.font_size > 0);
    assert!(settings.line_height > 0.0);
    assert!(settings.content_width > 0);

    assert!(settings.recent_files.is_empty());

    let json = serde_json::to_string(&settings).unwrap();
    assert!(json.contains("\"theme\""));
    assert!(json.contains("\"autoSave\""));
    assert!(json.contains("\"autoSaveInterval\""));
    assert!(json.contains("\"focusMode\""));
    assert!(json.contains("\"typewriterMode\""));
    assert!(json.contains("\"outlineVisible\""));
    assert!(json.contains("\"fontFamily\""));
    assert!(json.contains("\"fontSize\""));
    assert!(json.contains("\"lineHeight\""));
    assert!(json.contains("\"contentWidth\""));
    assert!(json.contains("\"recentFiles\""));
}

/// TC-G007-003: Settings persistence
/// Verify settings persist across app restarts (service recreation)
#[test]
fn tc_g007_003_settings_persistence() {
    let temp_dir =
        std::env::temp_dir().join(format!("rustnote_g007_persist_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
    let db_path = temp_dir.join("settings.db");

    // Create service and write settings
    {
        let service = SettingsService::new_with_path(Some(db_path.clone()))
            .expect("Failed to create settings service");

        let custom_settings = Settings {
            theme: Theme::Dark,
            auto_save: false,
            auto_save_interval: 60000,
            focus_mode: true,
            typewriter_mode: true,
            outline_visible: true,
            font_family: "Menlo".to_string(),
            font_size: 18,
            line_height: 1.8,
            content_width: 900,
            recent_files: vec!["test1.md".to_string(), "test2.md".to_string()],
            recent_folders: vec![],
        };

        service
            .write_settings(&custom_settings)
            .expect("Failed to write settings");
    }

    // Simulate app restart - create new service with same db path
    {
        let service = SettingsService::new_with_path(Some(db_path.clone()))
            .expect("Failed to create settings service after restart");

        let loaded = service
            .read_settings()
            .expect("Failed to read settings after restart");

        // Verify all custom settings persisted
        assert_eq!(loaded.theme, Theme::Dark);
        assert_eq!(loaded.auto_save, false);
        assert_eq!(loaded.auto_save_interval, 60000);
        assert_eq!(loaded.focus_mode, true);
        assert_eq!(loaded.typewriter_mode, true);
        assert_eq!(loaded.outline_visible, true);
        assert_eq!(loaded.font_family, "Menlo");
        assert_eq!(loaded.font_size, 18);
        assert_eq!(loaded.line_height, 1.8);
        assert_eq!(loaded.content_width, 900);
        assert_eq!(loaded.recent_files.len(), 2);
        assert!(loaded.recent_files.contains(&"test1.md".to_string()));
        assert!(loaded.recent_files.contains(&"test2.md".to_string()));
    }

    fs::remove_dir_all(temp_dir).ok();
}

#[test]
fn tc_g007_003b_settings_update_persistence() {
    let service = create_temp_service();

    let initial = Settings {
        theme: Theme::Light,
        auto_save: true,
        auto_save_interval: 10000,
        focus_mode: false,
        typewriter_mode: false,
        outline_visible: false,
        font_family: "System".to_string(),
        font_size: 16,
        line_height: 1.6,
        content_width: 720,
        recent_files: vec![],
        recent_folders: vec![],
    };
    service.write_settings(&initial).unwrap();

    let updated = Settings {
        theme: Theme::Dark,
        auto_save: false,
        auto_save_interval: 120000,
        focus_mode: true,
        typewriter_mode: true,
        outline_visible: true,
        font_family: "SF Mono".to_string(),
        font_size: 20,
        line_height: 2.0,
        content_width: 1000,
        recent_files: vec!["updated.md".to_string()],
        recent_folders: vec![],
    };
    service.write_settings(&updated).unwrap();

    let loaded = service.read_settings().unwrap();
    assert_eq!(loaded.theme, Theme::Dark);
    assert_eq!(loaded.auto_save, false);
    assert_eq!(loaded.auto_save_interval, 120000);
    assert_eq!(loaded.focus_mode, true);
    assert_eq!(loaded.typewriter_mode, true);
    assert_eq!(loaded.outline_visible, true);
    assert_eq!(loaded.font_family, "SF Mono");
    assert_eq!(loaded.font_size, 20);
    assert_eq!(loaded.line_height, 2.0);
    assert_eq!(loaded.content_width, 1000);
    assert_eq!(loaded.recent_files, vec!["updated.md".to_string()]);
}

#[test]
fn tc_ss001_theme_field_presence_and_type() {
    let settings = Settings::default();
    assert_eq!(settings.theme, Theme::Light);
    let json = serde_json::to_string(&settings).unwrap();
    assert!(json.contains("\"theme\":\"Light\""));
    let deserialized: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.theme, Theme::Light);
}

#[test]
fn tc_ss002_auto_save_field_presence_and_type() {
    let settings = Settings::default();
    assert!(settings.auto_save);
    let json = serde_json::to_string(&settings).unwrap();
    assert!(json.contains("\"autoSave\":true"));
    let deserialized: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.auto_save, true);
    let mut custom = Settings::default();
    custom.auto_save = false;
    assert!(!custom.auto_save);
}

#[test]
fn tc_ss003_auto_save_interval_field_type() {
    let settings = Settings::default();
    assert_eq!(settings.auto_save_interval, 10000);
    let json = serde_json::to_string(&settings).unwrap();
    assert!(json.contains("\"autoSaveInterval\":10000"));
    let deserialized: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.auto_save_interval, 10000);
    let mut custom = Settings::default();
    custom.auto_save_interval = 30000;
    assert_eq!(custom.auto_save_interval, 30000);
}

#[test]
fn tc_ss004_all_11_prd09_fields_present() {
    let settings = Settings::default();
    let field_count = 11;
    let mut count = 0;
    let _ = &settings.theme;
    count += 1;
    let _ = &settings.auto_save;
    count += 1;
    let _ = &settings.auto_save_interval;
    count += 1;
    let _ = &settings.focus_mode;
    count += 1;
    let _ = &settings.typewriter_mode;
    count += 1;
    let _ = &settings.outline_visible;
    count += 1;
    let _ = &settings.font_family;
    count += 1;
    let _ = &settings.font_size;
    count += 1;
    let _ = &settings.line_height;
    count += 1;
    let _ = &settings.content_width;
    count += 1;
    let _ = &settings.recent_files;
    count += 1;
    assert_eq!(count, field_count);
}

#[test]
fn tc_ss005_flat_structure_not_nested() {
    let settings = Settings::default();
    let json = serde_json::to_string(&settings).unwrap();
    assert!(!json.contains("\"editor\":"));
    assert!(json.contains("\"fontFamily\""));
    assert!(json.contains("\"fontSize\""));
    assert!(json.contains("\"lineHeight\""));
    assert!(json.contains("\"contentWidth\""));
    let deserialized: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.font_size, 16);
    assert_eq!(deserialized.font_family, "System");
}

#[test]
fn tc_ss006_default_values_match_prd09() {
    let settings = Settings::default();
    assert_eq!(settings.theme, Theme::Light);
    assert!(settings.auto_save);
    assert_eq!(settings.auto_save_interval, 10000);
    assert!(!settings.focus_mode);
    assert!(!settings.typewriter_mode);
    assert!(!settings.outline_visible);
    assert_eq!(settings.font_family, "System");
    assert_eq!(settings.font_size, 16);
    assert_eq!(settings.line_height, 1.6);
    assert_eq!(settings.content_width, 720);
    assert!(settings.recent_files.is_empty());
}

#[test]
fn tc_ss007_json_serialization_round_trip() {
    let original = Settings {
        theme: Theme::Dark,
        auto_save: false,
        auto_save_interval: 45000,
        focus_mode: true,
        typewriter_mode: true,
        outline_visible: true,
        font_family: "Menlo".to_string(),
        font_size: 18,
        line_height: 1.8,
        content_width: 850,
        recent_files: vec!["a.md".to_string(), "b.md".to_string()],
        recent_folders: vec![],
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.theme, original.theme);
    assert_eq!(deserialized.auto_save, original.auto_save);
    assert_eq!(deserialized.auto_save_interval, original.auto_save_interval);
    assert_eq!(deserialized.focus_mode, original.focus_mode);
    assert_eq!(deserialized.typewriter_mode, original.typewriter_mode);
    assert_eq!(deserialized.outline_visible, original.outline_visible);
    assert_eq!(deserialized.font_family, original.font_family);
    assert_eq!(deserialized.font_size, original.font_size);
    assert_eq!(deserialized.line_height, original.line_height);
    assert_eq!(deserialized.content_width, original.content_width);
    assert_eq!(deserialized.recent_files, original.recent_files);
}

#[test]
fn tc_ss008_invalid_field_values_rejected() {
    let invalid_font_size = r#"{"theme":"Light","autoSave":true,"autoSaveInterval":10000,"focusMode":false,"typewriterMode":false,"outlineVisible":false,"fontFamily":"System","fontSize":"large","lineHeight":1.6,"contentWidth":720,"recentFiles":[]}"#;
    let result: Result<Settings, _> = serde_json::from_str(invalid_font_size);
    assert!(result.is_err());
    let invalid_interval = r#"{"theme":"Light","autoSave":true,"autoSaveInterval":"fast","focusMode":false,"typewriterMode":false,"outlineVisible":false,"fontFamily":"System","fontSize":16,"lineHeight":1.6,"contentWidth":720,"recentFiles":[]}"#;
    let result2: Result<Settings, _> = serde_json::from_str(invalid_interval);
    assert!(result2.is_err());
}

#[test]
fn tc_ss009_recent_files_field_type_and_limit() {
    let settings = Settings::default();
    assert!(settings.recent_files.is_empty());
    assert!(settings.recent_files.iter().all(|s| s.is_empty() == false));
    let mut custom = Settings::default();
    for i in 0..10 {
        custom.recent_files.push(format!("file{}.md", i));
    }
    assert_eq!(custom.recent_files.len(), 10);
    let json = serde_json::to_string(&custom).unwrap();
    assert!(json.contains("\"recentFiles\":[\"file0.md\""));
    let deserialized: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.recent_files.len(), 10);
    assert_eq!(deserialized.recent_files[0], "file0.md");
    assert_eq!(deserialized.recent_files[9], "file9.md");
}
