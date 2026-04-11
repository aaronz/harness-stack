# RustNote PRD Wiki

Welcome to the RustNote Product Requirements Document wiki. This directory contains the detailed specification for RustNote, a Typora-like Markdown editor built in Rust.

## Navigation

| Document | Description |
|----------|-------------|
| [01-product-definition](01-product-definition.md) | Executive definition, product thesis, principles |
| [02-product-invariants](02-product-invariants.md) | Invariants, success definition, design principles |
| [03-scope-mvp](03-scope-mvp.md) | MVP scope, features, acceptance criteria |
| [04-functional-requirements](04-functional-requirements.md) | Detailed functional requirements (FR-001 to FR-034) |
| [05-architecture](05-architecture.md) | Non-functional requirements, architecture overview |
| [06-frontend-design](06-frontend-design.md) | Frontend design goals, product principles, IA |
| [07-technical-stack](07-technical-stack.md) | Engineering standards, frontend stack |
| [08-implementation-milestones](08-implementation-milestones.md) | Testing strategy, implementation milestones |
| [09-api-contracts](09-api-contracts.md) | Frontend ↔ Backend IPC API contracts |
| [10-rust-crate-design](10-rust-crate-design.md) | Repository structure, crate responsibilities |
| [11-ux-requirements](11-ux-requirements.md) | Accessibility, error states, keyboard design |
| [12-security](12-security.md) | Security, threat model, dependency policy |
| [13-governance](13-governance.md) | Release engineering, documentation, governance |
| [14-test-plan](14-test-plan.md) | Comprehensive test specifications |
| [CHANGELOG](CHANGELOG.md) | Document version history |

## Quick Links

- **Product**: Typora-like WYSIWYM Markdown editor in Rust
- **Tech Stack**: Tauri v2 + React + Rust
- **License**: MIT OR Apache-2.0
- **Version**: 3.1

## Contributing to this Wiki

This PRD follows the ADR (Architecture Decision Record) pattern. For major changes:

1. Open an issue to discuss the proposed change
2. Create a PR with updates to the relevant document(s)
3. Reference any ADR decisions if applicable
