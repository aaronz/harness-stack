//! Transform rules for smart editing behaviors
//!
//! Implements Enter/Backspace/Tab handling for:
//! - Lists (ordered and unordered)
//! - Blockquotes
//! - Headings
//! - Task lists

use crate::semantic::ast::{Position, SourceRange};

/// Transform types for structural editing
#[derive(Debug, Clone)]
pub enum Transform {
    /// Enter key pressed
    Enter,
    /// Backspace key pressed
    Backspace,
    /// Tab key pressed
    Tab,
    /// Shift+Tab pressed
    ShiftTab,
    /// Enter in list item
    EnterInListItem { is_empty: bool },
    /// Enter in blockquote
    EnterInBlockQuote,
    /// Enter in heading
    EnterInHeading { level: u8 },
    /// Wrap selection with markers
    Wrap { before: String, after: String },
}

/// Transform result containing the new content and cursor position
#[derive(Debug, Clone)]
pub struct TransformResult {
    pub content: String,
    pub cursor_offset: usize,
    pub selection: Option<(usize, usize)>,
}

impl TransformResult {
    pub fn new(content: String, cursor_offset: usize) -> Self {
        Self {
            content,
            cursor_offset,
            selection: None,
        }
    }

    pub fn with_selection(content: String, start: usize, end: usize) -> Self {
        Self {
            content,
            cursor_offset: start,
            selection: Some((start, end)),
        }
    }
}

/// Transform engine for smart editing
pub struct TransformEngine;

impl TransformEngine {
    pub fn new() -> Self {
        Self
    }

    /// Apply a transform to the content at the given cursor position
    pub fn apply(
        &self,
        transform: &Transform,
        content: &str,
        cursor_offset: usize,
    ) -> TransformResult {
        match transform {
            Transform::Enter => self.apply_enter(content, cursor_offset),
            Transform::Backspace => self.apply_backspace(content, cursor_offset),
            Transform::Tab => self.apply_tab(content, cursor_offset),
            Transform::ShiftTab => self.apply_shift_tab(content, cursor_offset),
            Transform::EnterInListItem { is_empty } => {
                self.apply_list_enter(content, cursor_offset, *is_empty)
            }
            Transform::EnterInBlockQuote => self.apply_blockquote_enter(content, cursor_offset),
            Transform::EnterInHeading { level } => {
                self.apply_heading_enter(content, cursor_offset, *level)
            }
            Transform::Wrap { before, after } => {
                self.apply_wrap(content, cursor_offset, before, after)
            }
        }
    }

    fn apply_enter(&self, content: &str, cursor_offset: usize) -> TransformResult {
        let (before, after) = content.split_at(cursor_offset);
        let newline = "\n";

        // Check if we're at the end of a list item
        let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let current_line = &before[line_start..];

        if current_line.trim_start().starts_with("- ")
            || current_line.trim_start().starts_with("* ")
        {
            // Unordered list
            if current_line.trim().eq("-") || current_line.trim().eq("*") {
                // Empty list item - exit the list
                let new_content =
                    format!("{}{}", &content[..line_start], &content[cursor_offset..]);
                return TransformResult::new(new_content, line_start);
            }
            return TransformResult::new(
                format!("{}{}{}", before, newline, after),
                cursor_offset + 1,
            );
        }

        if let Some(stripped) = current_line
            .strip_prefix("- [ ] ")
            .or_else(|| current_line.strip_prefix("- [x] "))
        {
            if stripped.trim().is_empty() {
                let new_content =
                    format!("{}{}", &content[..line_start], &content[cursor_offset..]);
                return TransformResult::new(new_content, line_start);
            }
            return TransformResult::new(
                format!("{}{}- [ ]{}\n", before, newline, stripped),
                before.len() + 1,
            );
        }

        // Check for ordered list
        if Self::is_ordered_list_line(current_line) {
            if current_line.trim().ends_with('.') && current_line.trim().len() <= 4 {
                // Empty ordered list item
                let new_content =
                    format!("{}{}", &content[..line_start], &content[cursor_offset..]);
                return TransformResult::new(new_content, line_start);
            }

            // Find the number and increment
            let num_str = current_line.trim().split('.').next().unwrap_or("1");
            if let Ok(num) = num_str.parse::<u32>() {
                return TransformResult::new(
                    format!("{}{}{}. ", before, newline, num + 1),
                    cursor_offset + 4,
                );
            }
        }

        // Check for blockquote
        if current_line.starts_with("> ") {
            return TransformResult::new(format!("{}{}> ", before, newline), cursor_offset + 3);
        }

        // Check for heading
        if let Some(level) = Self::get_heading_level(current_line) {
            let heading_prefix = "#".repeat(level as usize);
            return TransformResult::new(
                format!("{}{}{}", before, newline, heading_prefix),
                cursor_offset + 1 + level as usize,
            );
        }

        // Default: just insert newline
        TransformResult::new(format!("{}{}{}", before, newline, after), cursor_offset + 1)
    }

