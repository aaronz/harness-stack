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
    };
    let result = engine.apply(&transform, &content, cursor_offset);
    TransformResultDto {
        content: result.content,
        cursor_offset: result.cursor_offset,
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
}

#[derive(serde::Serialize)]
pub struct TransformResultDto {
    pub content: String,
    pub cursor_offset: usize,
}
