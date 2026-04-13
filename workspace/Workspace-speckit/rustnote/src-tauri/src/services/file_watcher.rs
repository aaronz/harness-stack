use crate::services::{FileWatcherResult, FileWatcherServiceError, FileWatcherServiceTrait};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub kind: String,
}

#[derive(Debug, Clone)]
struct FileState {
    content_hash: u64,
    modified: std::time::SystemTime,
}

pub struct FileWatcherService {
    watcher: Option<RecommendedWatcher>,
    receiver: Option<Mutex<Receiver<Result<Event, notify::Error>>>>,
    watched_paths: Arc<Mutex<HashMap<String, PathBuf>>>,
    file_states: Arc<Mutex<HashMap<String, FileState>>>,
}

impl FileWatcherService {
    pub fn new() -> Self {
        let watched_paths = Arc::new(Mutex::new(HashMap::new()));
        let file_states = Arc::new(Mutex::new(HashMap::new()));

        Self {
            watcher: None,
            receiver: None,
            watched_paths,
            file_states,
        }
    }
}

impl Default for FileWatcherService {
    fn default() -> Self {
        Self::new()
    }
}

impl FileWatcherServiceTrait for FileWatcherService {
    fn watch(&mut self, path: &str) -> FileWatcherResult<()> {
        self.watch_impl(path)
    }

    fn unwatch(&mut self, path: &str) -> FileWatcherResult<()> {
        self.unwatch_impl(path)
    }

    fn poll_changes(&self) -> Vec<FileChange> {
        self.poll_changes_impl()
    }

    fn check_file_changed(&self, path: &str) -> Option<bool> {
        self.check_file_changed_impl(path)
    }

    fn update_file_state(&self, path: &str) {
        self.update_file_state_impl(path);
    }

    fn is_watching(&self) -> bool {
        self.is_watching_impl()
    }
}

impl FileWatcherService {
    pub fn watch_impl(&mut self, path: &str) -> FileWatcherResult<()> {
        let path_buf = PathBuf::from(path);

        if !path_buf.exists() {
            return Err(FileWatcherServiceError::PathNotFound(path.to_string()));
        }

        if self.watcher.is_none() {
            let (tx, rx) = channel();

            let watcher = RecommendedWatcher::new(
                move |res| {
                    let _ = tx.send(res);
                },
                Config::default().with_poll_interval(Duration::from_secs(2)),
            )
            .map_err(|e| FileWatcherServiceError::WatcherError(e.to_string()))?;

            self.watcher = Some(watcher);
            self.receiver = Some(Mutex::new(rx));
        }

        let watch_path = if path_buf.is_dir() {
            path_buf.clone()
        } else {
            path_buf.clone()
        };

        if let Some(ref mut watcher) = self.watcher {
            watcher
                .watch(&watch_path, RecursiveMode::Recursive)
                .map_err(|e| FileWatcherServiceError::WatcherError(e.to_string()))?;
        }

        self.watched_paths
            .lock()
            .unwrap()
            .insert(path.to_string(), path_buf.clone());

        if let Ok(content) = fs::read_to_string(path) {
            let hash = self.compute_hash(&content);
            let modified = fs::metadata(path)
                .and_then(|m| m.modified())
                .unwrap_or_else(|_| std::time::SystemTime::now());

            self.file_states.lock().unwrap().insert(
                path.to_string(),
                FileState {
                    content_hash: hash,
                    modified,
                },
            );
        }

        Ok(())
    }

    pub fn unwatch_impl(&mut self, path: &str) -> FileWatcherResult<()> {
        if let Some(watched) = self.watched_paths.lock().unwrap().remove(path) {
            self.file_states.lock().unwrap().remove(path);
            if let Some(ref mut watcher) = self.watcher {
                watcher
                    .unwatch(&watched)
                    .map_err(|e| FileWatcherServiceError::WatcherError(e.to_string()))?;
            }
        }
        Ok(())
    }

    pub fn poll_changes_impl(&self) -> Vec<FileChange> {
        let mut changes = Vec::new();

        if let Some(ref rx) = self.receiver {
            if let Ok(guard) = rx.lock() {
                while let Ok(result) = guard.try_recv() {
                    if let Ok(event) = result {
                        for path in event.paths {
                            let kind = match event.kind {
                                notify::EventKind::Create(_) => "create",
                                notify::EventKind::Modify(_) => "modify",
                                notify::EventKind::Remove(_) => "remove",
                                _ => continue,
                            };

                            if let Some(path_str) = path.to_str() {
                                if path_str.ends_with(".md") || path_str.ends_with(".markdown") {
                                    changes.push(FileChange {
                                        path: path_str.to_string(),
                                        kind: kind.to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        changes
    }

    pub fn check_file_changed_impl(&self, path: &str) -> Option<bool> {
        let states = self.file_states.lock().unwrap();
        let previous_state = states.get(path)?;

        let current_content = fs::read_to_string(path).ok()?;
        let current_hash = self.compute_hash(&current_content);
        let current_modified = fs::metadata(path).and_then(|m| m.modified()).ok()?;

        Some(
            current_hash != previous_state.content_hash
                || current_modified > previous_state.modified,
        )
    }

    pub fn update_file_state_impl(&self, path: &str) {
        if let Ok(content) = fs::read_to_string(path) {
            let hash = self.compute_hash(&content);
            let modified = fs::metadata(path)
                .and_then(|m| m.modified())
                .unwrap_or_else(|_| std::time::SystemTime::now());

            self.file_states.lock().unwrap().insert(
                path.to_string(),
                FileState {
                    content_hash: hash,
                    modified,
                },
            );
        }
    }

    fn compute_hash(&self, content: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }

    pub fn is_watching_impl(&self) -> bool {
        self.watcher.is_some()
    }
}
