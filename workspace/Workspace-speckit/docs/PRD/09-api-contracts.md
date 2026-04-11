# API Contracts

## 23. API Contracts (Frontend ↔ Backend IPC)

All communication between the React frontend and Rust backend occurs through Tauri IPC commands.

## 23.1 Core Command Types

### Document Commands

```typescript
// FR-001: Create new document
invoke('create_document', { title: string }): DocumentResult

// FR-002: Open document from path
invoke('open_document', { path: string }): DocumentResult

// FR-003: Save document
invoke('save_document', { path: string, content: string }): SaveResult

// FR-004: Export document
invoke('export_document', { path: string, format: 'html' | 'pdf' }): ExportResult
```

### Editor Commands

```typescript
// FR-020: Apply text transformation
invoke('editor_apply_transform', { 
  transform: TransformType, 
  content: string, 
  cursorOffset: number 
}): TransformResult

// FR-021: Render Markdown for display
invoke('render_for_editor', { markdown: string, cursorOffset: number }): RenderResult

// FR-022: Render with syntax highlighting
invoke('render_for_editor_with_highlighting', { 
  markdown: string, 
  cursorOffset: number,
  includeHighlighting: boolean 
}): RenderResult
```

### Workspace Commands

```typescript
// FR-005: List workspace contents
invoke('list_workspace', { path: string }): WorkspaceResult

// FR-006: Watch file for changes
invoke('watch_file', { path: string }): void

// FR-007: Unwatch file
invoke('unwatch_file', { path: string }): void

// FR-008: Check for external changes
invoke('check_external_change', { path: string }): ChangeResult
```

### Settings Commands

```typescript
// CMD-050: Read settings
invoke('read_settings'): Settings

// CMD-051: Write settings
invoke('write_settings', { settings: Settings }): void
```

## 23.2 Data Schemas

### DocumentResult
```typescript
interface DocumentResult {
  id: string;
  title: string;
  content: string;
  file_path: string | null;
  is_dirty: boolean;
  headings: Heading[];
  created_at: string;
  modified_at: string;
}
```

### Heading
```typescript
interface Heading {
  level: 1 | 2 | 3 | 4 | 5 | 6;
  text: string;
  position: number;  // character offset in document
}
```

### TransformType
```typescript
type TransformType = 
  | { Enter: null }
  | { Backspace: null }
  | { Tab: null }
  | { ShiftTab: null }
  | { Wrap: { before: string, after: string } };

interface TransformResult {
  content: string;
  cursor_offset: number;
}
```

### Settings
```typescript
interface Settings {
  theme: 'light' | 'dark';
  autoSave: boolean;
  autoSaveInterval: number;  // milliseconds
  focusMode: boolean;
  typewriterMode: boolean;
  outlineVisible: boolean;
  fontSize: number;
  fontFamily: string;
  lineHeight: number;
}
```

## 23.3 Error Handling

All commands return `Result<T, Error>` where Error is:

```typescript
interface RustError {
  code: ErrorCode;
  message: string;
  details?: string;
}

enum ErrorCode {
  DocumentNotFound = 'DOC_NOT_FOUND',
  DocumentCorrupted = 'DOC_CORRUPTED',
  PermissionDenied = 'PERMISSION_DENIED',
  ExportFailed = 'EXPORT_FAILED',
  ParseError = 'PARSE_ERROR',
  InternalError = 'INTERNAL_ERROR',
}
```

## 23.4 Versioning Strategy

- Commands are versioned via Tauri capability permissions
- Breaking changes require major version bump
- Frontend pins to specific API version in package.json
- Backend validates command versions
