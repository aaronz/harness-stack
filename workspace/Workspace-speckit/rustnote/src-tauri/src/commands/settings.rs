use crate::model::Settings;
use crate::commands::CommandError;
use crate::services::SettingsService;
use once_cell::sync::OnceCell;

static SETTINGS_SERVICE: OnceCell<SettingsService> = OnceCell::new();

fn get_settings_service() -> Result<&'static SettingsService, CommandError> {
    SETTINGS_SERVICE
        .get_or_try_init(|| SettingsService::new())
        .map_err(|e| CommandError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))
}

#[tauri::command]
pub async fn read_settings() -> Result<Settings, CommandError> {
    let service = get_settings_service()?;
    service.read_settings()
        .map_err(|e| CommandError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))
}

#[tauri::command]
pub async fn write_settings(settings: Settings) -> Result<(), CommandError> {
    let service = get_settings_service()?;
    service.write_settings(&settings)
        .map_err(|e| CommandError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))
}
