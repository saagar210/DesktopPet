#!/usr/bin/env bash
set -euo pipefail

# Lightweight cross-layer smoke pass for key desktop flows.
npm test -- \
  src/components/panel/__tests__/CustomizationPanel.smoke.test.tsx \
  src/components/panel/__tests__/PetPanel.smoke.test.tsx \
  src/lib/__tests__/photoBooth.test.ts

if command -v cargo >/dev/null 2>&1 && {
  [ "$(uname -s)" != "Linux" ] || [ "${DESKTOP_PET_FORCE_RUST_SMOKE:-0}" = "1" ];
}; then
  cargo test --manifest-path src-tauri/Cargo.toml apply_quest_progress
else
  echo "Skipping Rust smoke step: unsupported environment for Tauri cargo smoke."
fi
