pub mod buffer;
pub mod commands;
pub mod editor;
pub mod model;
pub mod parser;
pub mod renderer;
pub mod semantic;
pub mod services;

use commands::{check_external_change, cleanup_old_snapshots, create_document, create_file, create_folder, delete_item, delete_recovery_snapshot, editor_apply_transform, editor_find_next, editor_find_previous, editor_replace_all, editor_replace_match, editor_search, export_to_html, export_to_pdf, export_to_pdf_native, get_highlighted_code_html, get_markdown_info, get_print_html, highlight_code_block, image_markdown_from_path, insert_image, list_recovery_snapshots, list_workspace, open_document, open_external_url, parse_markdown_ast, poll_file_changes, prehighlight_markdown, read_document_content, read_settings, rename_item, render_for_editor, render_for_editor_with_highlighting, render_markdown, restore_recovery_snapshot, save_document, save_image_from_base64_cmd, save_recovery_snapshot, serialize_markdown, unwatch_file, update_source, update_watched_file_state, watch_file, write_document_content, write_settings};

use std::panic;

fn setup_panic_handler() {
    panic::set_hook(Box::new(|panic_info| {
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };
        
        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown".to_string());
        
        log::error!("PANIC at {}: {}", location, msg);
    }));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_panic_handler();
    
    log::info!("Starting RustNote application");
    
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        );
    
    #[cfg(feature = "e2e-testing")]
    {
        builder = builder.plugin(tauri_plugin_playwright::init());
    }
    
    builder
        .invoke_handler(tauri::generate_handler![
            create_document,
            open_document,
            save_document,
            read_document_content,
            write_document_content,
            list_workspace,
            create_file,
            create_folder,
            rename_item,
            delete_item,
            read_settings,
            write_settings,
            export_to_html,
            export_to_pdf,
            export_to_pdf_native,
            get_print_html,
            render_markdown,
            parse_markdown_ast,
            serialize_markdown,
            get_markdown_info,
            editor_apply_transform,
            editor_search,
            editor_find_next,
            editor_find_previous,
            editor_replace_match,
            editor_replace_all,
            update_source,
            render_for_editor,
            render_for_editor_with_highlighting,
            highlight_code_block,
            get_highlighted_code_html,
            prehighlight_markdown,
            insert_image,
            image_markdown_from_path,
            save_image_from_base64_cmd,
            save_recovery_snapshot,
            list_recovery_snapshots,
            restore_recovery_snapshot,
            delete_recovery_snapshot,
            cleanup_old_snapshots,
            watch_file,
            unwatch_file,
            poll_file_changes,
            check_external_change,
            update_watched_file_state,
            open_external_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}