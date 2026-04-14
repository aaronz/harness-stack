use crate::editor::commands::EditorCommand;
use crate::editor::transforms::Transform;
use crate::semantic::ast::SourceRange;

#[tauri::command]
pub fn editor_apply_transform(
    transform: TransformType,
    content: String,
    cursor_offset: usize,
    selection_start: Option<usize>,
) -> TransformResultDto {
    let is_wrap = matches!(transform, TransformType::Wrap { .. });

    let editor_transform = match transform {
        TransformType::Enter => Transform::Enter,
        TransformType::Backspace => Transform::Backspace,
        TransformType::Tab => Transform::Tab,
        TransformType::ShiftTab => Transform::ShiftTab,
        TransformType::EnterInListItem => Transform::EnterInListItem { is_empty: false },
        TransformType::EnterInBlockQuote => Transform::EnterInBlockQuote,
        TransformType::EnterInHeading { level } => Transform::EnterInHeading { level },
        TransformType::Wrap { before, after } => Transform::Wrap { before, after },
    };

    let (new_content, new_cursor_offset) = if is_wrap {
        let engine = crate::editor::transforms::TransformEngine::new();
        let result = engine.apply(&editor_transform, &content, cursor_offset, selection_start);
        (result.content, result.cursor_offset)
    } else {
        let command = EditorCommand::transform(editor_transform);
        let (new_content, _) = command.apply(&content, cursor_offset);
        (new_content, cursor_offset)
    };

    TransformResultDto {
        content: new_content,
        cursor_offset: new_cursor_offset,
    }
}

#[tauri::command]
pub fn editor_search(
    content: String,
    pattern: String,
    case_sensitive: bool,
    use_regex: bool,
) -> SearchResultDto {
    let command = EditorCommand::search(pattern, case_sensitive, use_regex);
    let (_, result) = command.apply(&content, 0);
    match result {
        crate::editor::commands::CommandResult::Search(search_result) => SearchResultDto {
            matches: search_result
                .matches
                .into_iter()
                .map(|m| SearchMatchDto {
                    start: m.start,
                    end: m.end,
                    text: m.text,
                    line: m.line,
                    column: m.column,
                })
                .collect(),
            count: search_result.count,
            success: search_result.success,
            error: search_result.error,
        },
        _ => SearchResultDto {
            matches: vec![],
            count: 0,
            success: false,
            error: Some("Invalid result type".to_string()),
        },
    }
}

#[tauri::command]
pub fn editor_find_next(
    content: String,
    pattern: String,
    case_sensitive: bool,
    use_regex: bool,
    after_offset: usize,
) -> Option<SearchMatchDto> {
    let command = EditorCommand::find_next(pattern, case_sensitive, use_regex, after_offset);
    let (_, result) = command.apply(&content, 0);
    match result {
        crate::editor::commands::CommandResult::Match(m) => m.map(|m| SearchMatchDto {
            start: m.start,
            end: m.end,
            text: m.text,
            line: m.line,
            column: m.column,
        }),
        _ => None,
    }
}

#[tauri::command]
pub fn editor_find_previous(
    content: String,
    pattern: String,
    case_sensitive: bool,
    use_regex: bool,
    before_offset: usize,
) -> Option<SearchMatchDto> {
    let command = EditorCommand::find_previous(pattern, case_sensitive, use_regex, before_offset);
    let (_, result) = command.apply(&content, 0);
    match result {
        crate::editor::commands::CommandResult::Match(m) => m.map(|m| SearchMatchDto {
            start: m.start,
            end: m.end,
            text: m.text,
            line: m.line,
            column: m.column,
        }),
        _ => None,
    }
}

#[tauri::command]
pub fn editor_replace_match(
    content: String,
    start: usize,
    end: usize,
    replacement: String,
) -> String {
    let range = SourceRange::new(
        crate::semantic::ast::Position::new(start, 0, start as u32),
        crate::semantic::ast::Position::new(end, 0, end as u32),
    );
    let command = EditorCommand::replace_match(range, replacement);
    let (new_content, _) = command.apply(&content, 0);
    new_content
}

#[tauri::command]
pub fn editor_replace_all(
    content: String,
    pattern: String,
    case_sensitive: bool,
    use_regex: bool,
    replacement: String,
) -> ReplaceAllResultDto {
    let command = EditorCommand::replace_all(pattern, case_sensitive, use_regex, replacement);
    let (_, result) = command.apply(&content, 0);
    match result {
        crate::editor::commands::CommandResult::ReplaceAll {
            content,
            replacements,
        } => ReplaceAllResultDto {
            content,
            replacements,
            success: true,
            error: None,
        },
        _ => ReplaceAllResultDto {
            content,
            replacements: 0,
            success: false,
            error: Some("Invalid result type".to_string()),
        },
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransformType {
    Enter,
    Backspace,
    Tab,
    ShiftTab,
    EnterInListItem,
    EnterInBlockQuote,
    EnterInHeading { level: u8 },
    Wrap { before: String, after: String },
}

#[derive(serde::Serialize)]
pub struct TransformResultDto {
    pub content: String,
    pub cursor_offset: usize,
}

#[derive(serde::Serialize)]
pub struct SearchMatchDto {
    pub start: usize,
    pub end: usize,
    pub text: String,
    pub line: u32,
    pub column: u32,
}

#[derive(serde::Serialize)]
pub struct SearchResultDto {
    pub matches: Vec<SearchMatchDto>,
    pub count: usize,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ReplaceAllResultDto {
    pub content: String,
    pub replacements: usize,
    pub success: bool,
    pub error: Option<String>,
}
