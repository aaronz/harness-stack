#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rustnote_core::{
    core::{
        EditorState, FileNode, OutlineItem as CoreOutlineItem, RecoverySnapshot,
        RecoverySnapshotMeta, Settings,
    },
    services::{
        DocumentService, EditorService, ExportService, OutlineItem, RecoveryService,
        SettingsService, WorkspaceService,
    },
};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    editor: Mutex<EditorState>,
    current_file: Mutex<Option<PathBuf>>,
}

#[tauri::command]
fn doc_open(path: String, state: State<AppState>) -> Result<DocOpenResult, String> {
    let path = PathBuf::from(&path);
    let (content, _doc) = DocumentService::open(&path).map_err(|e| e.to_string())?;

    let mut editor = state.editor.lock().map_err(|e| e.to_string())?;
    editor.load_content(content.clone());

    let mut current_file = state.current_file.lock().map_err(|e| e.to_string())?;
    *current_file = Some(path.clone());

    let outline = DocumentService::get_outline(&content);

    Ok(DocOpenResult {
        content,
        outline,
        file_path: path.to_string_lossy().to_string(),
    })
}

#[derive(serde::Serialize)]
struct DocOpenResult {
    content: String,
    outline: Vec<OutlineItem>,
    file_path: String,
}

#[tauri::command]
fn doc_save(path: String, content: String, state: State<AppState>) -> Result<(), String> {
    let path = PathBuf::from(&path);
    DocumentService::save(&path, &content).map_err(|e| e.to_string())?;

    let mut editor = state.editor.lock().map_err(|e| e.to_string())?;
    editor.load_content(content);
    editor.mark_saved();

    Ok(())
}

#[tauri::command]
fn doc_reload(path: String, state: State<AppState>) -> Result<String, String> {
    let path = PathBuf::from(&path);
    let content = DocumentService::reload(&path).map_err(|e| e.to_string())?;

    let mut editor = state.editor.lock().map_err(|e| e.to_string())?;
    editor.load_content(content.clone());

    Ok(content)
}

#[tauri::command]
fn doc_get_outline(content: String) -> Result<Vec<OutlineItem>, String> {
    Ok(DocumentService::get_outline(&content))
}

#[tauri::command]
fn render_markdown(content: String) -> Result<String, String> {
    Ok(DocumentService::render_html(&content))
}

#[tauri::command]
fn apply_transform(
    text: String,
    selection_start: usize,
    selection_end: usize,
    key: String,
    state: State<AppState>,
) -> Result<TransformResult, String> {
    use rustnote_core::core::editor::EditorCommand;
    use rustnote_core::core::selection::{Cursor, Selection};

    let mut editor = state.editor.lock().map_err(|e| e.to_string())?;
    editor.load_content(text.clone());

    let selection = if selection_start == selection_end {
        Selection::caret(Cursor::new(selection_start))
    } else {
        Selection::new(Cursor::new(selection_start), Cursor::new(selection_end))
    };
    editor.set_selection(selection);

    let cmd = match key.as_str() {
        "Enter" => EditorCommand::InsertNewline,
        "Backspace" => EditorCommand::DeleteBackward,
        "Delete" => EditorCommand::DeleteForward,
        "ArrowLeft" => EditorCommand::MoveCursorLeft,
        "ArrowRight" => EditorCommand::MoveCursorRight,
        "ArrowUp" => EditorCommand::MoveCursorUp,
        "ArrowDown" => EditorCommand::MoveCursorDown,
        _ => EditorCommand::InsertText { text: key },
    };

    let result = editor.apply_command(cmd);

    Ok(TransformResult {
        content: result.content,
        cursor_offset: result.selection.start().offset,
    })
}

#[derive(serde::Serialize)]
struct TransformResult {
    content: String,
    cursor_offset: usize,
}

#[tauri::command]
fn editor_undo(state: State<AppState>) -> Result<Option<TransformResult>, String> {
    let mut editor = state.editor.lock().map_err(|e| e.to_string())?;
    if let Some(result) = EditorService::undo(&mut editor) {
        Ok(Some(TransformResult {
            content: result.content,
            cursor_offset: result.selection.start().offset,
        }))
    } else {
        Ok(None)
    }
}

#[tauri::command]
fn editor_redo(state: State<AppState>) -> Result<Option<TransformResult>, String> {
    let mut editor = state.editor.lock().map_err(|e| e.to_string())?;
    if let Some(result) = EditorService::redo(&mut editor) {
        Ok(Some(TransformResult {
            content: result.content,
            cursor_offset: result.selection.start().offset,
        }))
    } else {
        Ok(None)
    }
}

#[tauri::command]
fn workspace_open(root: String) -> Result<WorkspaceResult, String> {
    let root_path = PathBuf::from(&root);
    let workspace = WorkspaceService::open(&root_path).map_err(|e| e.to_string())?;

    fn file_node_to_json(node: &FileNode) -> FileNodeJson {
        FileNodeJson {
            path: node.path.to_string_lossy().to_string(),
            name: node.name.clone(),
            is_dir: node.is_dir,
            children: node.children.iter().map(file_node_to_json).collect(),
        }
    }

    Ok(WorkspaceResult {
        root_path: root,
        tree: file_node_to_json(&workspace.tree),
    })
}

#[derive(serde::Serialize)]
struct FileNodeJson {
    path: String,
    name: String,
    is_dir: bool,
    children: Vec<FileNodeJson>,
}

#[derive(serde::Serialize)]
struct WorkspaceResult {
    root_path: String,
    tree: FileNodeJson,
}

#[tauri::command]
fn workspace_create_file(parent: String, name: String) -> Result<String, String> {
    let parent_path = PathBuf::from(&parent);
    let path = WorkspaceService::create_file(&parent_path, &name).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn workspace_delete_file(path: String) -> Result<(), String> {
    let path = PathBuf::from(&path);
    WorkspaceService::delete_file(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn settings_get() -> Result<Settings, String> {
    SettingsService::load().map_err(|e| e.to_string())
}

#[tauri::command]
fn settings_update(settings: Settings) -> Result<(), String> {
    SettingsService::save(&settings).map_err(|e| e.to_string())
}

#[tauri::command]
fn export_html(content: String, standalone: bool) -> Result<String, String> {
    Ok(ExportService::export_html(&content, standalone))
}

#[tauri::command]
fn recovery_list() -> Result<Vec<RecoverySnapshotMeta>, String> {
    RecoveryService::list_snapshots().map_err(|e| e.to_string())
}

#[tauri::command]
fn recovery_restore(id: String) -> Result<RecoveryData, String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let (content, cursor_offset) =
        RecoveryService::restore_snapshot(&uuid).map_err(|e| e.to_string())?;
    Ok(RecoveryData {
        content,
        cursor_offset,
    })
}

#[derive(serde::Serialize)]
struct RecoveryData {
    content: String,
    cursor_offset: usize,
}

#[tauri::command]
fn recovery_delete(id: String) -> Result<(), String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    RecoveryService::delete_snapshot(&uuid).map_err(|e| e.to_string())
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting RustNote application");

    tauri::Builder::default()
        .manage(AppState {
            editor: Mutex::new(EditorState::new()),
            current_file: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            doc_open,
            doc_save,
            doc_reload,
            doc_get_outline,
            render_markdown,
            apply_transform,
            editor_undo,
            editor_redo,
            workspace_open,
            workspace_create_file,
            workspace_delete_file,
            settings_get,
            settings_update,
            export_html,
            recovery_list,
            recovery_restore,
            recovery_delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
