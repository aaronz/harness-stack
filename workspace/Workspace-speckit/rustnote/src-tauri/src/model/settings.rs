use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
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
    pub font_family: String,
    pub font_size: u32,
    pub line_height: f32,
    pub content_width: u32,
    pub recent_files: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            auto_save: true,
            auto_save_interval: 10000,
            focus_mode: false,
            typewriter_mode: false,
            outline_visible: false,
            font_family: "System".to_string(),
            font_size: 16,
            line_height: 1.6,
            content_width: 720,
            recent_files: vec![],
        }
    }
}
