#!/usr/bin/env bash
set -euo pipefail

# Lightweight cross-layer smoke pass for key desktop flows.
npm test -- \
  src/components/panel/__tests__/CustomizationPanel.smoke.test.tsx \
  src/components/panel/__tests__/PetPanel.smoke.test.tsx \
  src/lib/__tests__/photoBooth.test.ts

echo "Smoke environment: os=$(uname -s) node=$(node -v) rust=$(cargo --version 2>/dev/null || echo unavailable)"

if [ "$(uname -s)" = "Linux" ]; then
  if ./scripts/tauri-preflight.sh; then
    echo "Linux preflight passed; Rust smoke can be forced with DESKTOP_PET_FORCE_RUST_SMOKE=1."
  else
    echo "Skipping Rust smoke step: Linux native Tauri dependencies are missing."
    exit 0
  fi
fi

if command -v cargo >/dev/null 2>&1 && {
  [ "$(uname -s)" != "Linux" ] || [ "${DESKTOP_PET_FORCE_RUST_SMOKE:-0}" = "1" ];
}; then
  cargo test --manifest-path src-tauri/Cargo.toml apply_quest_progress
else
  echo "Skipping Rust smoke step: unsupported environment for Tauri cargo smoke."
fi
