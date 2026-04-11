# RustNote Keyboard Shortcuts

## File Operations

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+N` / `Cmd+N` | New | Create a new untitled document |
| `Ctrl+O` / `Cmd+O` | Open | Open an existing markdown file |
| `Ctrl+S` / `Cmd+S` | Save | Save the current document |

## Text Formatting

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+B` / `Cmd+B` | Bold | Make selected text bold (`**text**`) |
| `Ctrl+I` / `Cmd+I` | Italic | Make selected text italic (`*text*`) |
| `Ctrl+Shift+C` / `Cmd+Shift+C` | Toggle Task | Toggle checkbox state in task list |
| `Ctrl+Shift+X` / `Cmd+Shift+X` | Strikethrough | Add strikethrough (`~~text~~`) |

## Structural Editing

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Enter` in list | Continue list | Insert new list item |
| `Enter` on empty list item | Exit list | Remove the list marker |
| `Tab` in list | Indent | Indent the list item |
| `Shift+Tab` in list | Dedent | Outdent the list item |
| `Enter` in heading | New heading | Create heading at same level |
| `Enter` in blockquote | Continue quote | Continue the blockquote |
| `Backspace` at line start | Join lines | Join with previous line |

## Task Lists

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Shift+C` / `Cmd+Shift+C` | Toggle checkbox | Check/uncheck the current task |

## Search and Navigation

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+F` / `Cmd+F` | Find | Open the search panel |
| `Ctrl+G` / `Cmd+G` | Find Next | Go to next search result |
| `Ctrl+Shift+G` / `Cmd+Shift+G` | Find Previous | Go to previous search result |

## Undo and Redo

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Z` / `Cmd+Z` | Undo | Undo the last action |
| `Ctrl+Shift+Z` / `Cmd+Shift+Z` | Redo | Redo the last undone action |
| `Ctrl+Y` / `Cmd+Y` | Redo | Alternative redo shortcut |

## Platform Differences

### macOS
- Use `Cmd` key instead of `Ctrl`
- Use `Cmd+,` for Preferences

### Windows/Linux
- Use `Ctrl` as the modifier key
- Use `Ctrl+,` for Settings

## Smart Editing Behavior

RustNote provides smart editing behaviors that automatically handle Markdown structures:

### Lists
- Pressing `Enter` at the end of a list item continues the list with the next number/item
- Pressing `Enter` on an empty list item (`- ` or `- [ ]`) exits the list
- `Tab` indents list items, `Shift+Tab` dedents them
- Ordered lists automatically increment the number

### Blockquotes
- Pressing `Enter` in a blockquote continues the quote with `> ` prefix

### Headings
- Pressing `Enter` in a heading creates a new heading at the same level

### Task Lists
- `Ctrl+Shift+C` toggles the checkbox between `[ ]` and `[x]`

## Implementation Notes

The smart editing behaviors are powered by the Rust backend transform engine:
- `apply_transform` command handles Enter/Backspace/Tab based on cursor context
- Falls back to manual JavaScript handling if Rust command fails
- Live preview updates via `update_source` IPC command