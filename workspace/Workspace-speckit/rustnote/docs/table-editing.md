# Table Editing Constraints

**Project:** RustNote - Typora-like Markdown Editor  
**Version:** 1.0  
**Last Updated:** 2026-04-14  

---

## 1. Overview

This document describes the table editing constraints and limitations for the RustNote Markdown editor. Understanding these constraints is essential for maintaining data integrity when editing tables in documents.

RustNote uses TipTap (ProseMirror-based) for table editing, which provides a safe and constrained editing model. This document outlines the constraints, their reasons, and best practices for working with tables.

---

## 2. Table Constraints

### 2.1 Constraint 1: Pipe Character in Cell Content

**Issue:** Pipes (`|`) inside cell content can cause parsing issues.

**Behavior:**
- GFM (GitHub Flavored Markdown) tables use pipes as column delimiters
- A pipe character inside a cell must be escaped as `\|`
- Without escaping, the table will misparse during editing

**Example:**
```
| Column 1 | Column 2 |
|----------|----------|
| A \| B   | C        |   ✓ Correct
| A | B    | C        |   ✗ Will misparse
```

**Recommendation:** Avoid using literal pipes in table cells when possible. If needed, use the escaped form `\|`.

---

### 2.2 Constraint 2: Empty Cells

**Issue:** Empty cells must maintain proper pipe delimiters.

**Behavior:**
- Empty cells are represented as `||` (consecutive pipes)
- When editing, ensure empty cells maintain their structure
- Missing pipes can cause column misalignment

**Example:**
```
| A | B | C |
|---|---|---|
| 1 |   | 3 |   ✓ Correct - empty cell represented as space between pipes
| 1 || 3 |   ✗ Incorrect - missing pipe causes misalignment
```

**Recommendation:** Always maintain proper pipe delimiters for empty cells.

---

### 2.3 Constraint 3: Alignment Markers

**Issue:** Alignment markers must be consistent across all columns.

**Behavior:**
- GFM tables support left (`:---`), right (`---:`), and center (`:---:`) alignment
- Each column's delimiter cell must match the header count
- Inconsistent alignment can cause parsing issues

**Example:**
```
| Left | Right | Center |
|:-----|------:|:------:|   ✓ Correct
| A    |     B |   C    |
```

**Recommendation:** Ensure alignment markers are consistent across all delimiter cells.

---

### 2.4 Constraint 4: Row Addition

**Issue:** When adding rows, the new row must maintain proper alignment.

**Behavior:**
- New rows must have the same number of columns as the header
- The delimiter row must not be accidentally modified
- Data integrity of existing rows must be preserved

**Example:**
```
| A | B |
|---|---|
| 1 | 2 |      ← Original row (preserved)
| 3 | 4 |      ← New row (added correctly)
```

**Best Practices:**
1. Always verify column count before adding a row
2. Preserve the delimiter row (`|---|---...|`)
3. Test round-trip serialization after adding rows

---

### 2.5 Constraint 5: Row Deletion

**Issue:** Deleting rows must not accidentally remove the delimiter row.

**Behavior:**
- The delimiter row (alignment markers) is critical for table parsing
- Only delete data rows, never the delimiter
- Verify structure after deletion

**Dangerous:**
```
| A | B |   ← Header
|---|---|   ← Delimiter (DO NOT DELETE)
| 1 | 2 |   ← Data row 1
| 3 | 4 |   ← Data row 2
```

**Best Practices:**
1. Never delete the row immediately after the header
2. Always verify delimiter row is intact after deletion
3. Test round-trip serialization after deleting rows

---

### 2.6 Constraint 6: Column Addition

**Issue:** When adding columns, all rows must be updated consistently.

**Behavior:**
- All rows (header, delimiter, data) must gain a new cell
- Inconsistent column counts cause parsing errors
- The delimiter row must be updated to match header columns

**Example:**
```
Before:          After adding column:
| A | B |        | A | B | C |
|---|---|        |---|---|---|
| 1 | 2 |        | 1 | 2 | 3 |
```

**Best Practices:**
1. Update header row first
2. Update delimiter row to match new column count
3. Update all data rows consistently
4. Verify structure after adding columns

---

### 2.7 Constraint 7: Column Deletion

**Issue:** Column deletion can cause data loss if not done carefully.

**Behavior:**
- All rows must lose exactly one cell
- Must remove the correct column, not accidentally shift data
- Delimiter row must be updated to match new column count

