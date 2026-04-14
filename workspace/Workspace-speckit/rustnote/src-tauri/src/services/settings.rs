use crate::model::settings::Settings;
use crate::services::{SettingsResult, SettingsServiceError, SettingsServiceTrait};
use rusqlite::{params, Connection, Result as SqlResult};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct SettingsService {
    pub conn: Mutex<Connection>,
    db_path: PathBuf,
}

impl SettingsService {
    pub fn new() -> SqlResult<Self> {
        Self::new_with_path(None)
    }

    pub fn new_with_path(custom_path: Option<PathBuf>) -> SqlResult<Self> {
        let db_path = custom_path.unwrap_or_else(|| Self::get_default_db_path());
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(&db_path)?;
        let service = Self {
            conn: Mutex::new(conn),
            db_path: db_path.clone(),
        };
        service.initialize_schema()?;

        if let Err(e) = service.migrate_from_json_if_needed() {
            log::warn!("Migration warning: {}", e);
        }

        Ok(service)
    }

    fn get_default_db_path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rustnote")
            .join("settings.db")
    }

    fn get_json_path(&self) -> PathBuf {
        self.db_path
            .parent()
            .map(|p| p.join("settings.json"))
            .unwrap_or_else(|| PathBuf::from("settings.json"))
    }

    fn initialize_schema(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS recent_files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                last_opened INTEGER NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS recent_folders (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                last_opened INTEGER NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS workspace_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY
            );
            ",
        )?;

        let version: Option<i32> = conn
            .query_row("SELECT version FROM schema_version LIMIT 1", [], |row| {
                row.get(0)
            })
            .ok();

        if version.is_none() {
            conn.execute(
                "INSERT INTO schema_version (version) VALUES (?)",
                params![1],
            )?;
        }

        Ok(())
    }

    fn migrate_from_json_if_needed(&self) -> Result<(), SettingsServiceError> {
        let json_path = self.get_json_path();

        let conn = self.conn.lock().unwrap();
        let has_settings: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM settings WHERE key = 'main')",
                [],
                |row| row.get(0),
            )
            .unwrap_or(false);

        if has_settings {
            return Ok(());
        }

        if !json_path.exists() {
            drop(conn);
            return self.write_settings(&Settings::default());
        }

        if let Ok(json_content) = fs::read_to_string(&json_path) {
            if let Ok(settings) = serde_json::from_str::<Settings>(&json_content) {
                drop(conn);
                if let Err(e) = self.write_settings(&settings) {
                    return Err(e);
                }
                let backup_path = json_path.with_extension("json.bak");
                fs::rename(&json_path, backup_path).ok();
            }
        }

        Ok(())
    }
}

impl SettingsServiceTrait for SettingsService {
    fn read_settings(&self) -> SettingsResult<Settings> {
        let conn = self.conn.lock().unwrap();

        let settings_json: Option<String> = conn
            .query_row("SELECT value FROM settings WHERE key = 'main'", [], |row| {
                row.get(0)
            })
            .ok();

        match settings_json {
            Some(json) => serde_json::from_str(&json)
                .map_err(|e| SettingsServiceError::Serialization(e.to_string())),
            None => Ok(Settings::default()),
        }
    }

    fn write_settings(&self, settings: &Settings) -> SettingsResult<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute("BEGIN TRANSACTION", [])
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;

        let json = serde_json::to_string(settings)
            .map_err(|e| SettingsServiceError::Serialization(e.to_string()))?;

        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('main', ?)",
            params![json],
        )
        .map_err(|e| SettingsServiceError::Database(e.to_string()))?;

        conn.execute("DELETE FROM recent_files", [])
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;

        for (idx, path) in settings.recent_files.iter().enumerate() {
            conn.execute(
                "INSERT INTO recent_files (id, path, last_opened) VALUES (?, ?, ?)",
                params![idx as i64 + 1, path, chrono::Utc::now().timestamp()],
            )
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        }

        conn.execute("DELETE FROM recent_folders", [])
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;

        for (idx, path) in settings.recent_folders.iter().enumerate() {
            conn.execute(
                "INSERT INTO recent_folders (id, path, last_opened) VALUES (?, ?, ?)",
                params![idx as i64 + 1, path, chrono::Utc::now().timestamp()],
            )
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        }

        conn.execute("COMMIT", [])
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        Ok(())
    }

    fn update_setting(&self, key: &str, value: &str) -> SettingsResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)",
            params![key, value],
        )
        .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        Ok(())
    }

    fn get_setting(&self, key: &str) -> SettingsResult<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let result: Option<String> = conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?",
                params![key],
                |row| row.get(0),
            )
            .ok();
        Ok(result)
    }

    fn add_recent_file(&self, path: &str) -> SettingsResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO recent_files (path, last_opened) VALUES (?, ?)",
            params![path, chrono::Utc::now().timestamp()],
        )
        .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        Ok(())
    }

    fn get_recent_files(&self) -> SettingsResult<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT path FROM recent_files ORDER BY last_opened DESC LIMIT 20")
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        let files = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(files)
    }

    fn clear_recent_files(&self) -> SettingsResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM recent_files", [])
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        Ok(())
    }

    fn add_recent_folder(&self, path: &str) -> SettingsResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO recent_folders (path, last_opened) VALUES (?, ?)",
            params![path, chrono::Utc::now().timestamp()],
        )
        .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        Ok(())
    }

    fn get_recent_folders(&self) -> SettingsResult<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT path FROM recent_folders ORDER BY last_opened DESC LIMIT 20")
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        let folders = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(folders)
    }

    fn clear_recent_folders(&self) -> SettingsResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM recent_folders", [])
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        Ok(())
    }

    fn set_workspace_state(&self, key: &str, value: &str) -> SettingsResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO workspace_state (key, value) VALUES (?, ?)",
            params![key, value],
        )
        .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        Ok(())
    }

    fn get_workspace_state(&self, key: &str) -> SettingsResult<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let result: Option<String> = conn
            .query_row(
                "SELECT value FROM workspace_state WHERE key = ?",
                params![key],
                |row| row.get(0),
            )
            .ok();
        Ok(result)
    }

    fn delete_workspace_state(&self, key: &str) -> SettingsResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM workspace_state WHERE key = ?", params![key])
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        Ok(())
    }

    fn verify_integrity(&self) -> SettingsResult<bool> {
        let conn = self.conn.lock().unwrap();
        let result: String = conn
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))
            .map_err(|e| SettingsServiceError::Database(e.to_string()))?;
        Ok(result == "ok")
    }
}

