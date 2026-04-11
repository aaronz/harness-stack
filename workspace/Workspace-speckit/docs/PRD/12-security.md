# Security

## 12. Security

This document covers RustNote's security posture, threat model, sanitization policies, and mitigation strategies.

---

## 12.1 Security Principles

1. **Local-first means no server trust** - All data stays on the user's filesystem; no network transmission of document content in MVP.
2. **Defense in depth** - Multiple layers of protection for file operations, rendering, and export.
3. **Minimal attack surface** - No plugin marketplace, no telemetry, no third-party script execution.
4. **Rust as safety boundary** - Memory-safe Rust code for all file I/O and parsing operations.

---

## 12.2 Threat Model

### 12.2.1 Assets to Protect

| Asset | Risk | Impact |
|-------|------|--------|
| User documents | Read/write unauthorized access | Data confidentiality/loss |
| User workspace filesystem | Path traversal attacks | System compromise |
| Export outputs | HTML injection / XSS | User browser execution |
| Settings persistence | Tampering | Preference corruption |
| Recovery snapshots | Access to unsaved content | Privacy leak |

### 12.2.2 Threat Matrix

| Threat | Attack Vector | Mitigation | Owner |
|--------|---------------|------------|-------|
| Path traversal | `../` in file paths | Validate and normalize paths in Rust before I/O | @rust-team |
| Arbitrary file read | Opening malicious `.md` with symlinks | Sandboxed file access via Tauri | @rust-team |
| Arbitrary file write | Export to arbitrary paths | User confirmation for overwrite; path bounds | @rust-team |
| Markdown HTML injection | Malicious `<script>` in rendered output | Sanitize all raw HTML on render | @rust-team |
| XSS in HTML export | Malicious content in exported HTML | Sanitize exports; strip active content | @rust-team |
| Malicious image paths | Path pointing outside workspace | Validate image paths within workspace boundary | @rust-team |
| Recovery data leak | Crash recovery storing sensitive content | Encrypt recovery snapshots (future) | @rust-team |
| Settings tampering | Edit settings file directly | Validate schema on load | @rust-team |

---

## 12.3 Input Validation

### 12.3.1 File Paths

All file paths MUST be validated and normalized in Rust before any I/O operation:

```rust
// Required validations:
1. Canonicalize path (resolve symlinks, normalize separators)
2. Verify path is within allowed workspace boundary
3. Reject paths containing null bytes or control characters
4. Validate UTF-8 encoding
5. Check path length against OS limits
```

### 12.3.2 Markdown Content

| Input Type | Validation |
|------------|-----------|
| User typing | No validation needed (trusted source) |
| File open | Parse with comrak; discard unparseable sections |
| Paste from clipboard | Best-effort Markdown conversion; sanitize HTML |
| Drag-and-drop | Validate MIME type; copy assets to workspace |
| External file changes | Re-parse and validate on reload |

---

## 12.4 HTML Sanitization Policy

### 12.4.1 Allowed HTML in Rendered Output

The following semantic HTML elements are permitted in the live-rendered editor view:

| Category | Allowed Elements |
|----------|-----------------|
| Text | `<p>`, `<strong>`, `<em>`, `<s>`, `<code>`, `<kbd>`, `<samp>` |
| Headings | `<h1>`, `<h2>`, `<h3>`, `<h4>`, `<h5>`, `<h6>` |
| Lists | `<ul>`, `<ol>`, `<li>` |
| Block | `<blockquote>`, `<pre>`, `<hr>` |
| Inline | `<a>` (href validated), `<img>` (src validated) |
| Tables | `<table>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, `<td>` |
| Task lists | `<input type="checkbox">` (checked/disabled only) |

### 12.4.2 Forbidden HTML (Stripped on Render)

All instances of the following are stripped from rendered output:

```html
<!-- Active content -->
<script>...</script>
<iframe>...</iframe>
<object>...</object>
<embed>...</embed>
<applet>...</applet>

