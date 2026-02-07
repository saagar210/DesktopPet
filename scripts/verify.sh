#!/usr/bin/env bash
set -euo pipefail

npm test
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
