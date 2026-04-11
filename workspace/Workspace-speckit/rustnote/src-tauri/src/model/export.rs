use serde::{Deserialize, Serialize};

/// Options for PDF export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfExportOptions {
    pub page_size: PdfPageSize,
    pub margins: PdfMargins,
    pub embed_images: bool,
}

impl Default for PdfExportOptions {
    fn default() -> Self {
        Self {
            page_size: PdfPageSize::A4,
            margins: PdfMargins::default(),
            embed_images: true,
        }
    }
}

/// Page size for PDF export
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum PdfPageSize {
    A4,
    Letter,
    Legal,
    Custom { width_mm: f32, height_mm: f32 },
}

impl PdfPageSize {
    /// Returns (width_mm, height_mm) for the page size
    pub fn dimensions(&self) -> (f32, f32) {
        match self {
            PdfPageSize::A4 => (210.0, 297.0),
            PdfPageSize::Letter => (215.9, 279.4),
            PdfPageSize::Legal => (215.9, 355.6),
            PdfPageSize::Custom {
                width_mm,
                height_mm,
            } => (*width_mm, *height_mm),
        }
    }
}

/// Margins for PDF export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMargins {
    pub top_mm: f32,
    pub right_mm: f32,
    pub bottom_mm: f32,
    pub left_mm: f32,
}

impl Default for PdfMargins {
    fn default() -> Self {
        Self {
            top_mm: 20.0,
            right_mm: 20.0,
            bottom_mm: 20.0,
            left_mm: 20.0,
        }
    }
}

impl PdfMargins {
    /// Returns (top, right, bottom, left) in the same order
    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.top_mm, self.right_mm, self.bottom_mm, self.left_mm)
    }
}
