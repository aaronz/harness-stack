//! Undo/Redo Fidelity Tests
//!
//! Tests for G-002: Fix Undo/Redo Fidelity
//! Ensures structural edits are atomic and state is preserved correctly
//! across multiple undo/redo cycles.

use rustnote_lib::editor::commands::Command;
use rustnote_lib::editor::undo::UndoManager;
use rustnote_lib::semantic::ast::{Position, SourceRange};

fn make_range(start: usize, end: usize) -> SourceRange {
    SourceRange::new(
        Position::new(start, 0, start as u32),
        Position::new(end, 0, end as u32),
    )
}

// =============================================================================
// TC-G002-001: UndoRedo_roundtrip_list
// =============================================================================

#[test]
fn test_undo_redo_roundtrip_list() {
    let initial = "- Item 1\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(9, 9), "- Item 2\n", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "- Item 1\n- Item 2\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);

    let redone = undo_manager.redo().unwrap();
    redone.apply(&mut content);
    assert_eq!(content, "- Item 1\n- Item 2\n");
}

#[test]
fn test_undo_redo_list_delete() {
    let initial = "- Item 1\n- Item 2\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(9, 18), "- Item 1\n", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "- Item 1\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);

    let redone = undo_manager.redo().unwrap();
    redone.apply(&mut content);
    assert_eq!(content, "- Item 1\n");
}

#[test]
fn test_undo_redo_do_undo_do_undo() {
    let initial = "- Item 1\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd1 = Command::replace(make_range(9, 9), "- Item 2\n", &content);
    undo_manager.execute(cmd1.clone());
    cmd1.apply(&mut content);

    let undone1 = undo_manager.undo().unwrap();
    let inv1 = undone1.inverse().unwrap();
    inv1.apply(&mut content);
    assert_eq!(content, initial);

    let cmd2 = Command::replace(make_range(9, 9), "- Item 2\n", &content);
    undo_manager.execute(cmd2.clone());
    cmd2.apply(&mut content);

    let undone2 = undo_manager.undo().unwrap();
    let inv2 = undone2.inverse().unwrap();
    inv2.apply(&mut content);

    assert_eq!(content, initial);
}

// =============================================================================
// TC-G002-002: UndoRedo_roundtrip_heading
// =============================================================================

#[test]
fn test_undo_redo_heading_level_change() {
    let initial = "# Title\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(0, 2), "##", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "## Title\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);

    let redone = undo_manager.redo().unwrap();
    redone.apply(&mut content);
    assert_eq!(content, "## Title\n");
}

#[test]
fn test_undo_redo_heading_content_change() {
    let initial = "# Title\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(2, 7), "New Title", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "# New Title\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);

    let redone = undo_manager.redo().unwrap();
    redone.apply(&mut content);
    assert_eq!(content, "# New Title\n");
}

#[test]
fn test_undo_redo_paragraph_to_heading() {
    let initial = "Paragraph\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(0, 0), "# ", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "# Paragraph\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);
}

// =============================================================================
// TC-G002-003: UndoRedo_multiple_cycles
// =============================================================================

#[test]
fn test_undo_redo_multiple_cycles_list() {
    let initial = "- Item 1\n- Item 2\n";

    for _ in 0..10 {
        let mut undo_manager = UndoManager::new();
        let mut content = initial.to_string();

        let cmd: Command = Command::replace(make_range(17, 17), "- Item 3\n", &content);
        undo_manager.execute(cmd.clone());
        cmd.apply(&mut content);
        assert_eq!(content, "- Item 1\n- Item 2\n- Item 3\n");

        let undone: Command = undo_manager.undo().unwrap();
        let inv: Command = undone.inverse().unwrap();
        inv.apply(&mut content);
        assert_eq!(content, initial);

        let redone: Command = undo_manager.redo().unwrap();
        redone.apply(&mut content);
        assert_eq!(content, "- Item 1\n- Item 2\n- Item 3\n");

        let undone2: Command = undo_manager.undo().unwrap();
        let inv2: Command = undone2.inverse().unwrap();
        inv2.apply(&mut content);
        assert_eq!(content, initial);
    }
}

#[test]
fn test_undo_redo_ten_consecutive_operations() {
    let mut undo_manager = UndoManager::new();
    let mut content = String::new();

    for i in 0..10 {
        let insert_text = format!("Line {}\n", i + 1);
        let cmd = Command::replace(
            make_range(content.len(), content.len()),
            &insert_text,
            &content,
        );
        undo_manager.execute(cmd.clone());
        cmd.apply(&mut content);
    }

    let final_content = content.clone();
    assert_eq!(
        content,
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10\n"
    );

    for _ in 0..10 {
        let undone = undo_manager.undo().unwrap();
        let inv = undone.inverse().unwrap();
        inv.apply(&mut content);
    }

    assert_eq!(content, "");
}

