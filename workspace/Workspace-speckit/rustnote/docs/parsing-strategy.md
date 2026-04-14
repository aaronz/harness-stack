# Markdown Parsing Strategy

## Overview

RustNote uses a dual-parser architecture to handle Markdown with GitHub Flavored Markdown (GFM) extensions. This document describes the parsing strategy, the roles of each parser, and how they work together.

## Parser Architecture

### 1. Tree-Sitter Parser (Structural Analysis)

**Purpose**: Incremental syntax tree parsing for editor operations

**Technology**: `tree-sitter` with `tree-sitter-markdown` grammar

**Key Capabilities**:
- Incremental parsing for fast edits
- Syntax tree construction
- Node type identification
- Edit region detection

**Usage**:
```rust
use rustnote_lib::parser::TreeSitterParser;

let mut parser = TreeSitterParser::new()?;
let (tree, duration) = parser.parse(source)?;
```

**Strengths**:
- Fast incremental updates
- Precise structural information
- Language-aware parsing

**Limitations**:
- `tree-sitter-markdown` grammar has limited GFM-specific node types
- Tables, task lists, autolinks, strikethrough are parsed as generic text/nodes
- No semantic understanding of GFM features

### 2. Comrak Parser (Rendering)

**Purpose**: HTML rendering with full GFM support

**Technology**: `comrak` with GFM extensions enabled

**Key Capabilities**:
- Full GFM specification support
- HTML output generation
- Tables with alignment
- Task lists (checkboxes)
- Strikethrough
- Autolinks
- Frontmatter

**Usage**:
```rust
use rustnote_lib::parser::MarkdownParser;

let parser = MarkdownParser::new();
let html = parser.parse_to_html(markdown);
```

**Strengths**:
- Complete GFM support
- Battle-tested rendering
- Correct HTML output

**Limitations**:
- No incremental parsing
- Full re-parse on every change
- No edit tracking

## GFM Feature Support

| Feature | Tree-Sitter | Comrak | Notes |
|---------|-------------|--------|-------|
| Tables | Basic parse | Full support | Use comrak for rendering |
| Task Lists | Basic parse | Full support | Use comrak for checkbox HTML |
| Strikethrough | Basic parse | Full support | Use comrak for `<del>` tags |
| Autolinks | Basic parse | Full support | Use comrak for link HTML |
| Alignment | Not tracked | Full support | Comrak renders `align` attributes |

## Parsing Strategy

### Editor Workflow

1. **User types** → Tree-sitter updates incrementally → Fast response
2. **Render preview** → Comrak generates HTML → Accurate display
3. **Save document** → Raw markdown saved → Portable format

### Incremental Parsing Flow

```
Initial Load:
  markdown → TreeSitter (full parse) → AST
           → Comrak (full render) → HTML

User Edit:
  markdown → TreeSitter (incremental) → Updated AST
           → Comrak (full render) → Updated HTML
```

### Edit Detection

Tree-sitter tracks edits using byte offsets:

```rust
let edit = tree_sitter::InputEdit {
    start_byte,
    old_end_byte,
    new_end_byte,
    start_position,
    old_end_position,
    new_end_position,
};
```

This enables sub-100ms response times for typing.

## GFM Parsing Details

### Tables

**Tree-Sitter**: Parses pipe characters and cells, but doesn't understand table semantics

**Comrak**: Produces semantic HTML with `<table>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, `<td>`

**Alignment Support**:
- `|:---|` → `align="left"`
- `|:---:|` → `align="center"`
- `|---:|` → `align="right"`

### Task Lists

**Tree-Sitter**: Parses `[ ]` and `[x]` as text within list items

**Comrak**: Produces `<input type="checkbox" [checked]>` elements

### Autolinks

**Tree-Sitter**: Recognizes `<...>` delimiters

**Comrak**: Produces `<a href="...">` with proper URL schemes:
- `https://` and `http://`
- `ftp://`
- `mailto:` for email addresses

### Strikethrough

**Tree-Sitter**: Recognizes `~~...~~` delimiters

**Comrak**: Produces `<del>` elements (also accepts `<s>` or `<strike>`)

## Performance Considerations

### Benchmarks

| Operation | Target | Current |
|-----------|--------|---------|
| Full parse (1MB) | < 5s | ~2s |
| Incremental edit | < 50ms | ~10ms |
| Comrak render | < 100ms | ~20ms |

### Optimization Strategies

1. **Incremental updates**: Only re-parse changed regions
2. **Debounced rendering**: Batch rapid edits before HTML generation
3. **Parser reuse**: Maintain parser instances across edits

## Error Handling

### Malformed Input

Both parsers handle malformed input gracefully:

- **Tree-Sitter**: Returns partial tree, marks errors
- **Comrak**: Attempts best-effort rendering, no panics

### Edge Cases

| Case | Tree-Sitter | Comrak |
|------|-------------|--------|
| Missing table delimiter | Partial parse | Best-effort |
| Inconsistent columns | Full parse | Handles gracefully |
| Invalid autolink | Recognized as text | Skipped |
| Empty strikethrough | Parsed | Rendered as empty |

## Testing Strategy

### Test Categories

1. **Unit Tests**: Individual GFM feature parsing
2. **Integration Tests**: Combined GFM features
3. **Edge Case Tests**: Malformed input handling
4. **Performance Tests**: Large document handling

### Test Files

- `tests/tree_sitter_parser_tests.rs`: Core tree-sitter tests
- `tests/gfm_parser_tests.rs`: GFM-specific tests
- `tests/samples/gfm/*.md`: GFM fixture files

## Future Improvements

1. **Custom Grammar**: Consider extending tree-sitter-markdown for GFM nodes
2. **Parser Coordination**: Share structure between tree-sitter and comrak
3. **Caching**: Cache comrak output for unchanged regions
4. **Streaming**: Support for very large documents

## References

- [GitHub Flavored Markdown Spec](https://github.github.com/gfm/)
- [Tree-Sitter Markdown](https://github.com/tree-sitter-grammars/tree-sitter-markdown)
- [Comrak Documentation](https://comrak.github.io/)
