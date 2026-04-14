# PDF Complex Document Test Fixture

## TC-PDF004: All Features Combined

# Document Title

This is the introduction paragraph with **bold text**, *italic text*, `inline code`, and [links](https://example.com).

## Section 1: Text Formatting

### Emphasis Styles
- **Bold text** for emphasis
- *Italic text* for style
- ~~Strikethrough~~ for deleted content
- `Inline code` for technical terms
- [Hyperlink](https://rustnote.app) for references

### Combined Formatting
This paragraph demonstrates **bold and `code` together**, *italic with **nested bold***, and `code with *emphasis*`.

## Section 2: Code Blocks

### Rust Example
```rust
fn main() {
    println!("Hello, RustNote!");
    
    let markdown = "# Hello\n**World**";
    let html = render(markdown);
    println!("{}", html);
}
```

### JavaScript Example
```javascript
const renderer = new MarkdownRenderer();
const result = renderer.parse("# Hello\n**World**");
console.log(result.html);
```

## Section 3: Tables

### Feature Comparison
| Feature | Status | Priority |
|---------|--------|----------|
| Live Preview | ✅ | High |
| Syntax Highlighting | ✅ | High |
| Auto-save | ✅ | Medium |
| Export PDF | ⚠️ | Medium |
| Collaboration | ❌ | Low |

## Section 4: Lists

### Unordered List
- First level item
  - Second level item
    - Third level item
- Another first level
  - With nested content

### Ordered List
1. First numbered item
2. Second numbered item
   1. Nested numbered
   2. Another nested
3. Back to first level

### Task List
- [x] Completed task
- [ ] Pending task
- [x] Another done
  - [ ] Subtask
  - [x] Subtask done

## Section 5: Blockquotes

> This is a blockquote with important information.
> It can span multiple lines.

> **Tip:** Blockquotes are great for highlighting important content.
> 
> They can contain multiple paragraphs.

### Nested Blockquotes
> Outer quote
>> Middle quote
>>> Deep quote

## Section 6: Images

![Logo placeholder](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAoAAAAKCAYAAACNMs+9AAAAFUlEQVR42mNk+M9QzwAEIGAUowEABz8CQWvX+XQAAAAASUVORK5CYII=)

## Section 7: Horizontal Rules

---

Above and below the horizontal rule.

---

## Section 8: Complex Nesting

### List with Code Blocks
- Step 1: Initialize the parser
  ```rust
  let parser = MarkdownParser::new();
  ```
- Step 2: Parse the content
  ```rust
  let doc = parser.parse("# Title");
  ```
- Step 3: Render to HTML
  ```rust
  let html = doc.render();
  ```

### Blockquote with Lists
> Important checklist:
> - [ ] Item one
> - [x] Item two
> - [ ] Item three

### Table with Code in Cells
| Language | Example |
|----------|---------|
| Rust | `fn main()` |
| JS | `const x = 1` |
| Python | `def func():` |

## Section 9: Special Characters

The following characters need escaping: < > & " |

Math symbols: 2 < 3, 5 > 3, 100% complete.

## Conclusion

This document tests all major Markdown features for PDF export quality verification.
