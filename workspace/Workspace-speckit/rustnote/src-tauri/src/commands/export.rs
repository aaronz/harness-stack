use crate::commands::CommandError;
use crate::model::export::PdfExportOptions;
use crate::parser::MarkdownParser;
use crate::parser::syntax::SyntaxHighlighter;
use printpdf::*;
use std::fs;
use std::io::BufWriter;

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

#[tauri::command]
pub async fn export_to_pdf_native(
    markdown: String,
    output_path: String,
    options: PdfExportOptions,
) -> Result<(), CommandError> {
    let (page_width, page_height) = options.page_size.dimensions();
    let (top_mm, right_mm, bottom_mm, left_mm) = options.margins.as_tuple();

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(&markdown);
    let syntax_highlighter = SyntaxHighlighter::new();

    let (doc, page1, layer1) = PdfDocument::new(
        "RustNote Export",
        Mm(page_width),
        Mm(page_height),
        "Layer 1",
    );

    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).map_err(|e| {
        CommandError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).map_err(|e| {
        CommandError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;
    let font_mono = doc.add_builtin_font(BuiltinFont::Courier).map_err(|e| {
        CommandError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    })?;

    let content_width = page_width - left_mm - right_mm;
    let font_size = 11.0;
    let line_height = font_size * 1.4;

    let mut y_position = page_height - top_mm;
    let mut page_count = 1usize;

    let lines: Vec<&str> = html.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() || line == "<p>" || line == "</p>" {
            y_position -= line_height * 0.5;
            i += 1;
            continue;
        }

        if line.starts_with("<h1") || line.starts_with("<h2") || line.starts_with("<h3")
            || line.starts_with("<h4") || line.starts_with("<h5") || line.starts_with("<h6")
        {
            let level = line.chars().nth(2).unwrap_or('1').to_digit(10).unwrap_or(1) as usize;
            let size = 24.0 - (level as f32 * 2.0);
            let text = strip_html_tags(line);

            let chars_per_line = ((content_width * 10.0) / (size * 0.5)) as usize;
            let wrapped = wrap_text(&text, chars_per_line);

            let required_height = wrapped.len() as f32 * size * 1.2 + size * 0.3;
            if y_position < bottom_mm + required_height {
                let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
                y_position = page_height - top_mm;
                page_count += 1;
            }

            y_position -= size * 0.3;

            for chunk in wrapped {
                current_layer.use_text(chunk, size, Mm(left_mm), Mm(y_position), &font_bold);
                y_position -= size * 1.2;
            }

            i += 1;
            continue;
        }

        if line.starts_with("<pre") || (line.starts_with("<code") && line.contains("class=")) {
            let (code_content, language) = extract_code_block_with_lang(&lines, &mut i);
            let highlighted = if !language.is_empty() {
                syntax_highlighter.highlight_html(&code_content, &language)
            } else {
                syntax_highlighter.highlight_html(&code_content, "text")
            };
            let display_code = strip_html_tags(&highlighted);
            
            y_position -= font_size * 0.5;

            let chars_per_line = ((content_width * 10.0) / (font_size * 0.6)) as usize;
            let wrapped = wrap_text(&code_content, chars_per_line);

            let required_height = wrapped.len() as f32 * line_height * 0.9 + font_size;
            if y_position < bottom_mm + required_height {
                let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
                y_position = page_height - top_mm;
                page_count += 1;
            }

            for chunk in wrapped {
                current_layer.use_text(chunk, font_size * 0.85, Mm(left_mm), Mm(y_position), &font_mono);
                y_position -= line_height * 0.9;
            }

            i += 1;
            continue;
        }

        if line.starts_with("<table") {
            let table_end = find_table_end(&lines, i);
            let table_lines: Vec<&str> = lines[i..table_end].to_vec();
            i = table_end;
            
            let mut table_y = y_position;
            let mut table_lines_rendered = 0usize;
            
            for table_line in &table_lines {
                if !table_line.trim().starts_with('|') {
                    continue;
                }
                table_lines_rendered += 1;
            }
            
            let required_height = table_lines_rendered as f32 * line_height * 1.5 + 20.0;
            if table_y < bottom_mm + required_height {
                let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
                table_y = page_height - top_mm;
                page_count += 1;
            }
            
            table_y -= 10.0;
            
            let col_count = table_lines[0].split('|').filter(|s| !s.trim().is_empty()).count();
            if col_count > 0 {
                let col_width = content_width / col_count as f32;
                let row_height = line_height * 1.5;
                
                for tl in &table_lines {
                    if !tl.trim().starts_with('|') {
                        continue;
                    }
                    
                    let cells: Vec<&str> = tl.split('|').filter(|s| !s.trim().is_empty()).collect();
                    let mut x_pos = left_mm;
                    
                    for cell in cells {
                        let cell_text = strip_html_tags(cell).trim().to_string();
                        if table_y < bottom_mm + row_height {
                            break;
                        }
                        current_layer.use_text(cell_text, 10.0, Mm(x_pos), Mm(table_y), &font);
                        x_pos += col_width;
                    }
                    
                    table_y -= row_height;
                }
            }
            
            y_position = table_y;
            continue;
        }

        if line.starts_with("<blockquote") {
            let quote_text = strip_html_tags(line);
            let wrapped = wrap_text(&quote_text, ((content_width * 10.0) / (font_size * 0.6)) as usize);
            
            let required_height = wrapped.len() as f32 * line_height + font_size * 0.3;
            if y_position < bottom_mm + required_height {
                let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
                y_position = page_height - top_mm;
                page_count += 1;
            }

            y_position -= font_size * 0.3;
            for chunk in wrapped {
                current_layer.use_text(chunk, font_size, Mm(left_mm + 10.0), Mm(y_position), &font);
                y_position -= line_height;
            }

            i += 1;
            continue;
        }

        if line.starts_with("<li") || line.starts_with("<ul") || line.starts_with("<ol") {
            let item_text = strip_html_tags(line);
            let wrapped = wrap_text(&item_text, ((content_width * 10.0) / (font_size * 0.6)) as usize);
            
            let required_height = wrapped.len() as f32 * line_height;
            if y_position < bottom_mm + required_height {
                let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
                y_position = page_height - top_mm;
                page_count += 1;
            }

            for chunk in wrapped {
                current_layer.use_text(format!("  • {}", chunk), font_size, Mm(left_mm), Mm(y_position), &font);
                y_position -= line_height;
            }

            i += 1;
            continue;
        }

        if line.starts_with("<img") {
            if y_position < bottom_mm + line_height * 5.0 {
                let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
                y_position = page_height - top_mm;
                page_count += 1;
            }
            y_position -= line_height;
            i += 1;
            continue;
        }

        if line.starts_with("<") {
            i += 1;
            continue;
        }

        let text = strip_html_tags(line);
        if !text.is_empty() {
            let chars_per_line = ((content_width * 10.0) / (font_size * 0.5)) as usize;
            let wrapped = wrap_text(&text, chars_per_line);
            
            let required_height = wrapped.len() as f32 * line_height;
            if y_position < bottom_mm + required_height {
                let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
                y_position = page_height - top_mm;
                page_count += 1;
            }

            for chunk in wrapped {
                current_layer.use_text(chunk, font_size, Mm(left_mm), Mm(y_position), &font);
                y_position -= line_height;
            }
        }

        i += 1;
    }

    let file = fs::File::create(&output_path)?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).map_err(|e| CommandError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(())
}

fn extract_code_block_with_lang<'a>(lines: &'a [&str], index: &mut usize) -> (String, String) {
    let mut content = String::new();
    let mut language = String::new();
    
    let first_line = lines[*index];
    if let Some(class_start) = first_line.find("class=\"") {
        if let Some(class_end) = first_line[class_start..].find('"') {
            let class_value = &first_line[class_start + 7..class_start + class_end];
            if let Some(lang_start) = class_value.find("language-") {
                language = class_value[lang_start + 10..].split_whitespace().next().unwrap_or("").to_string();
            } else if let Some(lang_start) = class_value.find("lang-") {
                language = class_value[lang_start + 5..].split_whitespace().next().unwrap_or("").to_string();
            }
        }
    }
    
    *index += 1;
    
    while *index < lines.len() {
        let line = lines[*index];
        if line.contains("</pre>") || line.contains("</code>") {
            break;
        }
        if !content.is_empty() {
            content.push('\n');
        }
        content.push_str(strip_html_tags(line).trim());
        *index += 1;
    }
    
    (content, language)
}

