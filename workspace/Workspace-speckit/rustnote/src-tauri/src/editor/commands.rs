use crate::semantic::ast::{Position, SourceRange};

/// Core document editing commands
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

/// Unified editor command enum that encompasses ALL editor operations.
/// This is the PRIMARY dispatch mechanism for all editor operations.
/// All editor operations MUST go through this enum - no bypasses allowed.
#[derive(Debug, Clone)]
pub enum EditorCommand {
    // === Core Editing Commands (from Command enum) ===
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
    UndoReplace {
        range: SourceRange,
        old_text: String,
        new_text: String,
    },
    Format {
        range: SourceRange,
        format: FormatKind,
    },

    // === Search Commands ===
    Search {
        pattern: String,
        case_sensitive: bool,
        use_regex: bool,
    },
    FindNext {
        pattern: String,
        case_sensitive: bool,
        use_regex: bool,
        after_offset: usize,
    },
    FindPrevious {
        pattern: String,
        case_sensitive: bool,
        use_regex: bool,
        before_offset: usize,
    },
    ReplaceMatch {
        range: SourceRange,
        replacement: String,
    },
    ReplaceAll {
        pattern: String,
        case_sensitive: bool,
        use_regex: bool,
        replacement: String,
    },

    // === Transform Commands ===
    Transform {
        transform: crate::editor::transforms::Transform,
    },
}

/// Search result for editor search commands
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub matches: Vec<SearchMatch>,
    pub count: usize,
    pub success: bool,
    pub error: Option<String>,
}

/// A single search match with position information
#[derive(Debug, Clone)]
pub struct SearchMatch {
    pub start: usize,
    pub end: usize,
    pub text: String,
    pub line: u32,
    pub column: u32,
}

/// Result of executing an EditorCommand
#[derive(Debug, Clone)]
pub enum CommandResult {
    /// No result (for commands that just modify state)
    None,
    /// Search result
    Search(SearchResult),
    /// A single search match
    Match(Option<SearchMatch>),
    /// Content modification result
    Content {
        content: String,
        cursor_offset: usize,
    },
    /// Replace all result
    ReplaceAll {
        content: String,
        replacements: usize,
    },
}

impl EditorCommand {
    /// Create a search command
    pub fn search(pattern: String, case_sensitive: bool, use_regex: bool) -> Self {
        EditorCommand::Search {
            pattern,
            case_sensitive,
            use_regex,
        }
    }

    /// Create a find next command
    pub fn find_next(
        pattern: String,
        case_sensitive: bool,
        use_regex: bool,
        after_offset: usize,
    ) -> Self {
        EditorCommand::FindNext {
            pattern,
            case_sensitive,
            use_regex,
            after_offset,
        }
    }

    /// Create a find previous command
    pub fn find_previous(
        pattern: String,
        case_sensitive: bool,
        use_regex: bool,
        before_offset: usize,
    ) -> Self {
        EditorCommand::FindPrevious {
            pattern,
            case_sensitive,
            use_regex,
            before_offset,
        }
    }

    /// Create a replace match command
    pub fn replace_match(range: SourceRange, replacement: String) -> Self {
        EditorCommand::ReplaceMatch { range, replacement }
    }

    /// Create a replace all command
    pub fn replace_all(
        pattern: String,
        case_sensitive: bool,
        use_regex: bool,
        replacement: String,
    ) -> Self {
        EditorCommand::ReplaceAll {
            pattern,
            case_sensitive,
            use_regex,
            replacement,
        }
    }

    /// Create an insert command
    pub fn insert(pos: Position, text: &str) -> Self {
        EditorCommand::Insert {
            pos,
            text: text.to_string(),
        }
    }

    /// Create a delete command
    pub fn delete(range: SourceRange, deleted: &str) -> Self {
        EditorCommand::Delete {
            range,
            deleted: deleted.to_string(),
        }
    }

    /// Create a replace command
    pub fn replace(range: SourceRange, new_text: &str, old_text: &str) -> Self {
        EditorCommand::Replace {
            range,
            new_text: new_text.to_string(),
            old_text: old_text.to_string(),
        }
    }

    /// Create a format command
    pub fn format(range: SourceRange, format: FormatKind) -> Self {
        EditorCommand::Format { range, format }
    }

    /// Create a transform command
    pub fn transform(transform: crate::editor::transforms::Transform) -> Self {
        EditorCommand::Transform { transform }
    }

