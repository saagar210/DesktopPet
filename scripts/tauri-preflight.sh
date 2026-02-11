#!/usr/bin/env bash
set -euo pipefail

os="$(uname -s)"

if [ "$os" != "Linux" ]; then
  echo "Tauri preflight: $os detected, no Linux pkg-config checks required."
  exit 0
fi

missing=0

check_pkg() {
  local pkg="$1"
  local label="$2"
  if pkg-config --exists "$pkg"; then
    echo "OK: $label ($pkg)"
  else
    echo "MISSING: $label ($pkg)"
    missing=1
  fi
}

if ! command -v pkg-config >/dev/null 2>&1; then
  echo "MISSING: pkg-config is required for Linux Tauri checks."
  exit 1
fi

check_pkg "glib-2.0 >= 2.70" "GLib runtime"
check_pkg "gobject-2.0" "GObject runtime"
check_pkg "gio-2.0" "GIO runtime"

# WebKitGTK package names vary by distro; accept either modern or legacy names.
if pkg-config --exists "webkit2gtk-4.1"; then
  echo "OK: WebKitGTK runtime (webkit2gtk-4.1)"
elif pkg-config --exists "webkit2gtk-4.0"; then
  echo "OK: WebKitGTK runtime (webkit2gtk-4.0)"
else
  echo "MISSING: WebKitGTK runtime (webkit2gtk-4.1 or webkit2gtk-4.0)"
  missing=1
fi

if [ "$missing" -ne 0 ]; then
  echo ""
  echo "Tauri preflight failed. Install missing Linux packages before running cargo/tauri build."
  exit 1
fi

echo "Tauri preflight passed."
