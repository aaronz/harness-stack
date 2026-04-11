# Document Changelog

## v3.1 (Current)

Added critical refinement sections based on implementation review:

* **8.5 MVP Acceptance Criteria** - Added measurable performance thresholds, functional acceptance checklist, visual checkpoints, accessibility requirements, platform requirements
* **11.1 Performance** - Expanded with NFR-013 (Performance Budgets table) and NFR-014 (Scalability Targets by document size)
* **11.3 Privacy and Security** - Expanded with NFR-010 (Threat Model), NFR-011 (HTML Sanitization Policy), NFR-012 (Export Security)
* **20.7 Testing Coverage Matrix** - Added feature-to-test-type mapping, CI gates, test data requirements
* **25.5 Internationalization (i18n) Readiness** - Added NFR-014 i18n architecture guidance for future localization
* **32. API Contracts** - NEW section defining frontend ↔ backend IPC command types, data schemas, error handling, versioning

## v3.0

Major architecture update:
* Updated to Tauri v2
* Added production vs MVP architecture distinction
* Added recommended production architecture blueprint (Tiptap/ProseMirror, ropey, tree-sitter, Shiki, syntect)
* Expanded ADR topics (14 items)
* Updated repository structure (production + MVP dual structure)
* Updated crate responsibilities for production stack

## v2.1

Previous refinement draft

## v2.0

Initial architecture decisions documented