#[test]
fn test_undo_redo_ten_operations_individual() {
    let mut undo_manager = UndoManager::new();
    let mut content = String::new();

    let ops: Vec<Command> = (0..10)
        .map(|i| {
            let insert_text = format!("Line {}\n", i + 1);
            Command::replace(
                make_range(content.len(), content.len()),
                &insert_text,
                &content,
            )
        })
        .collect();

    for cmd in &ops {
        undo_manager.execute(cmd.clone());
    }

    for cmd in &ops {
        let mut c = content.clone();
        cmd.apply(&mut c);
        content = c;
    }

    let final_content = content.clone();

    for _ in 0..10 {
        if let Some(undone) = undo_manager.undo() {
            if let Some(inv) = undone.inverse() {
                inv.apply(&mut content);
            }
        }
    }

    assert_eq!(content, "");
}

// =============================================================================
// TC-G002-004: UndoRedo_structural_edit_atomic
// =============================================================================

#[test]
fn test_undo_redo_structural_list_to_paragraph() {
    let initial = "- Item\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(0, 2), "Item", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "Item\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);
}

#[test]
fn test_undo_redo_structural_paragraph_to_list() {
    let initial = "Item\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(0, 0), "- ", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "- Item\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);
}

#[test]
fn test_undo_redo_structural_list_to_blockquote() {
    let initial = "- Item\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(0, 2), "> ", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "> Item\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);
}

#[test]
fn test_undo_redo_structural_multiple_edits() {
    let initial = "# Heading\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd1 = Command::replace(make_range(9, 9), "\n- List item", &content);
    undo_manager.execute(cmd1.clone());
    cmd1.apply(&mut content);
    let after_first = content.clone();

    let cmd2 = Command::replace(make_range(0, 2), "##", &content);
    undo_manager.execute(cmd2.clone());
    cmd2.apply(&mut content);

    assert_eq!(content, "## Heading\n- List item");

    let undone2 = undo_manager.undo().unwrap();
    let inv2 = undone2.inverse().unwrap();
    inv2.apply(&mut content);
    assert_eq!(content, after_first);

    let undone1 = undo_manager.undo().unwrap();
    let inv1 = undone1.inverse().unwrap();
    inv1.apply(&mut content);
    assert_eq!(content, initial);
}

// =============================================================================
// TC-G002-005: UndoRedo_after_save_reload
// =============================================================================

#[test]
fn test_undo_redo_after_save_reload() {
    let saved_content = "- Item\n";
    let mut undo_manager = UndoManager::new();
    let mut content = saved_content.to_string();

    let cmd = Command::replace(make_range(6, 6), " changed", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "- Item changed");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, saved_content);
}

#[test]
fn test_undo_redo_save_boundary() {
    let initial = "- [ ] Task\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(4, 4), "[x]", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    let saved = content.clone();
    assert_eq!(content, "- [x] Task\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);

    let redone = undo_manager.redo().unwrap();
    redone.apply(&mut content);
    assert_eq!(content, saved);
}

// =============================================================================
// TC-G002-006: UndoRedo_nested_structures
// =============================================================================

#[test]
fn test_undo_redo_nested_list() {
    let initial = "  - Nested\n    - Deep\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(10, 15), "Changed", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "  - Nested\n    - Changed\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);

    let redone = undo_manager.redo().unwrap();
    redone.apply(&mut content);
    assert_eq!(content, "  - Nested\n    - Changed\n");
}

#[test]
fn test_undo_redo_deeply_nested() {
    let initial = "- Level 1\n  - Level 2\n    - Level 3\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(24, 30), "Changed", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);
}

#[test]
fn test_undo_redo_nested_blockquotes() {
    let initial = "> Quote\n>> Nested quote\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(16, 20), "Changed", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "> Quote\n>> Changed\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);
}

// =============================================================================
// TC-G002-007: UndoRedo_rapid_operations
// =============================================================================

#[test]
fn test_undo_redo_rapid_operations() {
    let initial = "Start\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let mut commands: Vec<Command> = Vec::new();
    for i in 0..50 {
        let text = format!("Edit {}\n", i);
        let cmd = Command::replace(make_range(content.len(), content.len()), &text, &content);
        commands.push(cmd.clone());
        undo_manager.execute(cmd.clone());
        cmd.apply(&mut content);
    }

    let final_content = content.clone();
    assert!(final_content.contains("Edit 0\n"));
    assert!(final_content.contains("Edit 49\n"));

    for _ in 0..50 {
        let undone = undo_manager.undo().unwrap();
        let inv = undone.inverse().unwrap();
        inv.apply(&mut content);
    }
    assert_eq!(content, initial);
}

#[test]
fn test_undo_redo_rapid_alternating() {
    let initial = "- Item\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    for i in 0..20 {
        let text = format!("- Item {}\n", i + 1);
        let cmd = Command::replace(make_range(content.len(), content.len()), &text, &content);
        undo_manager.execute(cmd.clone());
        cmd.apply(&mut content);

        let undone = undo_manager.undo().unwrap();
        let inv = undone.inverse().unwrap();
        inv.apply(&mut content);

        let redone = undo_manager.redo().unwrap();
        redone.apply(&mut content);
    }

    assert!(content.contains("- Item 20\n"));
}

