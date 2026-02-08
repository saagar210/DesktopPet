# Manual QA Run - Phase 12 Runtime Pack Guardrails

Date: February 8, 2026
Owner: pack schema and CI guardrails hardening
Scope: runtime schema enforcement, automated pack QA harness, and performance budget gate

## Automated Verification Snapshot

- `npm test` -> pass
- `npm run test:smoke` -> pass
- `npm run test:pack-qa` -> pass
- `npm run build` -> pass
- `npm run check:performance-budget` -> pass
- `cargo test --manifest-path src-tauri/Cargo.toml` -> pass
- `./scripts/verify.sh` -> pass

## Manual Product Checks

- Confirmed built-in species packs include `schemaVersion: 1` and continue to load/select correctly.
- Confirmed pack validation surfaces schema mismatches as explicit failures.
- Confirmed CI flow uploads pack QA/performance artifacts and enforces PR budget thresholds.
- Confirmed calm defaults and toast opt-in behavior remain unchanged.

## Phase 12 Acceptance

- Runtime pack schema compatibility is now enforced.
- Pack QA harness and performance budget checks are automated and reproducible.
- Documentation and contributor guidance updated to match enforced behavior.

Result: PASS
