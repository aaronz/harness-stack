use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSaveSettings {
    pub enabled: bool,
    pub debounce_ms: u64,
}

impl Default for AutoSaveSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            debounce_ms: 2000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub font_family: String,
    pub font_size: u16,
    pub content_width: u16,
    pub line_height: f32,
    pub focus_mode: bool,
    pub typewriter_mode: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font_family: "System UI".into(),
            font_size: 16,
            content_width: 720,
            line_height: 1.6,
            focus_mode: false,
            typewriter_mode: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HtmlExportMode {
    Standalone,
    LinkedAssets,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PdfPageSize {
    A4,
    Letter,
    Legal,
}

impl Default for PdfPageSize {
    fn default() -> Self {
        Self::A4
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMargins {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for PdfMargins {
    fn default() -> Self {
        Self {
            top: 72.0,
            right: 72.0,
            bottom: 72.0,
            left: 72.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettings {
    pub html_mode: HtmlExportMode,
    pub pdf_page_size: PdfPageSize,
    pub pdf_margins: PdfMargins,
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            html_mode: HtmlExportMode::Standalone,
            pdf_page_size: PdfPageSize::A4,
            pdf_margins: PdfMargins::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub auto_save: AutoSaveSettings,
    pub editor: EditorSettings,
    pub export: ExportSettings,
    pub recent_files: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            auto_save: AutoSaveSettings::default(),
            editor: EditorSettings::default(),
            export: ExportSettings::default(),
            recent_files: Vec::new(),
        }
    }
}
