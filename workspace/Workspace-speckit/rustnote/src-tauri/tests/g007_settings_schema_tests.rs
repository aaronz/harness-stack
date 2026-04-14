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

    // Verify all fields exist at top level (not nested in editor struct)
    // These fields should be directly on Settings, not on a nested editor struct
    assert!(settings.theme == Theme::Light || settings.theme == Theme::Dark);
    assert!(settings.auto_save == true || settings.auto_save == false);
    assert!(settings.auto_save_interval > 0);
    assert!(settings.focus_mode == true || settings.focus_mode == false);
    assert!(settings.typewriter_mode == true || settings.typewriter_mode == false);
    assert!(settings.outline_visible == true || settings.outline_visible == false);

    // Editor-related fields should be at top level, not nested
    assert_eq!(settings.font_family, "System".to_string());
    assert_eq!(settings.font_size, 16);
    assert_eq!(settings.line_height, 1.6);
    assert_eq!(settings.tab_size, 4);
    assert_eq!(settings.content_width, 720);
    assert!(settings.recent_files.is_empty());

    // Verify the struct doesn't have a nested editor field
    // This is compile-time verified - if there was an `editor` field of type EditorSettings,
    // the code would not compile
    let _ = settings;
}

/// TC-G007-002: Required settings fields present
/// Verify all required fields from PRD-09 are present in Settings model
#[test]
fn tc_g007_002_required_settings_fields_present() {
    // PRD-09 required fields:
    // - theme: 'light' | 'dark'
    // - autoSave: boolean
    // - autoSaveInterval: number
    // - focusMode: boolean
    // - typewriterMode: boolean
    // - outlineVisible: boolean
    // - fontSize: number
    // - fontFamily: string
    // - lineHeight: number
    // - contentWidth: number
    // - recentFiles: string[]

    let settings = Settings::default();

    // Core settings
    assert!(matches!(settings.theme, Theme::Light | Theme::Dark));
    assert!(settings.auto_save == true || settings.auto_save == false);
    assert!(settings.auto_save_interval > 0);
    assert!(settings.focus_mode == true || settings.focus_mode == false);
    assert!(settings.typewriter_mode == true || settings.typewriter_mode == false);
    assert!(settings.outline_visible == true || settings.outline_visible == false);

    // Editor typography settings
    assert!(!settings.font_family.is_empty());
    assert!(settings.font_size > 0);
    assert!(settings.line_height > 0.0);
    assert!(settings.content_width > 0);

    // Recent files
    assert!(settings.recent_files.is_empty());

    // Verify serialization includes all fields
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
    assert!(json.contains("\"tabSize\""));
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
            tab_size: 2,
            content_width: 900,
            recent_files: vec!["test1.md".to_string(), "test2.md".to_string()],
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
        assert_eq!(loaded.tab_size, 2);
        assert_eq!(loaded.content_width, 900);
        assert_eq!(loaded.recent_files.len(), 2);
        assert!(loaded.recent_files.contains(&"test1.md".to_string()));
        assert!(loaded.recent_files.contains(&"test2.md".to_string()));
    }

    fs::remove_dir_all(temp_dir).ok();
}

/// Additional test: Verify settings can be updated and changes persist
#[test]
fn tc_g007_003b_settings_update_persistence() {
    let service = create_temp_service();

    // Write initial settings
    let initial = Settings {
        theme: Theme::Light,
        auto_save: true,
        auto_save_interval: 30000,
        focus_mode: false,
        typewriter_mode: false,
        outline_visible: false,
        font_family: "System".to_string(),
        font_size: 16,
        line_height: 1.6,
        tab_size: 4,
        content_width: 720,
        recent_files: vec![],
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
        tab_size: 8,
        content_width: 1000,
        recent_files: vec!["updated.md".to_string()],
    };
    service.write_settings(&updated).unwrap();

    // Verify updates persisted
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
    assert_eq!(loaded.tab_size, 8);
    assert_eq!(loaded.content_width, 1000);
    assert_eq!(loaded.recent_files, vec!["updated.md".to_string()]);
}
