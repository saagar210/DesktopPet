#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

TARGETS=(
  "$ROOT_DIR/dist"
  "$ROOT_DIR/artifacts"
  "$ROOT_DIR/src-tauri/target"
  "$ROOT_DIR/node_modules/.vite"
)

for target in "${TARGETS[@]}"; do
  if [ -e "$target" ]; then
    rm -rf "$target"
    echo "Removed $target"
  else
    echo "Skipped $target (not present)"
  fi
done
