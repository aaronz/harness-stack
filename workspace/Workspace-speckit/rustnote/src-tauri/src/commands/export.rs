use crate::parser::MarkdownParser;
use crate::commands::CommandError;
use std::fs;

#[tauri::command]
pub async fn export_to_html(markdown: String, output_path: String) -> Result<(), CommandError> {
    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(&markdown);
    
    let full_html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Exported Document</title>
    <style>
        body {{ font-family: -apple-system, system-ui, sans-serif; max-width: 800px; margin: 40px auto; padding: 20px; }}
        pre {{ background: #f5f5f5; padding: 16px; border-radius: 4px; overflow-x: auto; }}
        code {{ background: #f5f5f5; padding: 2px 6px; border-radius: 3px; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        blockquote {{ border-left: 4px solid #ddd; margin: 0; padding-left: 16px; color: #666; }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        html
    );
    
    fs::write(&output_path, full_html)?;
    Ok(())
}

#[tauri::command]
pub fn get_print_html(markdown: String) -> String {
    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(&markdown);
    
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Exported Document</title>
    <style>
        @page {{ margin: 20mm; size: A4; }}
        body {{ font-family: -apple-system, system-ui, sans-serif; font-size: 12pt; line-height: 1.5; }}
        h1 {{ font-size: 24pt; page-break-after: avoid; }}
        h2 {{ font-size: 18pt; page-break-after: avoid; }}
        h3 {{ font-size: 14pt; page-break-after: avoid; }}
        pre {{ background: #f5f5f5; padding: 12px; border-radius: 4px; overflow-x: auto; page-break-inside: avoid; }}
        code {{ background: #f5f5f5; padding: 2px 4px; border-radius: 2px; font-family: 'SF Mono', Monaco, monospace; font-size: 10pt; }}
        table {{ border-collapse: collapse; width: 100%; page-break-inside: avoid; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background: #f5f5f5; }}
        blockquote {{ border-left: 3px solid #ddd; margin: 8px 0; padding-left: 12px; color: #666; page-break-inside: avoid; }}
        img {{ max-width: 100%; height: auto; page-break-inside: avoid; }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        html
    )
}

#[tauri::command]
pub async fn export_to_pdf(markdown: String, output_path: String) -> Result<(), CommandError> {
    let print_html = get_print_html(markdown);
    fs::write(&output_path, print_html)?;
    Ok(())
}