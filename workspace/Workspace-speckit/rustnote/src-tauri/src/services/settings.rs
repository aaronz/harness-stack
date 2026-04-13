use rusqlite::{params, Connection, Result as SqlResult};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::model::settings::Settings;

/// Database-backed settings service using rusqlite for atomic transactions and crash recovery
pub struct SettingsService {
    pub conn: Mutex<Connection>,
    db_path: PathBuf,
}

impl SettingsService {
    /// Create or open the settings database
    pub fn new() -> SqlResult<Self> {
        Self::new_with_path(None)
    }

    /// Create or open the settings database with a custom path
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

        // Run migration if needed
        service.migrate_from_json_if_needed()?;

        Ok(service)
    }

    /// Get the default database path
    fn get_default_db_path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rustnote")
            .join("settings.db")
    }

    /// Get the JSON settings path for migration
    fn get_json_path(&self) -> PathBuf {
        self.db_path
            .parent()
            .map(|p| p.join("settings.json"))
            .unwrap_or_else(|| PathBuf::from("settings.json"))
    }

    /// Initialize the database schema
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
            
            CREATE TABLE IF NOT EXISTS workspace_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY
            );
            ",
        )?;

        // Set initial schema version if not set
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

    /// Migrate settings from JSON if the JSON file exists and DB is empty
    fn migrate_from_json_if_needed(&self) -> SqlResult<()> {
        let json_path = self.get_json_path();

        // Check if we already have settings in the DB
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

        // Check if JSON file exists
        if !json_path.exists() {
            // Initialize with defaults
            drop(conn);
            self.write_settings(&Settings::default())?;
            return Ok(());
        }

        // Read and migrate JSON settings
        if let Ok(json_content) = fs::read_to_string(&json_path) {
            if let Ok(settings) = serde_json::from_str::<Settings>(&json_content) {
                drop(conn);
                self.write_settings(&settings)?;

                // Rename JSON file to backup
                let backup_path = json_path.with_extension("json.bak");
                fs::rename(&json_path, backup_path).ok();
            }
        }

        Ok(())
    }

    /// Read all settings from the database
    pub fn read_settings(&self) -> SqlResult<Settings> {
        let conn = self.conn.lock().unwrap();

        // Get main settings JSON
        let settings_json: Option<String> = conn
            .query_row("SELECT value FROM settings WHERE key = 'main'", [], |row| {
                row.get(0)
            })
            .ok();

        match settings_json {
            Some(json) => serde_json::from_str(&json)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string())),
            None => Ok(Settings::default()),
        }
    }

    /// Write all settings to the database (atomic transaction)
    pub fn write_settings(&self, settings: &Settings) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();

        // Use transaction for atomicity
        conn.execute("BEGIN TRANSACTION", [])?;

        let result = (|| {
            let json = serde_json::to_string(settings)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES ('main', ?)",
                params![json],
            )?;

            // Update recent files
            conn.execute("DELETE FROM recent_files", [])?;
            for (idx, path) in settings.recent_files.iter().enumerate() {
                conn.execute(
                    "INSERT INTO recent_files (id, path, last_opened) VALUES (?, ?, ?)",
                    params![idx as i64 + 1, path, chrono::Utc::now().timestamp()],
                )?;
            }

            Ok::<(), rusqlite::Error>(())
        })();

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

    /// Write settings atomically - all or nothing
    pub fn write_settings_atomic(&self, settings: &Settings) -> SqlResult<()> {
        self.write_settings(settings)
    }

    /// Update a single setting value atomically
    pub fn update_setting(&self, key: &str, value: &str) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)",
            params![key, value],
        )?;
        Ok(())
    }

    /// Get a single setting value
    pub fn get_setting(&self, key: &str) -> SqlResult<Option<String>> {
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

    // ===== Recent Files Operations =====

    /// Add a file to recent files
    pub fn add_recent_file(&self, path: &str) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO recent_files (path, last_opened) VALUES (?, ?)",
            params![path, chrono::Utc::now().timestamp()],
        )?;
        Ok(())
    }

    /// Get all recent files
    pub fn get_recent_files(&self) -> SqlResult<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT path FROM recent_files ORDER BY last_opened DESC LIMIT 20")?;
        let files = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(files)
    }

    /// Clear all recent files
    pub fn clear_recent_files(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM recent_files", [])?;
        Ok(())
    }

    // ===== Workspace State Operations =====

    /// Set workspace state
    pub fn set_workspace_state(&self, key: &str, value: &str) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO workspace_state (key, value) VALUES (?, ?)",
            params![key, value],
        )?;
        Ok(())
    }

    /// Get workspace state
    pub fn get_workspace_state(&self, key: &str) -> SqlResult<Option<String>> {
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

    /// Delete workspace state
    pub fn delete_workspace_state(&self, key: &str) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM workspace_state WHERE key = ?", params![key])?;
        Ok(())
    }

    // ===== Atomic Transaction Tests =====

    /// Execute multiple setting changes atomically
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

    /// Verify database integrity
    pub fn verify_integrity(&self) -> SqlResult<bool> {
        let conn = self.conn.lock().unwrap();
        let result: String = conn.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
        Ok(result == "ok")
    }

    /// Get database path for testing
    #[cfg(test)]
    pub fn get_db_path_for_test() -> PathBuf {
        Self::get_default_db_path()
    }
}

// Make SettingsService cloneable for testing purposes
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
    use crate::services::SettingsService;
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

        // Write settings
        let mut settings = Settings::default();
        settings.theme = Theme::Dark;
        settings.editor.font_size = 18;
        settings.editor.font_family = "Monaco".to_string();

        service.write_settings(&settings).unwrap();

        // Read settings back
        let read = service.read_settings().unwrap();
        assert_eq!(read.theme, Theme::Dark);
        assert_eq!(read.editor.font_size, 18);
        assert_eq!(read.editor.font_family, "Monaco");
    }

    #[test]
    fn test_settings_atomic_transaction() {
        // Test successful transaction
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

        // Values should be present after successful transaction
        let val1: Option<String> = service.get_setting("test1").unwrap();
        assert_eq!(val1, Some("value1".to_string()));

        // Test rollback on error
        let result2 = service.transaction_with_settings(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES ('test3', 'value3')",
                [],
            )?;
            // Return error to trigger rollback
            Err(rusqlite::Error::InvalidParameterName("Test".to_string()))
        });

        assert!(result2.is_err());

        // Value should not be present due to rollback
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