**Dangerous Scenario:**
```
Before:          If column A is deleted incorrectly:
| A | B | C |   | B | C |
|---|---|---|   |---|---|
| 1 | 2 | 3 |   | 2 | 3 |   ← Data shifted (1 lost)
```

**Best Practices:**
1. Identify the correct column to delete
2. Remove one cell from each row consistently
3. Update delimiter row
4. Verify no data was accidentally shifted

---

## 3. Edge Cases

### 3.1 Unicode Content

Tables support unicode content, including:
- Emoji: `🎉`, `👍`, `🚀`
- CJK characters: `标题`, `数据`
- Special symbols: `©`, `®`, `™`

**Example:**
```
| 标题 | 数据 |
|------|------|
| 🎉   | 👍   |
```

**Note:** While unicode is supported, some unicode characters may cause rendering issues in certain viewers.

---

### 3.2 Markdown Formatting in Cells

Cells can contain inline Markdown formatting:

| Cell Content | Markdown Source | HTML Output |
|--------------|-----------------|-------------|
| Bold | `**text**` | `<strong>text</strong>` |
| Italic | `*text*` | `<em>text</em>` |
| Code | `` `code` `` | `<code>code</code>` |
| Link | `[text](url)` | `<a href="url">text</a>` |

**Example:**
```
| Bold | Italic | Code |
|------|--------|------|
| **A** | *B* | `C` |
```

---

### 3.3 Complex Table Structures

RustNote supports:
- Wide tables (many columns)
- Tall tables (many rows)
- Tables at any document position
- Tables with frontmatter

**Large Table Example:**
```
| A | B | C | D | E | F | G | H | I | J |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
```

---

## 4. Data Integrity Guidelines

### 4.1 Round-Trip Verification

Always verify table structure through round-trip serialization:

1. **Parse** the Markdown source
2. **Serialize** back to Markdown
3. **Re-parse** the serialized output
4. **Compare** the results

```rust
// Example verification
let doc = SemanticDocument::parse(source);
let output = doc.serialize_to_commonmark();
let doc2 = SemanticDocument::parse(&output);
let output2 = doc2.serialize_to_commonmark();
assert_eq!(output, output2, "Round-trip should preserve table");
```

### 4.2 HTML Verification

Verify table structure in HTML output:

```rust
let html = doc.html();
assert!(html.contains("<table>"));
assert!(html.contains("<thead>"));
assert!(html.contains("<tbody>"));
assert!(html.contains("<th>"));  // Header cells
assert!(html.contains("<td>"));  // Data cells
```

### 4.3 Cell Count Verification

Verify correct cell counts:

```rust
let html = doc.html();
let th_count = html.matches("<th>").count();
let td_count = html.matches("<td>").count();
assert_eq!(th_count, expected_headers);
assert_eq!(td_count, expected_data_cells);
```

---

## 5. Testing Requirements

### 5.1 Required Test Coverage

All table editing operations must be tested for:

| Test Category | Coverage |
|--------------|----------|
| Row Addition | TC-TE001: Markdown syntax preserved |
| Row Deletion | TC-TE002: Markdown syntax preserved |
| Column Addition | TC-TE003: Markdown syntax preserved |
| Column Deletion | TC-TE004: Markdown syntax preserved |
| Pipe Escaping | TC-TE005: Pipe in cell escaped correctly |
| Empty Cells | TC-TE006: Empty cells render without corruption |
| Complex Structure | TC-TE007: Complex tables render correctly |

### 5.2 Test Commands

```bash
# Run all table editing tests
cargo test table_editing

# Run specific test
cargo test tc_te001
```

---

## 6. Best Practices Summary

### 6.1 Before Editing
- [ ] Verify table structure is valid
- [ ] Backup original content if critical
- [ ] Understand which column/row will be affected

### 6.2 During Editing
- [ ] Maintain proper pipe delimiters
- [ ] Keep delimiter row intact
- [ ] Ensure consistent column counts

### 6.3 After Editing
- [ ] Verify round-trip serialization
- [ ] Check HTML output structure
- [ ] Verify cell counts match expectations
- [ ] Test with edge cases (empty cells, unicode)

---

## 7. References

- [GFM Table Specification](https://github.github.com/gfm/#tables-extension-)
- [TipTap Table Extension](https://tiptap.dev/docs/editor/extensions/table)
- [CommonMark Specification](https://spec.commonmark.org/)

---

*Document maintained as part of RustNote table editing documentation*