    fn apply_backspace(&self, content: &str, cursor_offset: usize) -> TransformResult {
        if cursor_offset == 0 {
            return TransformResult::new(content.to_string(), 0);
        }

        let before = &content[..cursor_offset];
        let after = &content[cursor_offset..];

        let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let current_line = &before[line_start..];

        // Check for list continuation
        if current_line.trim().is_empty() && line_start > 0 {
            // Find previous line
            let prev_line_start = before[..line_start]
                .trim_end_matches(|c| c != '\n')
                .rfind('\n')
                .map(|i| i + 1)
                .unwrap_or(0);
            let prev_line = &content[prev_line_start..line_start];

            if prev_line.trim_start().starts_with("- ") || prev_line.trim_start().starts_with("* ")
            {
                // Join with previous list item
                let new_content = format!("{}{}", &content[..line_start], after);
                return TransformResult::new(
                    new_content,
                    prev_line_start + prev_line.trim_end().len(),
                );
            }
        }

        // Default: remove character before cursor
        TransformResult::new(
            format!("{}{}", &content[..cursor_offset - 1], after),
            cursor_offset - 1,
        )
    }

    fn apply_tab(&self, content: &str, cursor_offset: usize) -> TransformResult {
        let (before, after) = content.split_at(cursor_offset);
        let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let current_line = &before[line_start..];

        // Indent list item
        if current_line.trim_start().starts_with("- ")
            || current_line.trim_start().starts_with("* ")
        {
            let indent = "    ";
            let new_content = format!(
                "{}{}{}{}",
                &content[..line_start],
                indent,
                current_line.trim_start(),
                after
            );
            return TransformResult::new(new_content, cursor_offset + 4);
        }

        // Default: insert tab
        TransformResult::new(format!("{}\t{}{}", before, "", after), cursor_offset + 1)
    }

    fn apply_shift_tab(&self, content: &str, cursor_offset: usize) -> TransformResult {
        let (before, after) = content.split_at(cursor_offset);
        let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let current_line = &before[line_start..];

        if let Some(stripped) = current_line.strip_prefix("    ") {
            let new_content = format!("{}{}{}", &content[..line_start], stripped, after);
            return TransformResult::new(new_content, cursor_offset - 4);
        }
        if let Some(stripped) = current_line.strip_prefix("\t") {
            let new_content = format!("{}{}{}", &content[..line_start], stripped, after);
            return TransformResult::new(new_content, cursor_offset - 1);
        }

        TransformResult::new(content.to_string(), cursor_offset)
    }

    fn apply_list_enter(
        &self,
        content: &str,
        cursor_offset: usize,
        is_empty: bool,
    ) -> TransformResult {
        let before = &content[..cursor_offset];
        let after = &content[cursor_offset..];

        let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let current_line = &before[line_start..];

        let indent = current_line.len() - current_line.trim_start().len();
        let indent_str = &current_line[..indent];

        let marker_str: String = if current_line.trim_start().starts_with("- [ ] ")
            || current_line.trim_start().starts_with("- [x] ")
            || current_line.trim_start().starts_with("- [X] ")
        {
            "- [ ] ".to_string()
        } else if current_line.trim_start().starts_with("- ")
            || current_line.trim_start().starts_with("* ")
        {
            "- ".to_string()
        } else if Self::is_ordered_list_line(current_line) {
            let num = current_line.trim_start().split('.').next().unwrap_or("1");
            format!("{}. ", num)
        } else {
            "- ".to_string()
        };

        if is_empty {
            let new_content = format!("{}{}\n{}", &content[..line_start], indent_str, after);
            return TransformResult::new(new_content, line_start + indent_str.len() + 1);
        }

        let new_content = format!(
            "{}{}\n{}{}",
            before,
            marker_str.trim_end(),
            indent_str,
            marker_str
        );
        let cursor_pos = new_content.len();
        TransformResult::new(new_content, cursor_pos)
    }

    fn apply_blockquote_enter(&self, content: &str, cursor_offset: usize) -> TransformResult {
        let before = &content[..cursor_offset];
        let after = &content[cursor_offset..];

        let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let current_line = &before[line_start..];

        let line_content = current_line.trim_start();
        let is_empty_quote = line_content == ">"
            || line_content.starts_with("> ") && line_content[2..].trim().is_empty();

        if is_empty_quote {
            let new_content = format!("{}{}", &content[..line_start], after);
            return TransformResult::new(new_content, line_start);
        }

        let new_content = format!("{}{}> ", before, "\n");
        let cursor_pos = new_content.len();
        TransformResult::new(new_content, cursor_pos)
    }

