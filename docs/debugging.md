# Debugging Guide

## Quick Start

```bash
npm install
npm run tauri dev
```

## Common Checks

- Frontend tests: `npm test`
- Frontend build: `npm run build`
- Rust tests: `cargo test --manifest-path src-tauri/Cargo.toml`
- Full local verification: `./scripts/verify.sh`

## Runtime State Issues

Use Settings -> Data & Diagnostics:

- `Copy Diagnostics` for current app health snapshot
- `Export Backup` before high-risk changes
- `Import Backup` to restore known-good state
- `Reset App Data` to recover from irreparable local state

## Typical Failure Patterns

- UI stale state:
  - check event listeners and emitted events
  - verify command mutation emits matching state-change event
- Unexpected timer values:
  - inspect `timer_runtime` normalization logic
- Focus guardrails mismatches:
  - verify host normalization and suffix match rules

## Build/Bundle Failures

- Ensure Xcode CLI tools are installed on macOS.
- Re-run `npm run tauri build` from a clean dependency state.
- For CI parity, compare with `.github/workflows/ci.yml`:
  - Frontend Checks: `npm test`, `npm run test:smoke`, `npm run build`
  - Tauri Backend + Bundle: `npm run test:smoke`, Rust tests, `npm run tauri build`

## Release Mismatch Triage

If a release candidate differs from expected behavior:

1. Confirm docs and shipped behavior still match:
   - `README.md`
   - `docs/architecture.md`
   - `docs/release.md`
2. Re-run full verification:
   - `./scripts/verify.sh`
3. Validate manual calmness/cuteness checks:
   - `docs/manual-qa-checklist.md`
4. Capture diagnostics + backup before resetting state.

## Store Corruption Recovery

1. Export backup if possible.
2. Reset app data from Settings.
3. Re-import a known-good backup.
