#!/usr/bin/env bash
set -euo pipefail

# Lightweight cross-layer smoke pass for key desktop flows.
npm test -- \
  src/components/panel/__tests__/CustomizationPanel.smoke.test.tsx \
  src/components/panel/__tests__/PetPanel.smoke.test.tsx \
  src/lib/__tests__/photoBooth.test.ts

cargo test --manifest-path src-tauri/Cargo.toml apply_quest_progress