<!-- Event handlers -->
onclick, onerror, onload, onmouseover, onfocus, onblur,
onkeydown, onkeyup, onkeypress, onchange, onsubmit,
oninput, ondrag, ondrop, onscroll

<!-- Dangerous URLs -->
javascript:*
data: (except for explicitly allowed image types)

<!-- Injection vectors -->
<style>...</style>
<link rel="stylesheet">
<base href="...">
<svg>...</svg> (considered dangerous; sanitize)
<meta http-equiv="...">
```

### 12.4.3 Link Validation

Links are validated as follows:
- `http:` and `https:` URLs → allowed
- `mailto:` URLs → allowed
- `javascript:` URLs → blocked
- `data:` URLs → blocked except for safe image types (png, gif, webp, svg)
- Relative paths → validated against workspace boundary

---

## 12.5 Export Security

### 12.5.1 HTML Export

- Exports are **self-contained** (no external resource dependencies)
- No JavaScript in exported HTML
- All `<script>` tags stripped
- All event handler attributes stripped
- All `javascript:` URLs stripped
- `<base>` tag injection prevented
- External image references converted to embedded or removed

### 12.5.2 PDF Export

- Uses sandboxed PDF generation (`printpdf` with constrained context)
- No active content in PDF
- Fonts embedded, not referenced externally
- No executable content

### 12.5.3 Export File Overwrite Protection

- If export target exists, prompt user for confirmation
- Never silently overwrite user files
- Export to temp location first, then move (atomic write)

---

## 12.6 Memory Safety

### 12.6.1 Rust Safety Standards

From [Engineering Standards](docs/PRD/07-technical-stack.md):

- `cargo fmt` enforced
- `clippy` required in CI with zero warnings
- Avoid `unsafe` blocks; any `unsafe` must be:
  - Isolated in dedicated modules
  - Justified with safety comments
  - Covered by unit tests

### 12.6.2 Fuzz Testing

The Markdown parser MUST be fuzz-tested with:
- Malformed UTF-8 sequences
- Deeply nested structures (>1000 levels)
- Extremely long lines (>1MB)
- Null bytes and control characters
- Binary data injection attempts

---

## 12.7 Privacy

### 12.7.1 Local-First Privacy

| Data | Storage | Access |
|------|---------|--------|
| Document content | User's filesystem | User only |
| Settings | Local config file | User only |
| Recent files | Local SQLite DB | User only |
| Recovery snapshots | Local temp directory | User only |
| Telemetry | None | N/A |

### 12.7.2 No Network Privacy Concerns (MVP)

Since the MVP has no network functionality:
- No data leaves the user's machine
- No analytics or crash reporting
- No update checks that leak document names

---

## 12.8 Security Review Checklist

Before each release:

- [ ] Path traversal testing against all file operations
- [ ] HTML sanitization verified for all Markdown render paths
- [ ] XSS testing on HTML export
- [ ] Clipboard paste sanitization tested
- [ ] Image path validation tested
- [ ] `clippy` passes with zero warnings
- [ ] No `unsafe` blocks added without security review
- [ ] Fuzz tests pass against parser
- [ ] Export overwrite protection tested

---

## 12.9 Reporting Security Issues

For security vulnerabilities, please:

1. **Do NOT** open a public GitHub issue
2. Email the maintainers directly (see `SECURITY.md`)
3. Allow 48 hours for initial response
4. Provide details: affected versions, reproduction steps, severity assessment

---

## 12.10 Future Security Enhancements (Post-MVP)

| Enhancement | Priority | Notes |
|-------------|----------|-------|
| Encrypted recovery snapshots | High | Protect unsaved content |
| Signed releases | High | Prevent supply chain attacks |
| Plugin sandboxing | Medium | If plugin marketplace added |
| TOTP for sensitive settings | Low | Future consideration |