    fn apply_heading_enter(
        &self,
        content: &str,
        cursor_offset: usize,
        level: u8,
    ) -> TransformResult {
        let before = &content[..cursor_offset];

        let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let current_line = &before[line_start..];

        let trimmed = current_line.trim();
        let is_setext = trimmed == "===" || trimmed == "---";

        if is_setext {
            let new_content = format!("{}{}", before, "\n");
            let cursor_pos = new_content.len();
            return TransformResult::new(new_content, cursor_pos);
        }

        let heading_prefix = "#".repeat(level as usize);
        let new_content = format!("{}{}{} ", before, "\n", heading_prefix);
        let cursor_pos = new_content.len();
        TransformResult::new(new_content, cursor_pos)
    }

    fn is_ordered_list_line(line: &str) -> bool {
        let trimmed = line.trim();
        if let Some(dot_pos) = trimmed.find('.') {
            let before_dot = &trimmed[..dot_pos];
            before_dot.chars().all(|c| c.is_ascii_digit())
        } else {
            false
        }
    }

    fn get_heading_level(line: &str) -> Option<u8> {
        let trimmed = line.trim_start();
        let hashes: usize = trimmed.chars().take_while(|&c| c == '#').count();
        if hashes > 0 && hashes <= 6 {
            let rest = &trimmed[hashes..];
            if rest.starts_with(' ') {
                return Some(hashes as u8);
            }
        }
        None
    }

    fn apply_wrap(
        &self,
        content: &str,
        cursor_offset: usize,
        before_marker: &str,
        after_marker: &str,
    ) -> TransformResult {
        let (pre_cursor, post_cursor) = content.split_at(cursor_offset);
        let new_content = format!(
            "{}{}{}{}",
            pre_cursor, before_marker, after_marker, post_cursor
        );
        TransformResult::new(
            new_content,
            cursor_offset + before_marker.len() + after_marker.len(),
        )
    }
}

impl Default for TransformEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if cursor is at the end of an empty list item
pub fn is_empty_list_item(content: &str, cursor_offset: usize) -> bool {
    let before = &content[..cursor_offset];
    let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let current_line = before[line_start..].trim();

    current_line == "-"
        || current_line == "*"
        || current_line.starts_with("- [ ]")
        || current_line.starts_with("- [x]")
        || current_line.starts_with("- [X]")
        || is_empty_ordered_list(current_line)
}

fn is_empty_ordered_list(line: &str) -> bool {
    // Check if line matches pattern like "1." or "123."
    if let Some(dot_pos) = line.find('.') {
        let before_dot = &line[..dot_pos];
        let after_dot = &line[dot_pos + 1..];
        before_dot.chars().all(|c| c.is_ascii_digit()) && after_dot.trim().is_empty()
    } else {
        false
    }
}

/// Check if cursor is in a blockquote
pub fn is_in_blockquote(content: &str, cursor_offset: usize) -> bool {
    let before = &content[..cursor_offset];
    let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let current_line = before[line_start..].trim_start();

    current_line.starts_with("> ")
}

/// Check if cursor is in a heading
pub fn is_in_heading(content: &str, cursor_offset: usize) -> Option<u8> {
    let before = &content[..cursor_offset];
    let line_start = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let current_line = before[line_start..].trim_start();

    let hashes = current_line.chars().take_while(|&c| c == '#').count();
    if hashes > 0 && hashes <= 6 {
        let rest = &current_line[hashes..];
        if rest.starts_with(' ') {
            return Some(hashes as u8);
        }
    }
    None
}

use crate::editor::commands::Command;

pub fn apply_transform(
    transform: Transform,
    content: &str,
    cursor_offset: usize,
) -> (String, Command) {
    let engine = TransformEngine::new();
    let result = engine.apply(&transform, content, cursor_offset);
    let cmd = Command::replace(
        SourceRange::new(
            Position::new(cursor_offset, 0, cursor_offset as u32),
            Position::new(cursor_offset, 0, cursor_offset as u32),
        ),
        &result.content,
        content,
    );
    (result.content, cmd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enter_in_empty_list_item() {
        let engine = TransformEngine::new();
        let content = "- ";
        let result = engine.apply(&Transform::Enter, content, 2);
        assert_eq!(result.content, "");
        assert_eq!(result.cursor_offset, 0);
    }

    #[test]
    fn test_enter_in_list_item() {
        let engine = TransformEngine::new();
        let content = "- item";
        let result = engine.apply(&Transform::Enter, content, 6);
        assert!(result.content.contains("- item"));
        assert!(result.content.contains('\n'));
    }

    #[test]
    fn test_tab_in_list_item() {
        let engine = TransformEngine::new();
        let content = "- item";
        let result = engine.apply(&Transform::Tab, content, 2);
        assert!(result.content.starts_with("    -"));
    }

    #[test]
    fn test_backspace_at_line_start() {
        let engine = TransformEngine::new();
        let content = "line1\nline2";
        let result = engine.apply(&Transform::Backspace, content, 6);
        assert_eq!(result.content, "line1line2");
        assert_eq!(result.cursor_offset, 5);
    }
}
