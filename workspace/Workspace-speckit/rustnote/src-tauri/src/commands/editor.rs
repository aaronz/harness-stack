use crate::editor::search::{SearchEngine, SearchOptions};
use crate::editor::transforms::{Transform, TransformEngine};

#[tauri::command]
pub fn editor_apply_transform(
    transform: TransformType,
    content: String,
    cursor_offset: usize,
) -> TransformResultDto {
    let engine = TransformEngine::new();
    let transform = match transform {
        TransformType::Enter => Transform::Enter,
        TransformType::Backspace => Transform::Backspace,
        TransformType::Tab => Transform::Tab,
        TransformType::ShiftTab => Transform::ShiftTab,
        TransformType::EnterInListItem => Transform::EnterInListItem { is_empty: false },
        TransformType::EnterInBlockQuote => Transform::EnterInBlockQuote,
        TransformType::EnterInHeading { level } => Transform::EnterInHeading { level },
        TransformType::Wrap { before, after } => Transform::Wrap {
            before: before.clone(),
            after: after.clone(),
        },
    };
    let result = engine.apply(&transform, &content, cursor_offset);
    TransformResultDto {
        content: result.content,
        cursor_offset: result.cursor_offset,
    }
}

#[tauri::command]
pub fn editor_search(
    content: String,
    pattern: String,
    case_sensitive: bool,
    use_regex: bool,
) -> SearchResultDto {
    let engine = SearchEngine::new();
    let options = SearchOptions {
        case_sensitive,
        use_regex,
    };
    let result = engine.search(&content, &pattern, &options);
    SearchResultDto {
        matches: result
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
        count: result.count,
        success: result.success,
        error: result.error,
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
    let engine = SearchEngine::new();
    let options = SearchOptions {
        case_sensitive,
        use_regex,
    };
    engine
        .find_next(&content, &pattern, &options, after_offset)
        .map(|m| SearchMatchDto {
            start: m.start,
            end: m.end,
            text: m.text,
            line: m.line,
            column: m.column,
        })
}

#[tauri::command]
pub fn editor_find_previous(
    content: String,
    pattern: String,
    case_sensitive: bool,
    use_regex: bool,
    before_offset: usize,
) -> Option<SearchMatchDto> {
    let engine = SearchEngine::new();
    let options = SearchOptions {
        case_sensitive,
        use_regex,
    };
    engine
        .find_previous(&content, &pattern, &options, before_offset)
        .map(|m| SearchMatchDto {
            start: m.start,
            end: m.end,
            text: m.text,
            line: m.line,
            column: m.column,
        })
}

#[tauri::command]
pub fn editor_replace_match(
    content: String,
    start: usize,
    end: usize,
    replacement: String,
) -> String {
    SearchEngine::replace_match(&content, start, end, &replacement)
}

#[tauri::command]
pub fn editor_replace_all(
    content: String,
    pattern: String,
    case_sensitive: bool,
    use_regex: bool,
    replacement: String,
) -> ReplaceAllResultDto {
    let options = SearchOptions {
        case_sensitive,
        use_regex,
    };
    let result = SearchEngine::replace_all(&content, &pattern, &options, &replacement);
    ReplaceAllResultDto {
        content: result
            .matches
            .first()
            .map(|m| m.text.clone())
            .unwrap_or(content),
        replacements: result.count,
        success: result.success,
        error: result.error,
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
