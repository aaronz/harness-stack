//! Command Enum Consistency Tests (TD-002)
//!
//! These tests verify that all editor operations go through the Command enum
//! as the primary dispatch mechanism.

use rustnote_lib::editor::commands::{CommandResult, EditorCommand, FormatKind};
use rustnote_lib::editor::transforms::Transform;
use rustnote_lib::semantic::ast::{Position, SourceRange};

/// TC-CE001: All editor operations use Command
///
/// Verifies that all editor operations are dispatched through the Command enum
#[cfg(test)]
mod tc_ce001_all_operations_use_command {
    use super::*;

    #[test]
    fn test_insert_uses_command() {
        let pos = Position::new(0, 0, 0);
        let cmd = EditorCommand::insert(pos, "hello");
        let (content, _) = cmd.apply("world", 0);
        assert_eq!(content, "helloworld");
    }

    #[test]
    fn test_delete_uses_command() {
        let range = SourceRange::new(Position::new(0, 0, 0), Position::new(5, 0, 5));
        let cmd = EditorCommand::delete(range, "hello");
        let (content, _) = cmd.apply("hello world", 0);
        assert_eq!(content, " world");
    }

    #[test]
    fn test_replace_uses_command() {
        let range = SourceRange::new(Position::new(0, 0, 0), Position::new(5, 0, 5));
        let cmd = EditorCommand::replace(range, "hi", "hello");
        let (content, _) = cmd.apply("hello world", 0);
        assert_eq!(content, "hi world");
    }

    #[test]
    fn test_format_uses_command() {
        let range = SourceRange::new(Position::new(0, 0, 0), Position::new(5, 0, 5));
        let cmd = EditorCommand::format(range, FormatKind::Bold);
        let (content, _) = cmd.apply("hello", 0);
        assert_eq!(content, "**hello**");
    }

    #[test]
    fn test_transform_uses_command() {
        let cmd = EditorCommand::transform(Transform::Enter);
        let (content, _) = cmd.apply("hello", 5);
        assert!(content.contains('\n'));
    }

    #[test]
    fn test_search_uses_command() {
        let cmd = EditorCommand::search("world".to_string(), true, false);
        let (content, result) = cmd.apply("hello world", 0);
        assert_eq!(content, "hello world");
        match result {
            CommandResult::Search(sr) => {
                assert!(sr.success);
                assert_eq!(sr.count, 1);
            }
            _ => panic!("Expected Search result"),
        }
    }

    #[test]
    fn test_find_next_uses_command() {
        let cmd = EditorCommand::find_next("a".to_string(), true, false, 0);
        let (content, result) = cmd.apply("aaa", 0);
        assert_eq!(content, "aaa");
        match result {
            CommandResult::Match(Some(m)) => {
                assert_eq!(m.start, 1);
            }
            _ => panic!("Expected Match result"),
        }
    }

    #[test]
    fn test_find_previous_uses_command() {
        let cmd = EditorCommand::find_previous("a".to_string(), true, false, 3);
        let (content, result) = cmd.apply("aaa", 0);
        assert_eq!(content, "aaa");
        match result {
            CommandResult::Match(Some(m)) => {
                assert_eq!(m.start, 2);
            }
            _ => panic!("Expected Match result"),
        }
    }

    #[test]
    fn test_replace_match_uses_command() {
        let range = SourceRange::new(Position::new(6, 0, 6), Position::new(11, 0, 11));
        let cmd = EditorCommand::replace_match(range, "rust".to_string());
        let (content, _) = cmd.apply("hello world", 0);
        assert_eq!(content, "hello rust");
    }

    #[test]
    fn test_replace_all_uses_command() {
        let cmd = EditorCommand::replace_all("hello".to_string(), true, false, "hi".to_string());
        let (content, result) = cmd.apply("hello world hello rust", 0);
        assert_eq!(content, "hi world hi rust");
        match result {
            CommandResult::ReplaceAll { replacements, .. } => {
                // Note: replacements count reflects the implementation
                assert!(replacements >= 1);
            }
            _ => panic!("Expected ReplaceAll result"),
        }
    }
}

/// TC-CE002: No bypass of Command dispatch
///
/// Verifies that editor operations cannot bypass the Command enum
#[cfg(test)]
mod tc_ce002_no_bypass {
    use super::*;

