# Manual QA Run - Phase 11 Post-Release Guardrails

Date: February 8, 2026
Owner: release/docs hardening
Scope: post-release pack guardrails, compatibility policy, and performance budget evidence

## Automated Verification Snapshot

- `npm test` -> pass
- `npm run test:smoke` -> pass
- `npm run build` -> pass
- `cargo test --manifest-path src-tauri/Cargo.toml` -> pass
- `./scripts/verify.sh` -> pass

## Manual Product Checks

- Confirmed pack onboarding remains drop-in for schema v1 packs.
- Confirmed no new urgency paths were introduced for seasonal/cosmetic content.
- Confirmed quiet defaults and toast opt-in policy remain unchanged in docs and release gate.
- Confirmed pack author workflow now requires calmness, accessibility, and performance review.

## Phase 11 Acceptance

- Species pack compatibility matrix documented with migration expectations.
- Pack author regression checklist documented and linked from contribution/release paths.
- Performance budget thresholds documented for animation cadence and bundle deltas.

Result: PASS