fn find_table_end(lines: &[&str], start: usize) -> usize {
    let mut depth = 0;
    for i in start..lines.len() {
        if lines[i].contains("<table") {
            depth += 1;
        }
        if lines[i].contains("</table>") {
            depth -= 1;
            if depth == 0 {
                return i + 1;
            }
        }
    }
    lines.len()
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    let mut in_entity = false;
    let mut entity_buf = String::new();

    for ch in html.chars() {
        if ch == '<' {
            in_tag = true;
            continue;
        }
        if ch == '>' && in_tag {
            in_tag = false;
            continue;
        }
        if in_tag {
            continue;
        }

        if ch == '&' {
            in_entity = true;
            entity_buf.clear();
            continue;
        }
        if in_entity {
            entity_buf.push(ch);
            if ch == ';' {
                in_entity = false;
                result.push(match entity_buf.as_str() {
                    "amp" => '&',
                    "lt" => '<',
                    "gt" => '>',
                    "quot" => '"',
                    "apos" => '\'',
                    "nbsp" => ' ',
                    _ => {
                        if let Ok(num) = entity_buf.trim_end_matches(';').parse::<u32>() {
                            char::from_u32(num).unwrap_or(' ')
                        } else {
                            ' '
                        }
                    }
                });
                entity_buf.clear();
            }
            continue;
        }

        result.push(ch);
    }

    result
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    if text.len() <= max_chars {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 <= max_chars {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            if !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
            }
            if word.len() > max_chars {
                let mut remaining = word;
                while remaining.len() > max_chars {
                    lines.push(remaining[..max_chars].to_string());
                    remaining = &remaining[max_chars..];
                }
                current_line.push_str(remaining);
            } else {
                current_line.push_str(word);
            }
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

fn extract_pre_content(lines: &[&str], index: &mut usize) -> String {
    let mut content = String::new();
    *index += 1;

    while *index < lines.len() {
        let line = lines[*index];
        if line.contains("</pre>") || line.contains("</code>") {
            break;
        }
        if !content.is_empty() {
            content.push(' ');
        }
        content.push_str(strip_html_tags(line).trim());
        *index += 1;
    }

    content
}

fn extract_and_render_table(
    lines: &[&str],
    index: &mut usize,
    layer: &PdfLayerReference,
    font: IndirectFontRef,
    left_mm: f32,
    content_width: f32,
    y_position: &mut f32,
    bottom_mm: f32,
    line_height: f32,
) -> Option<f32> {
    let mut table_lines = Vec::new();

    while *index < lines.len() {
        let line = lines[*index];
        if line.contains("</table>") {
            *index += 1;
            break;
        }
        if line.trim().starts_with('|') {
            table_lines.push(line.to_string());
        }
        *index += 1;
    }

    if table_lines.len() < 2 {
        return Some(*y_position);
    }

    let mut y_pos = *y_position;
    y_pos -= 10.0;

    let col_count = table_lines[0].split('|').filter(|s| !s.trim().is_empty()).count();
    if col_count == 0 {
        return Some(*y_position);
    }

    let col_width = content_width / col_count as f32;
    let row_height = line_height * 1.5;

    for (_row_idx, table_line) in table_lines.iter().enumerate() {
        let cells: Vec<&str> = table_line.split('|').filter(|s| !s.trim().is_empty()).collect();

        let mut x_pos = left_mm;

        for cell in cells {
            let cell_text = strip_html_tags(cell).trim().to_string();
            if y_pos < bottom_mm + row_height {
                return Some(y_pos);
            }
            layer.use_text(cell_text, 10.0, Mm(x_pos), Mm(y_pos), &font);
            x_pos += col_width;
        }

        y_pos -= row_height;
    }

    *y_position = y_pos;
    Some(y_pos)
}