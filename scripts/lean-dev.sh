#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LEAN_CACHE_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/desktop-pet-lean.XXXXXX")"

export CARGO_TARGET_DIR="$LEAN_CACHE_ROOT/cargo-target"
export DESKTOPPET_VITE_CACHE_DIR="$LEAN_CACHE_ROOT/vite-cache"
mkdir -p "$CARGO_TARGET_DIR" "$DESKTOPPET_VITE_CACHE_DIR"

print_size_report() {
  local label="$1"
  echo
  echo "[$label] Disk usage snapshot"
  for path in \
    "$ROOT_DIR/node_modules" \
    "$ROOT_DIR/node_modules/.vite" \
    "$ROOT_DIR/src-tauri/target" \
    "$ROOT_DIR/dist" \
    "$ROOT_DIR/artifacts" \
    "$CARGO_TARGET_DIR" \
    "$DESKTOPPET_VITE_CACHE_DIR"; do
    if [ -e "$path" ]; then
      du -sh "$path"
    else
      echo "0B  $path (missing)"
    fi
  done
}

cleanup() {
  local exit_code=$?
  echo
  echo "lean-dev: cleaning temporary caches"
  rm -rf "$LEAN_CACHE_ROOT"

  if "$ROOT_DIR/scripts/clean-heavy.sh"; then
    true
  else
    echo "lean-dev: warning - heavy cleanup encountered an issue" >&2
  fi

  print_size_report "After cleanup"
  exit "$exit_code"
}
trap cleanup EXIT INT TERM

print_size_report "Before start"
echo

echo "lean-dev: using temporary cache root $LEAN_CACHE_ROOT"
cd "$ROOT_DIR"
npm run tauri dev
