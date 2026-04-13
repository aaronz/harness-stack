use crate::semantic::ast::{Position, SourceRange};

#[derive(Debug, Clone)]
pub enum Command {
    Insert {
        pos: Position,
        text: String,
    },
    Delete {
        range: SourceRange,
        deleted: String,
    },
    Replace {
        range: SourceRange,
        new_text: String,
        old_text: String,
    },
    /// Inverse of Replace: removes new_text that was inserted and inserts old_text that was deleted
    /// This is needed because Replace is a compound operation (delete + insert) and its inverse
    /// must also be compound (delete the new_text, insert the old_text)
    UndoReplace {
        range: SourceRange,
        old_text: String,
        new_text: String,
    },
    Format {
        range: SourceRange,
        format: FormatKind,
    },
}

#[derive(Debug, Clone)]
pub enum FormatKind {
    Bold,
    Italic,
    Strikethrough,
    Code,
    Link { href: String },
    Image { src: String },
}

impl Command {
    pub fn insert(pos: Position, text: &str) -> Self {
        Command::Insert {
            pos,
            text: text.to_string(),
        }
    }

    pub fn delete(range: SourceRange, deleted: &str) -> Self {
        Command::Delete {
            range,
            deleted: deleted.to_string(),
        }
    }

    pub fn replace(range: SourceRange, new_text: &str, old_text: &str) -> Self {
        Command::Replace {
            range,
            new_text: new_text.to_string(),
            old_text: old_text.to_string(),
        }
    }

    pub fn apply(&self, content: &mut String) {
        match self {
            Command::Insert { pos, text } => {
                let offset = pos.offset.min(content.len());
                content.insert_str(offset, text);
            }
            Command::Delete { range, .. } => {
                let start = range.start.offset.min(content.len());
                let end = range.end.offset.min(content.len());
                if start < end {
                    content.drain(start..end);
                }
            }
            Command::Replace {
                range, new_text, ..
            } => {
                let start = range.start.offset.min(content.len());
                let end = range.end.offset.min(content.len());
                content.drain(start..end);
                content.insert_str(start, new_text);
            }
            Command::UndoReplace {
                range,
                old_text,
                new_text,
            } => {
                let start = range.start.offset.min(content.len());
                let end = (start + new_text.len()).min(content.len());
                content.drain(start..end);
                content.insert_str(start, old_text);
            }
            Command::Format { range, format } => {
                let start = range.start.offset.min(content.len());
                let end = range.end.offset.min(content.len());
                let (prefix, suffix) = format.markdown_syntax();
                content.insert_str(end, suffix);
                content.insert_str(start, prefix);
            }
        }
    }

    pub fn inverse(&self) -> Option<Command> {
        match self.clone() {
            Command::Insert { pos, text } => Some(Command::delete(
                SourceRange::new(
                    pos,
                    Position::new(pos.offset + text.len(), pos.line, pos.column),
                ),
                &text,
            )),
            Command::Delete { range, deleted } => Some(Command::insert(range.start, &deleted)),
            Command::Replace {
                range,
                new_text,
                old_text,
                ..
            } => Some(Command::UndoReplace {
                range: range.clone(),
                old_text: old_text.clone(),
                new_text: new_text.clone(),
            }),
            Command::Format { range, format } => {
                let (prefix, suffix) = format.markdown_syntax();
                let start = range.start.offset;
                let end = range.end.offset;
                if end > start + prefix.len() + suffix.len() {
                    let inner_start = start + prefix.len();
                    let inner_end = end - suffix.len();
                    Some(Command::delete(
                        SourceRange::new(
                            Position::new(
                                inner_start,
                                range.start.line,
                                range.start.column + prefix.len() as u32,
                            ),
                            Position::new(
                                inner_end,
                                range.end.line,
                                range.end.column - suffix.len() as u32,
                            ),
                        ),
                        "",
                    ))
                } else {
                    None
                }
            }
            // UndoReplace is the inverse of Replace - it cannot be further inverted
            // because we've lost the original new_text content
            Command::UndoReplace { .. } => None,
        }
    }
}

impl FormatKind {
    pub fn markdown_syntax(&self) -> (&str, &str) {
        match self {
            FormatKind::Bold => ("**", "**"),
            FormatKind::Italic => ("*", "*"),
            FormatKind::Strikethrough => ("~~", "~~"),
            FormatKind::Code => ("`", "`"),
            FormatKind::Link { .. } => ("[", "](url)"),
            FormatKind::Image { .. } => ("![", "](url)"),
        }
    }
}
