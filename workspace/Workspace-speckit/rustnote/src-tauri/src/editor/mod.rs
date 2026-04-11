//! Editor engine - cursor, selection, commands, transforms, and undo/redo
//!
//! This module provides:
//! - [`cursor`] - Cursor movement rules
//! - [`selection`] - Selection handling
//! - [`commands`] - Editor command types
//! - [`transforms`] - Smart Enter/Backspace/Tab handling
//! - [`undo`] - Undo/redo history

pub mod commands;
pub mod cursor;
pub mod selection;
pub mod transforms;
pub mod undo;

pub use commands::*;
pub use cursor::*;
pub use selection::*;
pub use transforms::*;
pub use undo::*;