impl SettingsService {
    pub fn transaction_with_settings<F>(&self, f: F) -> SqlResult<()>
    where
        F: FnOnce(&Connection) -> SqlResult<()>,
    {
        let conn = self.conn.lock().unwrap();
        conn.execute("BEGIN TRANSACTION", [])?;

        let result = f(&conn);

        match result {
            Ok(()) => {
                conn.execute("COMMIT", [])?;
                Ok(())
            }
            Err(e) => {
                conn.execute("ROLLBACK", [])?;
                Err(e)
            }
        }
    }

    pub fn get_db_path_for_test() -> PathBuf {
        Self::get_default_db_path()
    }
}

impl Clone for SettingsService {
    fn clone(&self) -> Self {
        let conn = Connection::open(&self.db_path).expect("Failed to reopen settings database");
        Self {
            conn: Mutex::new(conn),
            db_path: self.db_path.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::settings::{Settings, Theme};
    use crate::services::{SettingsService, SettingsServiceTrait};
    use std::fs;

    fn create_test_db() -> SettingsService {
        let temp_dir = std::env::temp_dir().join("rustnote_test_db");
        fs::create_dir_all(&temp_dir).ok();
        let db_path = temp_dir.join("settings.db");
        SettingsService::new_with_path(Some(db_path))
            .expect("Failed to create test settings service")
    }

    #[test]
    fn test_settings_crud() {
        let service = create_test_db();

        let mut settings = Settings::default();
        settings.theme = Theme::Dark;
        settings.font_size = 18;
        settings.font_family = "Monaco".to_string();

        service.write_settings(&settings).unwrap();

        let read = service.read_settings().unwrap();
        assert_eq!(read.theme, Theme::Dark);
        assert_eq!(read.font_size, 18);
        assert_eq!(read.font_family, "Monaco");
    }

    #[test]
    fn test_settings_atomic_transaction() {
        let service = create_test_db();
        let result = service.transaction_with_settings(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES ('test1', 'value1')",
                [],
            )?;
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES ('test2', 'value2')",
                [],
            )?;
            Ok(())
        });

        assert!(result.is_ok());

        let val1: Option<String> = service.get_setting("test1").unwrap();
        assert_eq!(val1, Some("value1".to_string()));

        let result2 = service.transaction_with_settings(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES ('test3', 'value3')",
                [],
            )?;
            Err(rusqlite::Error::InvalidParameterName("Test".to_string()))
        });

        assert!(result2.is_err());

        let val3: Option<String> = service.get_setting("test3").unwrap();
        assert!(val3.is_none());
    }

    #[test]
    fn test_recent_files() {
        let service = create_test_db();

        service.add_recent_file("/path/to/file1.md").unwrap();
        service.add_recent_file("/path/to/file2.md").unwrap();

        let files = service.get_recent_files().unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&"/path/to/file1.md".to_string()));
        assert!(files.contains(&"/path/to/file2.md".to_string()));
    }

    #[test]
    fn test_workspace_state() {
        let service = create_test_db();

        service
            .set_workspace_state("last_folder", "/Users/test")
            .unwrap();
        let value = service.get_workspace_state("last_folder").unwrap();
        assert_eq!(value, Some("/Users/test".to_string()));

        service.delete_workspace_state("last_folder").unwrap();
        let value = service.get_workspace_state("last_folder").unwrap();
        assert!(value.is_none());
    }

    #[test]
    fn test_integrity_check() {
        let service = create_test_db();
        assert!(service.verify_integrity().unwrap());
    }
}
