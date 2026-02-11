#!/usr/bin/env bash
set -euo pipefail

npm test
npm run test:smoke
npm run test:pack-qa
npm run build
npm run check:performance-budget
npm run test:tauri-preflight
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
