//! Visual Regression Tests for RustNote
//!
//! Tests for TD-004: Visual Regression Baselines
//!
//! Test Cases:
//! - TC-VR001: Empty editor baseline
//! - TC-VR002: Focused editor with content baseline
//! - TC-VR003: Dark theme baseline
//! - TC-VR004: Light theme baseline
//! - TC-VR005: Focus mode baseline
//! - TC-VR006: Typewriter mode baseline
//! - TC-VR007: Export modal baseline
//! - TC-VR008: Visual regression CI check

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    const TEST_BASELINE_DIR: &str = "./visual-baselines";
    const TEST_SCREENSHOTS_DIR: &str = "./visual-screenshots";

    fn ensure_directories_exist() {
        std::fs::create_dir_all(TEST_BASELINE_DIR).ok();
        std::fs::create_dir_all(TEST_SCREENSHOTS_DIR).ok();
    }

    fn get_baseline_path(test_name: &str) -> PathBuf {
        PathBuf::from(TEST_BASELINE_DIR).join(format!("{}.png", test_name))
    }

    fn get_screenshot_path(test_name: &str, suffix: &str) -> PathBuf {
        PathBuf::from(TEST_SCREENSHOTS_DIR).join(format!("{}-{}.png", test_name, suffix))
    }

    /// TC-VR001: Empty editor baseline
    /// Category: render
    /// Input: Fresh editor, no content
    /// Expected: Baseline captured for empty editor state
    #[test]
    fn test_vr001_empty_editor_baseline_exists() {
        ensure_directories_exist();

        let baseline_path = get_baseline_path("vr001-empty-editor");

        // In CI, baseline must exist
        if std::env::var("CI").is_ok() {
            assert!(
                baseline_path.exists(),
                "Baseline for empty editor must exist in CI"
            );
        } else {
            // In dev, baseline may not exist yet - it's created by Playwright tests
            // Just verify the path is correctly constructed
            assert!(baseline_path.parent().is_some());
        }
    }

    #[test]
    fn test_vr001_baseline_directory_structure() {
        ensure_directories_exist();

        let baseline_dir = PathBuf::from(TEST_BASELINE_DIR);
        assert!(baseline_dir.exists() || std::fs::create_dir_all(&baseline_dir).is_ok());

        let screenshots_dir = PathBuf::from(TEST_SCREENSHOTS_DIR);
        assert!(screenshots_dir.exists() || std::fs::create_dir_all(&screenshots_dir).is_ok());
    }

    /// TC-VR002: Focused editor with content baseline
    /// Category: render
    /// Input: Editor with sample Markdown content
    /// Expected: Baseline captured for content rendering
    #[test]
    fn test_vr002_content_baseline_paths() {
        let expected_baselines = vec![
            "vr002-editor-with-content",
            "vr002-editor-with-task-list",
            "vr002-editor-with-nested-lists",
        ];

        for name in expected_baselines {
            let path = get_baseline_path(name);
            if std::env::var("CI").is_ok() {
                assert!(path.exists(), "Baseline {} must exist in CI", name);
            }
        }
    }

    /// TC-VR003: Dark theme baseline
    /// Category: render
    /// Input: Dark theme enabled
    /// Expected: Baseline captured for dark theme rendering
    #[test]
    fn test_vr003_dark_theme_baseline_paths() {
        let expected_baselines = vec![
            "vr003-dark-theme-empty",
            "vr003-dark-theme-with-content",
            "vr003-dark-theme-code",
        ];

        for name in expected_baselines {
            let path = get_baseline_path(name);
            if std::env::var("CI").is_ok() {
                assert!(
                    path.exists(),
                    "Dark theme baseline {} must exist in CI",
                    name
                );
            }
        }
    }

    /// TC-VR004: Light theme baseline
    /// Category: render
    /// Input: Light theme enabled
    /// Expected: Baseline captured for light theme rendering
    #[test]
    fn test_vr004_light_theme_baseline_paths() {
        let expected_baselines = vec![
            "vr004-light-theme-empty",
            "vr004-light-theme-with-content",
            "vr004-light-theme-sidebar",
        ];

        for name in expected_baselines {
            let path = get_baseline_path(name);
            if std::env::var("CI").is_ok() {
                assert!(
                    path.exists(),
                    "Light theme baseline {} must exist in CI",
                    name
                );
            }
        }
    }

    /// TC-VR005: Focus mode baseline
    /// Category: render
    /// Input: Focus mode enabled
    /// Expected: Baseline captured for focus mode rendering
    #[test]
    fn test_vr005_focus_mode_baseline_paths() {
        let expected_baselines = vec![
            "vr005-focus-mode-content",
            "vr005-focus-mode-second-paragraph",
            "vr005-focus-mode-full-page",
        ];

        for name in expected_baselines {
            let path = get_baseline_path(name);
            if std::env::var("CI").is_ok() {
                assert!(
                    path.exists(),
                    "Focus mode baseline {} must exist in CI",
                    name
                );
            }
        }
    }

    /// TC-VR006: Typewriter mode baseline
    /// Category: render
    /// Input: Typewriter mode enabled
    /// Expected: Baseline captured for typewriter mode rendering
    #[test]
    fn test_vr006_typewriter_mode_baseline_paths() {
        let expected_baselines = vec![
            "vr006-typewriter-mode-content",
            "vr006-typewriter-mode-scroll",
            "vr006-typewriter-mode-full-page",
        ];

        for name in expected_baselines {
            let path = get_baseline_path(name);
            if std::env::var("CI").is_ok() {
                assert!(
                    path.exists(),
                    "Typewriter mode baseline {} must exist in CI",
                    name
                );
            }
        }
    }

    /// TC-VR007: Export modal baseline
    /// Category: render
    /// Input: Export modal open
    /// Expected: Baseline captured for export modal rendering
    #[test]
    fn test_vr007_export_modal_baseline_paths() {
        let expected_baselines = vec![
            "vr007-export-modal",
            "vr007-export-modal-pdf-selected",
            "vr007-export-modal-dark",
        ];

        for name in expected_baselines {
            let path = get_baseline_path(name);
            if std::env::var("CI").is_ok() {
                assert!(
                    path.exists(),
                    "Export modal baseline {} must exist in CI",
                    name
                );
            }
        }
    }

    /// TC-VR008: Visual regression CI check
    /// Category: integration
    /// Input: Run visual regression CI
    /// Expected: Passes when no visual changes detected
    #[test]
    fn test_vr008_all_required_baselines_exist() {
        let required_baselines = vec![
            // TC-VR001
            "vr001-empty-editor",
            "vr001-empty-editor-with-toolbar",
            // TC-VR002
            "vr002-editor-with-content",
            "vr002-editor-with-task-list",
            "vr002-editor-with-nested-lists",
            // TC-VR003
            "vr003-dark-theme-empty",
            "vr003-dark-theme-with-content",
            "vr003-dark-theme-code",
            // TC-VR004
            "vr004-light-theme-empty",
            "vr004-light-theme-with-content",
            "vr004-light-theme-sidebar",
            // TC-VR005
            "vr005-focus-mode-content",
            "vr005-focus-mode-second-paragraph",
            "vr005-focus-mode-full-page",
            // TC-VR006
            "vr006-typewriter-mode-content",
            "vr006-typewriter-mode-scroll",
            "vr006-typewriter-mode-full-page",
            // TC-VR007
            "vr007-export-modal",
            "vr007-export-modal-pdf-selected",
            "vr007-export-modal-dark",
            // Additional baselines
            "vr008-ci-check-test",
            "vr008-syntax-bold",
            "vr008-syntax-italic",
            "vr008-syntax-strikethrough",
            "vr008-syntax-inline-code",
            "vr008-syntax-link",
            "vr008-syntax-image",
            "vr008-syntax-horizontal-rule",
        ];

        let baseline_dir = PathBuf::from(TEST_BASELINE_DIR);

        // Create baseline directory if it doesn't exist
        if !baseline_dir.exists() {
            std::fs::create_dir_all(&baseline_dir).ok();
        }

        let missing: Vec<_> = required_baselines
            .iter()
            .filter(|name| !get_baseline_path(name).exists())
            .collect();

        if std::env::var("CI").is_ok() {
            assert_eq!(
                missing.len(),
                0,
                "All {} required baselines must exist in CI. Missing: {:?}",
                required_baselines.len(),
                missing
            );
        } else {
            // In development, missing baselines will be created by Playwright
            println!(
                "Baseline status: {}/{} exist",
                required_baselines.len() - missing.len(),
                required_baselines.len()
            );
            if !missing.is_empty() {
                println!(
                    "Missing baselines (will be created on first Playwright run): {:?}",
                    missing
                );
            }
        }
    }

    #[test]
    fn test_vr008_baseline_file_format() {
        let baseline_dir = PathBuf::from(TEST_BASELINE_DIR);

        if !baseline_dir.exists() {
            return;
        }

        let entries = std::fs::read_dir(&baseline_dir)
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "png"));

        for entry in entries {
            let path = entry.path();
            if let Ok(metadata) = std::fs::metadata(&path) {
                // Valid PNG should be at least 100 bytes
                assert!(
                    metadata.len() > 100,
                    "Baseline {} should be a valid PNG (>100 bytes)",
                    path.display()
                );

                // Check PNG magic bytes
                if let Ok(data) = std::fs::read(&path) {
                    let png_magic: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
                    assert!(
                        data.starts_with(&png_magic),
                        "Baseline {} should have valid PNG magic bytes",
                        path.display()
                    );
                }
            }
        }
    }

    #[test]
    fn test_baseline_count_per_category() {
        let baseline_dir = PathBuf::from(TEST_BASELINE_DIR);

        if !baseline_dir.exists() {
            return;
        }

        let files: Vec<_> = std::fs::read_dir(&baseline_dir)
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "png"))
            .collect();

        // Count baselines per category
        let vr001_count = files
            .iter()
            .filter(|f| f.file_name().to_string_lossy().starts_with("vr001"))
            .count();
        let vr002_count = files
            .iter()
            .filter(|f| f.file_name().to_string_lossy().starts_with("vr002"))
            .count();
        let vr003_count = files
            .iter()
            .filter(|f| f.file_name().to_string_lossy().starts_with("vr003"))
            .count();
        let vr004_count = files
            .iter()
            .filter(|f| f.file_name().to_string_lossy().starts_with("vr004"))
            .count();
        let vr005_count = files
            .iter()
            .filter(|f| f.file_name().to_string_lossy().starts_with("vr005"))
            .count();
        let vr006_count = files
            .iter()
            .filter(|f| f.file_name().to_string_lossy().starts_with("vr006"))
            .count();
        let vr007_count = files
            .iter()
            .filter(|f| f.file_name().to_string_lossy().starts_with("vr007"))
            .count();
        let vr008_count = files
            .iter()
            .filter(|f| f.file_name().to_string_lossy().starts_with("vr008"))
            .count();

        println!("Baseline counts:");
        println!("  TC-VR001 (Empty editor): {}", vr001_count);
        println!("  TC-VR002 (Content): {}", vr002_count);
        println!("  TC-VR003 (Dark theme): {}", vr003_count);
        println!("  TC-VR004 (Light theme): {}", vr004_count);
        println!("  TC-VR005 (Focus mode): {}", vr005_count);
        println!("  TC-VR006 (Typewriter): {}", vr006_count);
        println!("  TC-VR007 (Export modal): {}", vr007_count);
        println!("  TC-VR008 (CI check): {}", vr008_count);
        println!("  Total baselines: {}", files.len());

        // Minimum expected per category (can be 0 in dev before first Playwright run)
        if std::env::var("CI").is_ok() {
            assert!(
                vr001_count >= 2,
                "TC-VR001 should have at least 2 baselines"
            );
            assert!(
                vr002_count >= 3,
                "TC-VR002 should have at least 3 baselines"
            );
            assert!(
                vr003_count >= 3,
                "TC-VR003 should have at least 3 baselines"
            );
            assert!(
                vr004_count >= 3,
                "TC-VR004 should have at least 3 baselines"
            );
            assert!(
                vr005_count >= 3,
                "TC-VR005 should have at least 3 baselines"
            );
            assert!(
                vr006_count >= 3,
                "TC-VR006 should have at least 3 baselines"
            );
            assert!(
                vr007_count >= 3,
                "TC-VR007 should have at least 3 baselines"
            );
        }
    }

    /// Test markdown syntax rendering baselines
    #[test]
    fn test_markdown_syntax_baselines() {
        let syntax_tests = vec![
            "vr008-syntax-bold",
            "vr008-syntax-italic",
            "vr008-syntax-strikethrough",
            "vr008-syntax-inline-code",
            "vr008-syntax-link",
            "vr008-syntax-image",
            "vr008-syntax-horizontal-rule",
        ];

        for name in syntax_tests {
            let path = get_baseline_path(name);
            if std::env::var("CI").is_ok() {
                assert!(
                    path.exists(),
                    "Markdown syntax baseline {} must exist in CI",
                    name
                );
            }
        }
    }

    /// Test additional component baselines
    #[test]
    fn test_additional_component_baselines() {
        let additional_baselines = vec![
            "additional-table-rendering",
            "additional-frontmatter-block",
            "additional-search-panel",
            "additional-outline-panel",
            "additional-preferences-modal",
        ];

        for name in additional_baselines {
            let path = get_baseline_path(name);
            // Additional baselines are optional but should be tracked
            if path.exists() {
                println!("Found additional baseline: {}", name);
            }
        }
    }
}