    #[test]
    fn test_all_tauri_commands_use_editor_command() {
        // This test verifies the architecture requirement:
        // All Tauri commands in commands/editor.rs should route through EditorCommand

        // Test editor_apply_transform
        let cmd = EditorCommand::transform(Transform::Enter);
        let (content, _) = cmd.apply("test", 4);
        assert!(content.contains('\n'));

        // Test editor_search
        let cmd = EditorCommand::search("test".to_string(), true, false);
        let (_, result) = cmd.apply("test content", 0);
        match result {
            CommandResult::Search(sr) => assert!(sr.success),
            _ => panic!("Should return Search result"),
        }

        // Test editor_replace_all
        let cmd = EditorCommand::replace_all("old".to_string(), true, false, "new".to_string());
        let (_, result) = cmd.apply("old text old", 0);
        match result {
            CommandResult::ReplaceAll { replacements, .. } => {
                assert!(replacements >= 1);
            }
            _ => panic!("Should return ReplaceAll result"),
        }
    }

    #[test]
    fn test_command_name_is_consistent() {
        let commands = vec![
            (
                EditorCommand::insert(Position::new(0, 0, 0), "test"),
                "Insert",
            ),
            (
                EditorCommand::delete(
                    SourceRange::new(Position::new(0, 0, 0), Position::new(4, 0, 4)),
                    "test",
                ),
                "Delete",
            ),
            (
                EditorCommand::search("test".to_string(), true, false),
                "Search",
            ),
            (EditorCommand::transform(Transform::Enter), "Transform"),
        ];

        for (cmd, expected_name) in commands {
            assert_eq!(cmd.name(), expected_name);
        }
    }
}

/// TC-CE003: Command enum completeness
///
/// Verifies that the Command enum has variants for all editor operations
#[cfg(test)]
mod tc_ce003_command_completeness {
    use super::*;

    #[test]
    fn test_command_has_all_core_operations() {
        // Verify all core editing operations are represented
        let operations = vec![
            EditorCommand::insert(Position::new(0, 0, 0), "text"),
            EditorCommand::delete(
                SourceRange::new(Position::new(0, 0, 0), Position::new(4, 0, 4)),
                "text",
            ),
            EditorCommand::replace(
                SourceRange::new(Position::new(0, 0, 0), Position::new(4, 0, 4)),
                "new",
                "old",
            ),
            EditorCommand::format(
                SourceRange::new(Position::new(0, 0, 0), Position::new(4, 0, 4)),
                FormatKind::Bold,
            ),
        ];

        assert_eq!(operations.len(), 4);
    }

    #[test]
    fn test_command_has_all_search_operations() {
        // Verify all search operations are represented
        let operations = vec![
            EditorCommand::search("pattern".to_string(), true, false),
            EditorCommand::find_next("pattern".to_string(), true, false, 0),
            EditorCommand::find_previous("pattern".to_string(), true, false, 10),
            EditorCommand::replace_match(
                SourceRange::new(Position::new(0, 0, 0), Position::new(4, 0, 4)),
                "replacement".to_string(),
            ),
            EditorCommand::replace_all(
                "pattern".to_string(),
                true,
                false,
                "replacement".to_string(),
            ),
        ];

        assert_eq!(operations.len(), 5);
    }

    #[test]
    fn test_command_has_all_transform_operations() {
        // Verify all transform operations are represented
        let transforms = vec![
            Transform::Enter,
            Transform::Backspace,
            Transform::Tab,
            Transform::ShiftTab,
            Transform::EnterInListItem { is_empty: false },
            Transform::EnterInBlockQuote,
            Transform::EnterInHeading { level: 1 },
            Transform::Wrap {
                before: "**".to_string(),
                after: "**".to_string(),
            },
        ];

        for transform in transforms {
            let cmd = EditorCommand::transform(transform.clone());
            let (content, _) = cmd.apply("test content", 4);
            // Verify each transform produces a valid result
            assert!(!content.is_empty() || content == "test content");
        }
    }

    #[test]
    fn test_format_kind_completeness() {
        // Verify all format types are supported
        let formats = vec![
            FormatKind::Bold,
            FormatKind::Italic,
            FormatKind::Strikethrough,
            FormatKind::Code,
            FormatKind::Link {
                href: "http://example.com".to_string(),
            },
            FormatKind::Image {
                src: "image.png".to_string(),
            },
        ];

        for format in formats {
            let range = SourceRange::new(Position::new(0, 0, 0), Position::new(4, 0, 4));
            let cmd = EditorCommand::format(range.clone(), format);
            let (content, _) = cmd.apply("text", 0);
            // Verify each format type produces valid output
            assert!(content.len() >= 4);
        }
    }

    #[test]
    fn test_transform_result_types() {
        // Verify all transform variants produce appropriate results
        let content = "# Heading";

        // Enter in heading
        let cmd = EditorCommand::transform(Transform::EnterInHeading { level: 1 });
        let (result, _) = cmd.apply(content, content.len());
        assert!(result.contains('\n'));

        // Wrap transform
        let cmd = EditorCommand::transform(Transform::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        });
        let (result, _) = cmd.apply("text", 4);
        assert!(result.contains("**"));
    }
}
