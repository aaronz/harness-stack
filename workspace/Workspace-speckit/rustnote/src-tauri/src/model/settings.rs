use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorSettings {
    pub font_family: String,
    pub font_size: u32,
    pub line_height: f32,
    pub tab_size: u32,
    pub content_width: u32,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font_family: "System".to_string(),
            font_size: 16,
            line_height: 1.6,
            tab_size: 4,
            content_width: 720,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub theme: Theme,
    pub auto_save: bool,
    pub auto_save_interval: u32,
    pub focus_mode: bool,
    pub typewriter_mode: bool,
    pub outline_visible: bool,
    #[serde(flatten)]
    pub editor: EditorSettings,
    pub recent_files: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            auto_save: true,
            auto_save_interval: 30000,
            focus_mode: false,
            typewriter_mode: false,
            outline_visible: false,
            editor: EditorSettings::default(),
            recent_files: vec![],
        }
    }
}
