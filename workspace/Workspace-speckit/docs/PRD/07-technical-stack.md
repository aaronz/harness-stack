# Technical Stack

## 16. Engineering Standards

### 16.1 Rust Standards

* `cargo fmt` enforced
* `clippy` required in CI
* avoid unnecessary `unsafe`
* any `unsafe` must be isolated and justified

### 16.2 Code Organization

* business logic should not live in UI event handlers
* public APIs documented
* non-trivial features need tests and short design rationale

### 16.3 PR Standards

* small PRs preferred
* issue or rationale linked
* UI changes include screenshots/gifs when practical
* user-visible behavior changes update docs or fixtures

---

## 17. Frontend Architecture Standards

### 17.1 Framework

React 18 with component-based architecture.

### 17.2 Build Tool

Vite for fast development and optimized production builds.

### 17.3 Styling

Tailwind CSS for utility-first styling with design token integration.

### 17.4 State Management

React Context for application state:
- `DocumentContext` - current document, file operations
- `SettingsContext` - theme, preferences, mode toggles

### 17.5 Tauri Integration

Use `@tauri-apps/api` for:
- `invoke()` for backend commands
- `open()` / `save()` dialogs from `@tauri-apps/plugin-dialog`
- `read()` / `write()` from `@tauri-apps/plugin-fs`

---

## 18. Frontend Layer Model

The frontend is divided into layers:

### A. Presentation Layer
* layout containers
* typography, colors, spacing
* visual states
* menus, dialogs, panels, buttons, inputs

### B. Editor Interaction Layer
* editor canvas rendering
* selection and caret visualization
* keyboard event dispatch
* mouse/pointer interactions
* drag-and-drop handling
* contextual affordances around document elements

### C. App State Layer
* active workspace
* open document metadata
* visible panels and dialogs
* user preferences currently applied
* command palette state
* search/replace state
* notifications and transient UI states

### D. Service Bridge Layer
* calling Rust-owned services via Tauri IPC
* serializing requests/responses
* subscribing to events from core modules
* enforcing typed contracts

---

## 19. Frontend State Model

### State Categories

#### A. Persistent User State
* theme
* focus mode default
* typewriter mode default
* content width
* font preferences
* panel visibility preferences
* recent items cache display preferences

#### B. Session App State
* active workspace
* current document ID/path
* open panel states
* current search query
* current replace query
* export dialog state
* modal visibility
* pending notifications

#### C. Transient Interaction State
* hover states
* context menu anchor
* drag target
* current pointer selection preview
* temporary link editing popover state

#### D. Derived View State
* outline tree from current document
* current dirty state indicator
* export availability
* save/recovery banners
* current section in viewport if tracked

### State Ownership Rules
* Rust owns document truth and editing semantics
* frontend owns ephemeral view state and interaction affordances
* duplicated source-of-truth state between frontend and Rust should be minimized

---

## 20. Component Hierarchy

### App-Level Components
* `AppShell`
* `TitleBar`
* `MenuBarBridge`
* `WorkspaceSidebar`
* `EditorWorkspace`
* `OutlinePanel`
* `StatusSurface`
* `ModalHost`
* `ToastHost`
* `CommandPalette`

### Workspace Components
* `WorkspaceTree`
* `WorkspaceTreeNode`
* `FileRow`
* `FolderRow`
* `WorkspaceContextMenu`
* `RecentItemsList`

### Editor Components
* `EditorCanvas`
* `DocumentViewport`
* `BlockRenderer`
* `InlineRenderer`
* `CaretLayer`
* `SelectionLayer`
* `DropCursorLayer`
* `LinkPopover`
* `ImageBlock`
* `CodeFenceBlock`
* `TableBlock`
* `QuoteBlock`
* `TaskListItem`
* `HeadingBlock`

### Utility Components
* `SearchBar`
* `ReplaceBar`
* `ExportDialog`
* `PreferencesDialog`
* `ShortcutHint`
* `EmptyState`
* `RecoveryBanner`
* `InlineNotice`
