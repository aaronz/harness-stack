use crate::model::export::{HtmlExportOptions, PdfExportOptions};
use crate::parser::syntax::SyntaxHighlighter;
use crate::parser::MarkdownParser;
use base64::Engine;
use printpdf::*;
use std::fs;
use std::io::BufWriter;
use std::path::Path;

pub type ExportResult<T> = Result<T, ExportServiceError>;

#[derive(Debug, Clone)]
pub enum ExportServiceError {
    Io(String),
    Parse(String),
    Pdf(String),
}

impl std::error::Error for ExportServiceError {}

impl std::fmt::Display for ExportServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportServiceError::Io(s) => write!(f, "IO error: {}", s),
            ExportServiceError::Parse(s) => write!(f, "Parse error: {}", s),
            ExportServiceError::Pdf(s) => write!(f, "PDF error: {}", s),
        }
    }
}

impl From<std::io::Error> for ExportServiceError {
    fn from(e: std::io::Error) -> Self {
        ExportServiceError::Io(e.to_string())
    }
}

pub trait ExportServiceTrait: Send + Sync {
    fn export_to_html(
        &self,
        markdown: &str,
        output_path: &str,
        options: HtmlExportOptions,
    ) -> ExportResult<()>;
    fn export_to_pdf(&self, markdown: &str, output_path: &str) -> ExportResult<()>;
    fn export_to_pdf_with_options(
        &self,
        markdown: &str,
        output_path: &str,
        options: PdfExportOptions,
    ) -> ExportResult<()>;
    fn get_print_html(&self, markdown: &str) -> String;
}

pub struct ExportService;

impl ExportService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ExportService {
    fn default() -> Self {
        Self::new()
    }
}

