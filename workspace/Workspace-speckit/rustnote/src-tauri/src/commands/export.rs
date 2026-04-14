use crate::commands::CommandError;
use crate::model::export::{HtmlExportMode, HtmlExportOptions, PdfExportOptions};
use crate::parser::MarkdownParser;
use crate::parser::syntax::SyntaxHighlighter;
use base64::Engine;
use printpdf::*;
use std::fs;
use std::io::BufWriter;
use std::path::Path;

const DEFAULT_CSS: &str = r#"body { font-family: -apple-system, system-ui, sans-serif; max-width: 800px; margin: 40px auto; padding: 20px; }
pre { background: #f5f5f5; padding: 16px; border-radius: 4px; overflow-x: auto; }
code { background: #f5f5f5; padding: 2px 6px; border-radius: 3px; }
table { border-collapse: collapse; width: 100%; }
th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
blockquote { border-left: 4px solid #ddd; margin: 0; padding-left: 16px; color: #666; }"#;

#[tauri::command]
pub async fn export_to_html(
    markdown: String,
    output_path: String,
    options: HtmlExportOptions,
) -> Result<(), CommandError> {
    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(&markdown);

    let output_dir = Path::new(&output_path).parent().unwrap_or(Path::new("."));
    let (processed_html, css_link) = match &options.mode {
        HtmlExportMode::Linked { assets_dir } => {
            let assets_path = output_dir.join(assets_dir);
            fs::create_dir_all(&assets_path)?;

            let css_path = assets_path.join("styles.css");
            fs::write(&css_path, DEFAULT_CSS)?;

            let processed = process_images_linked(&html, &assets_path)?;
            let css_href = format!("{}/styles.css", assets_dir);
            (processed, Some(css_href))
        }
        HtmlExportMode::Inline => {
            let processed = process_images_inline(&html)?;
            (processed, None)
        }
    };

    let head_section = if options.embed_css {
        if let Some(ref css_href) = css_link {
            format!(r#"<link rel="stylesheet" href="{}">"#, css_href)
        } else {
            format!(r#"<style>{}</style>"#, DEFAULT_CSS)
        }
    } else if let Some(ref css_href) = css_link {
        format!(r#"<link rel="stylesheet" href="{}">"#, css_href)
    } else {
        format!(r#"<style>{}</style>"#, DEFAULT_CSS)
    };

    let full_html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Exported Document</title>
    {}
</head>
<body>
{}
</body>
</html>"#,
        head_section, processed_html
    );

    fs::write(&output_path, full_html)?;
    Ok(())
}

pub fn process_images_linked(html: &str, assets_dir: &Path) -> Result<String, CommandError> {
    let mut result = html.to_string();
    let mut img_index = 0;

    while let Some(img_start) = result.find("<img ") {
        if let Some(src_start) = result[img_start..].find("src=\"") {
            let src_start = img_start + src_start + 5;
            if let Some(src_end) = result[src_start..].find('"') {
                let original_src = &result[src_start..src_start + src_end];
                let extension = Path::new(original_src)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("png");
                let filename = format!("image_{}.{}", img_index, extension);
                let target_path = assets_dir.join(&filename);

                if let Ok(data) = fs::read(original_src) {
                    fs::write(&target_path, &data)?;
                    result = format!(
                        "{}{}",
                        &result[..src_start],
                        format!("{}/{}", assets_dir.file_name().unwrap_or_default().to_string_lossy(), filename)
                    );
                }
                img_index += 1;
            }
        }

        if let Some(next_tag) = result[img_start + 5..].find("<img ") {
            result = format!("{}{}", &result[..img_start + 5], &result[img_start + 5..]);
        } else {
            break;
        }
    }

    Ok(result)
}

pub fn process_images_inline(html: &str) -> Result<String, CommandError> {
    let mut result = html.to_string();
    let mut offset = 0;

    while let Some(img_start) = result[offset..].find("<img ") {
        let actual_start = offset + img_start;
        if let Some(src_start) = result[actual_start..].find("src=\"") {
            let src_pos = actual_start + src_start + 5;
            if let Some(src_end) = result[src_pos..].find('"') {
                let original_src = &result[src_pos..src_pos + src_end];
                if let Ok(data) = fs::read(original_src) {
                    let mime_type = guess_mime_type(original_src);
                    let base64_data = base64::engine::general_purpose::STANDARD.encode(&data);
                    let embedded = format!("data:{};base64,{}", mime_type, base64_data);
                    result = format!("{}{}", &result[..src_pos], embedded);
                    offset = src_pos + embedded.len();
                    continue;
                }
            }
        }
        offset = actual_start + 5;
    }

    Ok(result)
}

pub fn guess_mime_type(path: &str) -> &'static str {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "bmp" => "image/bmp",
        _ => "application/octet-stream",
    }
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
            
            for table_line in &table_lines {
                if !table_line.trim().starts_with('|') {
                    continue;
                }
            }
            
            let required_height = table_lines.iter().filter(|l| l.trim().starts_with('|')).count() as f32 * line_height * 1.5 + 20.0;
            if table_y < bottom_mm + required_height {
                let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
                table_y = page_height - top_mm;
                page_count += 1;
            }
            
            table_y = render_table_with_borders(
                &table_lines,
                &current_layer,
                &font,
                &font_bold,
                left_mm,
                content_width,
                table_y,
                bottom_mm,
                line_height,
            );
            
            y_position = table_y;
            continue;
        }

        if line.starts_with("<blockquote") {
            let nesting_depth = line.matches("<blockquote").count();
            let indent_mm = (nesting_depth as f32) * 15.0;
            
            let quote_text = strip_html_tags(line);
            let wrapped = wrap_text(&quote_text, ((content_width * 10.0 - indent_mm * 10.0) / (font_size * 0.6)) as usize);
            
            let required_height = wrapped.len() as f32 * line_height + font_size * 0.3;
            if y_position < bottom_mm + required_height {
                let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
                y_position = page_height - top_mm;
                page_count += 1;
            }

            let blockquote_line_height = wrapped.len() as f32 * line_height + font_size * 0.3;
            current_layer.set_outline_color(Color::Rgb(Rgb::new(0.6, 0.6, 0.6, None)));
            current_layer.set_outline_thickness(2.0);
            let line_y_start = y_position;
            let line_y_end = y_position - blockquote_line_height;
            let border_x = left_mm + indent_mm - 3.0;
            let line = Line {
                points: vec![
                    (Point::new(Mm(border_x), Mm(line_y_start)), false),
                    (Point::new(Mm(border_x), Mm(line_y_end)), false)
                ],
                is_closed: false,
            };
            current_layer.add_line(line);

            y_position -= font_size * 0.3;
            for chunk in wrapped {
                current_layer.use_text(chunk, font_size, Mm(left_mm + indent_mm), Mm(y_position), &font);
                y_position -= line_height;
            }

            i += 1;
            continue;
        }

        if line.starts_with("<li") || line.starts_with("<ul") || line.starts_with("<ol") {
            let item_text = strip_html_tags(line);
            
            let checkbox_symbol = if line.contains("type=\"checkbox\"") {
                if line.contains("checked") {
                    "[☑]" 
                } else {
                    "[☐]"
                }
            } else {
                "•"
            };
            
            let wrapped = wrap_text(&item_text, ((content_width * 10.0) / (font_size * 0.6)) as usize);
            
            let required_height = wrapped.len() as f32 * line_height;
            if y_position < bottom_mm + required_height {
                let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
                y_position = page_height - top_mm;
                page_count += 1;
            }

            for chunk in wrapped {
                current_layer.use_text(format!("  {} {}", checkbox_symbol, chunk), font_size, Mm(left_mm), Mm(y_position), &font);
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

/// Renders a table with borders in the PDF
fn render_table_with_borders(
    table_lines: &[&str],
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    left_mm: f32,
    content_width: f32,
    y_position: f32,
    bottom_mm: f32,
    line_height: f32,
) -> f32 {
    if table_lines.is_empty() {
        return y_position;
    }

    let col_count = table_lines[0].split('|').filter(|s| !s.trim().is_empty()).count();
    if col_count == 0 {
        return y_position;
    }

    let col_width = content_width / col_count as f32;
    let row_height = line_height * 1.5;
    let border_thickness = 0.5;

    // Set border color to light gray
    layer.set_outline_color(Color::Rgb(Rgb::new(0.8, 0.8, 0.8, None)));
    layer.set_outline_thickness(border_thickness);

    let mut table_y = y_position;
    table_y -= 10.0; // Top padding

    for (row_idx, table_line) in table_lines.iter().enumerate() {
        if !table_line.trim().starts_with('|') {
            continue;
        }

        let cells: Vec<&str> = table_line.split('|').filter(|s| !s.trim().is_empty()).collect();
        let mut x_pos = left_mm;

        // Draw top border for first row
        if row_idx == 0 {
            let rect = Rect::new(Mm(left_mm), Mm(table_y), Mm(left_mm + content_width), Mm(table_y - row_height));
            layer.add_rect(rect);
        }

        // Draw cell borders and text
        for (cell_idx, cell) in cells.iter().enumerate() {
            let cell_text = strip_html_tags(cell).trim().to_string();
            
            // Draw cell border (vertical lines between cells)
            let cell_x2 = x_pos + col_width;
            
            // Draw left border of cell (or outer border for first cell)
            let left_border_x = if cell_idx == 0 { left_mm } else { x_pos };
            let rect = Rect::new(
                Mm(left_border_x), 
                Mm(table_y), 
                Mm(left_border_x + 0.1), 
                Mm(table_y - row_height)
            );
            layer.add_rect(rect);

            // Draw bottom border of row
            let rect_bottom = Rect::new(
                Mm(left_mm), 
                Mm(table_y - row_height), 
                Mm(left_mm + content_width), 
                Mm(table_y - row_height - 0.1)
            );
            layer.add_rect(rect_bottom);

            // Use bold font for header row
            let cell_font = if row_idx == 0 { &font_bold } else { &font };
            
            // Draw cell text with padding
            let text_x = x_pos + 2.0;
            let text_y = table_y - row_height + 4.0;
            layer.use_text(cell_text, 10.0, Mm(text_x), Mm(text_y), cell_font);

            x_pos += col_width;
        }

        table_y -= row_height;
    }

    // Draw outer border (right side)
    let rect_right = Rect::new(
        Mm(left_mm + content_width - 0.1), 
        Mm(y_position), 
        Mm(left_mm + content_width), 
        Mm(table_y)
    );
    layer.add_rect(rect_right);

    table_y
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