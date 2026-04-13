use crate::services::file_watcher::FileWatcherService;
use crate::services::FileWatcherServiceTrait;
use std::fs;
use std::sync::Mutex;

static WATCHER: Mutex<Option<FileWatcherService>> = Mutex::new(None);

fn get_watcher() -> Result<std::sync::MutexGuard<'static, Option<FileWatcherService>>, String> {
    WATCHER.lock().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn watch_file(path: String) -> Result<(), String> {
    let mut guard = get_watcher()?;

    if guard.is_none() {
        *guard = Some(FileWatcherService::new());
    }

    if let Some(ref mut watcher) = *guard {
        watcher.watch(&path).map_err(|e| e.to_string())
    } else {
        Err("Failed to initialize watcher".to_string())
    }
}

#[tauri::command]
pub fn unwatch_file(path: String) -> Result<(), String> {
    let mut guard = get_watcher()?;

    if let Some(ref mut watcher) = *guard {
        watcher.unwatch(&path).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn poll_file_changes() -> Vec<crate::services::file_watcher::FileChange> {
    if let Ok(guard) = get_watcher() {
        if let Some(ref watcher) = *guard {
            return watcher.poll_changes();
        }
    }
    Vec::new()
}

#[tauri::command]
pub fn check_external_change(path: String) -> Result<ExternalChangeResult, String> {
    let guard = get_watcher()?;

    if let Some(ref watcher) = *guard {
        if let Some(has_changed) = watcher.check_file_changed(&path) {
            let current_content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            return Ok(ExternalChangeResult {
                has_changed,
                content: current_content,
            });
        }
    }

    let current_content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    Ok(ExternalChangeResult {
        has_changed: false,
        content: current_content,
    })
}

#[tauri::command]
pub fn update_watched_file_state(path: String) -> Result<(), String> {
    let guard = get_watcher()?;

    if let Some(ref watcher) = *guard {
        watcher.update_file_state(&path);
    }

    Ok(())
}

#[derive(serde::Serialize)]
pub struct ExternalChangeResult {
    pub has_changed: bool,
    pub content: String,
}