const DEFAULT_CSS: &str = r#"body { font-family: -apple-system, system-ui, sans-serif; max-width: 800px; margin: 40px auto; padding: 20px; }
pre { background: #f5f5f5; padding: 16px; border-radius: 4px; overflow-x: auto; }
code { background: #f5f5f5; padding: 2px 6px; border-radius: 3px; }
table { border-collapse: collapse; width: 100%; }
th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
blockquote { border-left: 4px solid #ddd; margin: 0; padding-left: 16px; color: #666; }"#;

impl ExportServiceTrait for ExportService {
    fn export_to_html(
        &self,
        markdown: &str,
        output_path: &str,
        options: HtmlExportOptions,
    ) -> ExportResult<()> {
        let parser = MarkdownParser::new();
        let html = parser.parse_to_html(markdown);

        let output_dir = Path::new(&output_path).parent().unwrap_or(Path::new("."));
        let (processed_html, css_link) = match &options.mode {
            crate::model::export::HtmlExportMode::Linked { assets_dir } => {
                let assets_path = output_dir.join(assets_dir);
                fs::create_dir_all(&assets_path)?;

                let css_path = assets_path.join("styles.css");
                fs::write(&css_path, DEFAULT_CSS)?;

                let processed = self.process_images_linked(&html, &assets_path)?;
                let css_href = format!("{}/styles.css", assets_dir);
                (processed, Some(css_href))
            }
            crate::model::export::HtmlExportMode::Inline => {
                let processed = self.process_images_inline(&html)?;
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

    fn export_to_pdf(&self, markdown: &str, output_path: &str) -> ExportResult<()> {
        let print_html = self.get_print_html(markdown);
        fs::write(&output_path, print_html)?;
        Ok(())
    }

    fn export_to_pdf_with_options(
        &self,
        markdown: &str,
        output_path: &str,
        options: PdfExportOptions,
    ) -> ExportResult<()> {
        let (page_width, page_height) = options.page_size.dimensions();
        let (top_mm, right_mm, bottom_mm, left_mm) = options.margins.as_tuple();

        let parser = MarkdownParser::new();
        let html = parser.parse_to_html(markdown);
        let syntax_highlighter = SyntaxHighlighter::new();

        let (doc, page1, layer1) = PdfDocument::new(
            "RustNote Export",
            Mm(page_width),
            Mm(page_height),
            "Layer 1",
        );

        let mut current_layer = doc.get_page(page1).get_layer(layer1);

        let font = doc
            .add_builtin_font(BuiltinFont::Helvetica)
            .map_err(|e| ExportServiceError::Pdf(e.to_string()))?;
        let font_bold = doc
            .add_builtin_font(BuiltinFont::HelveticaBold)
            .map_err(|e| ExportServiceError::Pdf(e.to_string()))?;
        let font_mono = doc
            .add_builtin_font(BuiltinFont::Courier)
            .map_err(|e| ExportServiceError::Pdf(e.to_string()))?;

        let content_width = page_width - left_mm - right_mm;
        let font_size = 11.0;
        let line_height = font_size * 1.4;

        let mut y_position = page_height - top_mm;

        let lines: Vec<&str> = html.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() || line == "<p>" || line == "</p>" {
                y_position -= line_height * 0.5;
                i += 1;
                continue;
            }

            if line.starts_with("<h1")
                || line.starts_with("<h2")
                || line.starts_with("<h3")
                || line.starts_with("<h4")
                || line.starts_with("<h5")
                || line.starts_with("<h6")
            {
                let level = line.chars().nth(2).unwrap_or('1').to_digit(10).unwrap_or(1) as usize;
                let size = 24.0 - (level as f32 * 2.0);
                let text = self.strip_html_tags(line);

                let chars_per_line = ((content_width * 10.0) / (size * 0.5)) as usize;
                let wrapped = self.wrap_text(&text, chars_per_line);

                let required_height = wrapped.len() as f32 * size * 1.2 + size * 0.3;
                if y_position < bottom_mm + required_height {
                    let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                    current_layer = doc.get_page(new_page).get_layer(new_layer);
                    y_position = page_height - top_mm;
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
                let (code_content, language) = self.extract_code_block_with_lang(&lines, &mut i);
                let highlighted = if !language.is_empty() {
                    syntax_highlighter.highlight_html(&code_content, &language)
                } else {
                    syntax_highlighter.highlight_html(&code_content, "text")
                };

                y_position -= font_size * 0.5;

                let chars_per_line = ((content_width * 10.0) / (font_size * 0.6)) as usize;
                let wrapped = self.wrap_text(&code_content, chars_per_line);

                let required_height = wrapped.len() as f32 * line_height * 0.9 + font_size;
                if y_position < bottom_mm + required_height {
                    let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                    current_layer = doc.get_page(new_page).get_layer(new_layer);
                    y_position = page_height - top_mm;
                }

                for chunk in wrapped {
                    current_layer.use_text(
                        chunk,
                        font_size * 0.85,
                        Mm(left_mm),
                        Mm(y_position),
                        &font_mono,
                    );
                    y_position -= line_height * 0.9;
                }

                i += 1;
                continue;
            }

            if line.starts_with("<") {
                i += 1;
                continue;
            }

            let text = self.strip_html_tags(line);
            if !text.is_empty() {
                let chars_per_line = ((content_width * 10.0) / (font_size * 0.5)) as usize;
                let wrapped = self.wrap_text(&text, chars_per_line);

                let required_height = wrapped.len() as f32 * line_height;
                if y_position < bottom_mm + required_height {
                    let (new_page, new_layer) = doc.add_page(Mm(page_width), Mm(page_height), "");
                    current_layer = doc.get_page(new_page).get_layer(new_layer);
                    y_position = page_height - top_mm;
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
        doc.save(&mut writer)
            .map_err(|e| ExportServiceError::Pdf(e.to_string()))?;

        Ok(())
    }

    fn get_print_html(&self, markdown: &str) -> String {
        let parser = MarkdownParser::new();
        let html = parser.parse_to_html(markdown);

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
}

impl ExportService {
    fn process_images_linked(
        &self,
        html: &str,
        assets_dir: &Path,
    ) -> Result<String, ExportServiceError> {
        let mut result = html.to_string();
        let mut img_index = 0usize;

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
                            format!(
                                "{}/{}",
                                assets_dir.file_name().unwrap_or_default().to_string_lossy(),
                                filename
                            )
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

    fn process_images_inline(&self, html: &str) -> Result<String, ExportServiceError> {
        let mut result = html.to_string();
        let mut offset = 0usize;

        while let Some(img_start) = result[offset..].find("<img ") {
            let actual_start = offset + img_start;
            if let Some(src_start) = result[actual_start..].find("src=\"") {
                let src_pos = actual_start + src_start + 5;
                if let Some(src_end) = result[src_pos..].find('"') {
                    let original_src = &result[src_pos..src_pos + src_end];
                    if let Ok(data) = fs::read(original_src) {
                        let mime_type = self.guess_mime_type(original_src);
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

    fn guess_mime_type(&self, path: &str) -> &'static str {
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

    fn extract_code_block_with_lang<'a>(
        &self,
        lines: &'a [&str],
        index: &mut usize,
    ) -> (String, String) {
        let mut content = String::new();
        let mut language = String::new();

        let first_line = lines[*index];

        // Find class attribute with proper bounds checking
        if let Some(class_attr_start) = first_line.find("class=\"") {
            // class_attr_start is position of 'c' in "class"
            // After "class=" there's the opening quote
            let open_quote = class_attr_start + 6; // position of opening "

            // Make sure we don't go past the string
            if open_quote < first_line.len() {
                let remaining = &first_line[open_quote..];
                if let Some(close_offset) = remaining.find('"') {
                    // Extract class value (between quotes)
                    if close_offset > 0 {
                        let class_value = &remaining[1..close_offset];

                        if let Some(lang_pos) = class_value.find("language-") {
                            language = class_value[lang_pos + 9..]
                                .split_whitespace()
                                .next()
                                .unwrap_or("")
                                .to_string();
                        } else if let Some(lang_pos) = class_value.find("lang-") {
                            language = class_value[lang_pos + 5..]
                                .split_whitespace()
                                .next()
                                .unwrap_or("")
                                .to_string();
                        }
                    }
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
            content.push_str(self.strip_html_tags(line).trim());
            *index += 1;
        }

        (content, language)
    }

    fn strip_html_tags(&self, html: &str) -> String {
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

    fn wrap_text(&self, text: &str, max_chars: usize) -> Vec<String> {
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
}
