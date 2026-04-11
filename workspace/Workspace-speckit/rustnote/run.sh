#!/bin/bash
set -e

cd "$(dirname "$0")"

echo "Building RustNote..."
cargo tauri build

echo "Launching RustNote..."
open target/release/bundle/macos/RustNote.app