    /// Get the command name for logging/auditing purposes
    pub fn name(&self) -> &'static str {
        match self {
            EditorCommand::Insert { .. } => "Insert",
            EditorCommand::Delete { .. } => "Delete",
            EditorCommand::Replace { .. } => "Replace",
            EditorCommand::UndoReplace { .. } => "UndoReplace",
            EditorCommand::Format { .. } => "Format",
            EditorCommand::Search { .. } => "Search",
            EditorCommand::FindNext { .. } => "FindNext",
            EditorCommand::FindPrevious { .. } => "FindPrevious",
            EditorCommand::ReplaceMatch { .. } => "ReplaceMatch",
            EditorCommand::ReplaceAll { .. } => "ReplaceAll",
            EditorCommand::Transform { .. } => "Transform",
        }
    }

    /// Apply command to content and return the result
    /// This is the unified dispatch mechanism for all editor operations
    pub fn apply(&self, content: &str, cursor_offset: usize) -> (String, CommandResult) {
        match self {
            EditorCommand::Insert { pos, text } => {
                let mut result = content.to_string();
                let offset = pos.offset.min(result.len());
                result.insert_str(offset, text);
                let new_content = result.clone();
                let new_cursor = offset + text.len();
                (
                    result,
                    CommandResult::Content {
                        content: new_content,
                        cursor_offset: new_cursor,
                    },
                )
            }
            EditorCommand::Delete { range, .. } => {
                let mut result = content.to_string();
                let start = range.start.offset.min(result.len());
                let end = range.end.offset.min(result.len());
                if start < end {
                    result.drain(start..end);
                }
                let new_content = result.clone();
                (
                    result,
                    CommandResult::Content {
                        content: new_content,
                        cursor_offset: start,
                    },
                )
            }
            EditorCommand::Replace {
                range, new_text, ..
            } => {
                let mut result = content.to_string();
                let start = range.start.offset.min(result.len());
                let end = range.end.offset.min(result.len());
                result.drain(start..end);
                result.insert_str(start, new_text);
                let new_content = result.clone();
                let new_cursor = start + new_text.len();
                (
                    result,
                    CommandResult::Content {
                        content: new_content,
                        cursor_offset: new_cursor,
                    },
                )
            }
            EditorCommand::UndoReplace {
                range,
                old_text,
                new_text,
            } => {
                let mut result = content.to_string();
                let start = range.start.offset.min(result.len());
                let end = (start + new_text.len()).min(result.len());
                result.drain(start..end);
                result.insert_str(start, old_text);
                let new_content = result.clone();
                let new_cursor = start + old_text.len();
                (
                    result,
                    CommandResult::Content {
                        content: new_content,
                        cursor_offset: new_cursor,
                    },
                )
            }
            EditorCommand::Format { range, format } => {
                let mut result = content.to_string();
                let start = range.start.offset.min(result.len());
                let end = range.end.offset.min(result.len());
                let (prefix, suffix) = format.markdown_syntax();
                result.insert_str(end, suffix);
                result.insert_str(start, prefix);
                let new_content = result.clone();
                let new_cursor = end + suffix.len();
                (
                    result,
                    CommandResult::Content {
                        content: new_content,
                        cursor_offset: new_cursor,
                    },
                )
            }
            EditorCommand::Search {
                pattern,
                case_sensitive,
                use_regex,
            } => {
                let engine = crate::editor::search::SearchEngine::new();
                let options = crate::editor::search::SearchOptions {
                    case_sensitive: *case_sensitive,
                    use_regex: *use_regex,
                };
                let result = engine.search(content, pattern, &options);
                let search_result = SearchResult {
                    matches: result
                        .matches
                        .into_iter()
                        .map(|m| SearchMatch {
                            start: m.start,
                            end: m.end,
                            text: m.text,
                            line: m.line,
                            column: m.column,
                        })
                        .collect(),
                    count: result.count,
                    success: result.success,
                    error: result.error,
                };
                (content.to_string(), CommandResult::Search(search_result))
            }
            EditorCommand::FindNext {
                pattern,
                case_sensitive,
                use_regex,
                after_offset,
            } => {
                let engine = crate::editor::search::SearchEngine::new();
                let options = crate::editor::search::SearchOptions {
                    case_sensitive: *case_sensitive,
                    use_regex: *use_regex,
                };
                let result = engine.find_next(content, pattern, &options, *after_offset);
                let search_match = result.map(|m| SearchMatch {
                    start: m.start,
                    end: m.end,
                    text: m.text,
                    line: m.line,
                    column: m.column,
                });
                (content.to_string(), CommandResult::Match(search_match))
            }
            EditorCommand::FindPrevious {
                pattern,
                case_sensitive,
                use_regex,
                before_offset,
            } => {
                let engine = crate::editor::search::SearchEngine::new();
                let options = crate::editor::search::SearchOptions {
                    case_sensitive: *case_sensitive,
                    use_regex: *use_regex,
                };
                let result = engine.find_previous(content, pattern, &options, *before_offset);
                let search_match = result.map(|m| SearchMatch {
                    start: m.start,
                    end: m.end,
                    text: m.text,
                    line: m.line,
                    column: m.column,
                });
                (content.to_string(), CommandResult::Match(search_match))
            }
            EditorCommand::ReplaceMatch { range, replacement } => {
                let mut result = content.to_string();
                let start = range.start.offset.min(result.len());
                let end = range.end.offset.min(result.len());
                result.drain(start..end);
                result.insert_str(start, replacement);
                let new_content = result.clone();
                let new_cursor = start + replacement.len();
                (
                    result,
                    CommandResult::Content {
                        content: new_content,
                        cursor_offset: new_cursor,
                    },
                )
            }
            EditorCommand::ReplaceAll {
                pattern,
                case_sensitive,
                use_regex,
                replacement,
            } => {
                let options = crate::editor::search::SearchOptions {
                    case_sensitive: *case_sensitive,
                    use_regex: *use_regex,
                };
                let result = crate::editor::search::SearchEngine::replace_all(
                    content,
                    pattern,
                    &options,
                    replacement,
                );
                let new_content = result
                    .matches
                    .first()
                    .map(|m| m.text.clone())
                    .unwrap_or_else(|| content.to_string());
                (
                    new_content.clone(),
                    CommandResult::ReplaceAll {
                        content: new_content,
                        replacements: result.count,
                    },
                )
            }
            EditorCommand::Transform { transform } => {
                let engine = crate::editor::transforms::TransformEngine::new();
                let result = engine.apply(transform, content, cursor_offset, None);
                (
                    result.content.clone(),
                    CommandResult::Content {
                        content: result.content,
                        cursor_offset: result.cursor_offset,
                    },
                )
            }
        }
    }
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
