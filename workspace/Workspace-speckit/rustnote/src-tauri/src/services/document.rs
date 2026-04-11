use crate::editor::commands::Command;
use crate::editor::undo::UndoManager;
use crate::model::document::Document;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

pub struct DocumentService {
    documents: HashMap<Uuid, DocumentState>,
}

struct DocumentState {
    document: Document,
    undo_manager: UndoManager,
}

impl DocumentService {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    pub fn create(&mut self, title: String) -> Document {
        let doc = Document::new(title);
        let id = doc.id;
        self.documents.insert(
            id,
            DocumentState {
                document: doc.clone(),
                undo_manager: UndoManager::new(),
            },
        );
        doc
    }

    pub fn open(&mut self, path: PathBuf) -> std::io::Result<Document> {
        let content = std::fs::read_to_string(&path)?;
        let doc = Document::from_file(path.to_str().unwrap_or(""), content)?;
        let id = doc.id;
        self.documents.insert(
            id,
            DocumentState {
                document: doc.clone(),
                undo_manager: UndoManager::new(),
            },
        );
        Ok(doc)
    }

    pub fn save(&mut self, id: Uuid, path: Option<PathBuf>) -> std::io::Result<()> {
        if let Some(state) = self.documents.get_mut(&id) {
            if let Some(ref path) = path {
                state.document.file_path = Some(path.to_string_lossy().to_string());
            }
            let content = state.document.content.clone();
            if let Some(ref file_path) = state.document.file_path {
                let temp_path = format!("{}.tmp", file_path);
                std::fs::write(&temp_path, &content)?;
                std::fs::rename(&temp_path, file_path)?;
                state.document.mark_saved();
            }
        }
        Ok(())
    }

    pub fn get_content(&self, id: Uuid) -> Option<String> {
        self.documents.get(&id).map(|s| s.document.content.clone())
    }

    pub fn apply_command(&mut self, id: Uuid, command: Command) {
        if let Some(state) = self.documents.get_mut(&id) {
            let mut content = state.document.content.clone();
            command.apply(&mut content);
            state.document.update_content(content);
            state.undo_manager.execute(command);
        }
    }

    pub fn update_source(&mut self, id: Uuid, source: String) {
        if let Some(state) = self.documents.get_mut(&id) {
            state.document.update_content(source);
        }
    }

    pub fn undo(&mut self, id: Uuid) -> Option<Command> {
        if let Some(state) = self.documents.get_mut(&id) {
            if let Some(cmd) = state.undo_manager.undo() {
                if let Some(inv) = cmd.inverse() {
                    let mut content = state.document.content.clone();
                    inv.apply(&mut content);
                    state.document.update_content(content);
                    return Some(inv);
                }
            }
        }
        None
    }

    pub fn redo(&mut self, id: Uuid) -> Option<Command> {
        if let Some(state) = self.documents.get_mut(&id) {
            if let Some(cmd) = state.undo_manager.redo() {
                let mut content = state.document.content.clone();
                cmd.apply(&mut content);
                state.document.update_content(content);
                return Some(cmd);
            }
        }
        None
    }
}

impl Default for DocumentService {
    fn default() -> Self {
        Self::new()
    }
}
