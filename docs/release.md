# Release Process

## Versioning

- Tag format: `v<major>.<minor>.<patch>`
- Source of truth:
  - `package.json` version
  - `src-tauri/tauri.conf.json` version

Keep these in sync before tagging.

## Pre-Release Checklist

1. Run full verification:
   - `./scripts/verify.sh`
2. Confirm no unexpected files in git status.
3. Confirm capability and security-sensitive changes were reviewed.
4. Confirm backup/import/reset flows work in manual smoke test.

## CI Release Workflow

`/.github/workflows/release.yml` handles:

- build macOS bundles
- upload artifacts
- publish GitHub release assets for tag builds

## Signing / Notarization

Current workflow builds unsigned artifacts by default.
For notarized production distribution, configure signing and notarization credentials in CI and add secure signing steps to the release workflow.

## Rollback

- Keep previous release artifacts available.
- If regression is found:
  1. unpublish latest release assets
  2. re-promote last known-good tagged release
  3. restore affected user state from backup if needed
