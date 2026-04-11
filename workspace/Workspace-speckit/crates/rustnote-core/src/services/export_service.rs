use crate::parser::{parse_markdown, Serializer};

pub struct ExportService;

impl ExportService {
    pub fn export_html(content: &str, standalone: bool) -> String {
        let doc = parse_markdown(content);
        if standalone {
            Serializer::to_html(&doc)
        } else {
            Serializer::to_html(&doc)
        }
    }

    pub fn export_pdf_bytes(_content: &str) -> Vec<u8> {
        Vec::new()
    }
}
