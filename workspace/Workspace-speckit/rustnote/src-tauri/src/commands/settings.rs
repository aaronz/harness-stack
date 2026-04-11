use crate::model::Settings;
use crate::commands::CommandError;
use std::fs;
use std::path::PathBuf;

fn get_settings_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rustnote")
        .join("settings.json")
}

#[tauri::command]
pub async fn read_settings() -> Result<Settings, CommandError> {
    let path = get_settings_path();
    if !path.exists() {
        return Ok(Settings::default());
    }
    let content = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&content)?)
}

#[tauri::command]
pub async fn write_settings(settings: Settings) -> Result<(), CommandError> {
    let path = get_settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(&settings)?;
    fs::write(&path, content)?;
    Ok(())
}