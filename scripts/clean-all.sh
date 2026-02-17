#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

"$ROOT_DIR/scripts/clean-heavy.sh"

ALL_TARGETS=(
  "$ROOT_DIR/node_modules"
)

for target in "${ALL_TARGETS[@]}"; do
  if [ -e "$target" ]; then
    rm -rf "$target"
    echo "Removed $target"
  else
    echo "Skipped $target (not present)"
  fi
done
