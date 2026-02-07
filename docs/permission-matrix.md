# Permission Matrix

Capability file: `src-tauri/capabilities/default.json`

## Granted Permissions

- `core:default`
  - Required baseline for invoking registered Rust commands from the renderer.
- `core:event:allow-emit`
  - Frontend emits tray/timer/guardrail-related events.
- `core:event:allow-listen`
  - Frontend listens for backend state-change events.
- `core:window:allow-start-dragging`
  - Pet overlay drag behavior (`startDragging`).
- `store:default`
  - Rust command layer reads/writes local store state.

## Removed From Prior Broader Scope

- `core:event:default`
- `core:window:default`
- `core:window:allow-show`
- `core:window:allow-hide`
- `core:window:allow-set-focus`
- `core:window:allow-unminimize`

These were removed because they were not required by renderer code paths.

## Verification Notes

After permission updates, verify these flows manually:

1. drag pet overlay
2. tray controls for timer start/pause/resume/reset
3. panel + pet state updates from emitted events
