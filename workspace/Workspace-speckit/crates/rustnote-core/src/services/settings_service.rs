use crate::core::settings::Settings;
use crate::error::Result;
use std::fs;
use std::path::PathBuf;

pub struct SettingsService;

impl SettingsService {
    fn settings_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rustnote")
            .join("settings.json")
    }

    pub fn load() -> Result<Settings> {
        let path = Self::settings_path();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let settings: Settings = serde_json::from_str(&content)
                .map_err(|e| crate::error::Error::Serialization(e.to_string()))?;
            Ok(settings)
        } else {
            Ok(Settings::default())
        }
    }

    pub fn save(settings: &Settings) -> Result<()> {
        let path = Self::settings_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(settings)
            .map_err(|e| crate::error::Error::Serialization(e.to_string()))?;
        fs::write(&path, content)?;
        Ok(())
    }
}