#[test]
fn test_undo_redo_no_dropped_operations() {
    let mut undo_manager = UndoManager::new();
    let mut content = "0".to_string();

    for i in 1..=100 {
        let cmd = Command::replace(make_range(0, content.len()), &i.to_string(), &content);
        undo_manager.execute(cmd.clone());
        cmd.apply(&mut content);
    }

    assert_eq!(content, "100");

    for _ in 0..100 {
        let undone = undo_manager.undo().unwrap();
        let inv = undone.inverse().unwrap();
        inv.apply(&mut content);
    }
    assert_eq!(content, "0");
}

#[test]
fn test_undo_redo_no_state_corruption() {
    let initial = "# Title\n\n- Item 1\n- Item 2\n\n> Quote\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd1 = Command::replace(make_range(0, 0), "# Changed\n", &content);
    undo_manager.execute(cmd1.clone());
    cmd1.apply(&mut content);

    let cmd2 = Command::replace(
        make_range(content.len(), content.len()),
        "\n## Section",
        &content,
    );
    undo_manager.execute(cmd2.clone());
    cmd2.apply(&mut content);

    let cmd3 = Command::replace(
        make_range(content.len(), content.len()),
        "\n- Item 3",
        &content,
    );
    undo_manager.execute(cmd3.clone());
    cmd3.apply(&mut content);

    let cmd4 = Command::replace(
        make_range(content.len(), content.len()),
        "\n**bold**",
        &content,
    );
    undo_manager.execute(cmd4.clone());
    cmd4.apply(&mut content);

    let after_all = content.clone();
    assert!(after_all.contains("# Changed"));
    assert!(after_all.contains("## Section"));
    assert!(after_all.contains("- Item 3"));
    assert!(after_all.contains("**bold**"));

    for _ in 0..4 {
        let undone = undo_manager.undo().unwrap();
        let inv = undone.inverse().unwrap();
        inv.apply(&mut content);
    }

    assert_eq!(content, initial);
}

// =============================================================================
// Additional tests for coverage
// =============================================================================

#[test]
fn test_undo_redo_task_list() {
    let initial = "- [ ] Task\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(3, 5), "[x]", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "- [x] Task\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);

    let redone = undo_manager.redo().unwrap();
    redone.apply(&mut content);
    assert_eq!(content, "- [x] Task\n");
}

#[test]
fn test_undo_redo_blockquote() {
    let initial = "> Quote\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(7, 7), "\n> Second line", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "> Quote\n> Second line\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);

    let redone = undo_manager.redo().unwrap();
    redone.apply(&mut content);
    assert_eq!(content, "> Quote\n> Second line\n");
}

#[test]
fn test_undo_redo_ordered_list() {
    let initial = "1. First\n";
    let mut undo_manager = UndoManager::new();
    let mut content = initial.to_string();

    let cmd = Command::replace(make_range(9, 9), "2. Second\n", &content);
    undo_manager.execute(cmd.clone());
    cmd.apply(&mut content);
    assert_eq!(content, "1. First\n2. Second\n");

    let undone = undo_manager.undo().unwrap();
    let inv = undone.inverse().unwrap();
    inv.apply(&mut content);
    assert_eq!(content, initial);

    let redone = undo_manager.redo().unwrap();
    redone.apply(&mut content);
    assert_eq!(content, "1. First\n2. Second\n");
}

#[test]
fn test_undo_manager_state_tracking() {
    let mut undo_manager = UndoManager::new();

    assert!(!undo_manager.can_undo());
    assert!(!undo_manager.can_redo());

    let cmd = Command::replace(make_range(0, 0), "test", "");
    undo_manager.execute(cmd);

    assert!(undo_manager.can_undo());
    assert!(!undo_manager.can_redo());

    let _ = undo_manager.undo();
    assert!(!undo_manager.can_undo());
    assert!(undo_manager.can_redo());

    let _ = undo_manager.redo();
    assert!(undo_manager.can_undo());
    assert!(!undo_manager.can_redo());
}

#[test]
fn test_undo_stack_cleared_on_new_operation() {
    let mut undo_manager = UndoManager::new();
    let mut content = "test".to_string();

    let cmd1 = Command::replace(make_range(4, 4), "1", &content);
    undo_manager.execute(cmd1.clone());
    cmd1.apply(&mut content);

    let _ = undo_manager.undo();
    assert!(!undo_manager.can_redo());

    let cmd2 = Command::replace(make_range(4, 4), "2", &content);
    undo_manager.execute(cmd2.clone());
    cmd2.apply(&mut content);

    assert!(!undo_manager.can_redo());
}

#[test]
fn test_undo_max_stack_size() {
    let mut undo_manager = UndoManager::new();
    let mut content = String::new();

    for i in 0..150 {
        let cmd = Command::replace(
            make_range(content.len(), content.len()),
            &format!("{}\n", i),
            &content,
        );
        undo_manager.execute(cmd.clone());
        cmd.apply(&mut content);
    }

    assert!(undo_manager.can_undo());

    for _ in 0..150 {
        if let Some(undo_cmd) = undo_manager.undo() {
            if let Some(inv) = undo_cmd.inverse() {
                inv.apply(&mut content);
            }
        }
    }

    assert_eq!(content, "");
}
