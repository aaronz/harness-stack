# UX Requirements

## 28. UX Requirements

### 28.1 Layout
* left sidebar for workspace tree
* central editor canvas
* optional outline panel
* minimal top bar/menu
* find/replace panel
* export dialog
* preferences dialog

### 28.2 Interaction Rules
* split preview is not primary mode
* visible chrome must stay minimal
* command palette recommended
* link, image, and table interactions must be intentional and documented

### 28.3 Quality Bar
The product should feel calm enough for long writing sessions and precise enough for technical documentation.

---

## 29. Accessibility and Keyboard Design

### 29.1 Keyboard-First Navigation

Essential actions must be reachable via keyboard:
* new/open/save
* panel toggle
* search/replace
* command palette
* export
* focus/typewriter mode toggle
* heading/list/link/image/code/table insertion actions

### 29.2 Focus Management
* visible focus indicators required for interactive controls
* modals and popovers must trap and restore focus correctly
* command palette and search bar should return focus to editor appropriately

### 29.3 Contrast and Legibility
* default themes must meet a reasonable accessibility baseline
* selected text, inactive dimmed text, code spans, and links must remain legible

### 29.4 Screen Reader Considerations
* semantic labeling on controls
* accessible dialogs and menus
* sensible reading order outside the editing canvas

### 29.5 Internationalization (i18n) Readiness

#### NFR-014 i18n Architecture

The MVP targets English-only UI, but the architecture must not preclude future localization:

**Requirements:**
* All user-facing strings externalized (not hardcoded)
* String extraction tooling in place for translation workflow
* RTL layout support considered in CSS (not required for MVP)

**Implementation guidance:**
* Use i18n library (e.g., `i18next` for React)
* Keys follow naming convention: `section.action.description`
* Example: `toolbar.file.new.tooltip = "Create new document"`

**Future phases:**
* Language pack format: JSON per locale
* Supported locales (future): en, zh, ja, de, fr, es
* Date/time formatting via Intl API

---

## 30. Error States, Empty States, and Recovery UX

### 30.1 Empty States

Examples:
* no file open
* empty workspace
* no search results
* no recent files
* no outline items

These should guide action without feeling verbose.

### 30.2 Error Surfaces

The frontend should distinguish:
* blocking errors
* recoverable errors
* transient notifications
* background warnings

### 30.3 Recovery UX

#### Error Class Taxonomy
| Class | Behavior | User Action |
|-------|----------|-------------|
| Blocking | Cannot proceed | Must resolve to continue |
| Recoverable | Can proceed but data may be lost | Prompt user choice |
| Transient | Auto-resolves | Show briefly, no action needed |
| Warning | Informational | User decides |

#### User Messaging Templates
* Save failed: "Could not save file. [Retry] [Save As...] [Discard Changes]"
* File changed externally: "[Reload] [Keep Current] [View Diff]"
* Crash recovery: "[Restore] [Discard] [View Backup]"
