# Next Steps Implementation Plan

Date: February 8, 2026

## Scope
Implement all previously proposed next steps and close known risks/follow-ups:
1. Pack validator before species activation.
2. Tauri smoke script for species switch + quest completion + photo booth.
3. Visual polish pass for placeholder species output.
4. Risk mitigation for context-aware chill heuristics, pack-format assumptions, and tray badge platform variance.

## Plan

### Step 1: Pack Validation Gate (Activation Required)
- Build deterministic pack checks (id, thresholds, sprite formats, anchor bounds, verbs, blink range).
- Add a "Species Pack Validator" UI in customization.
- Keep species locked until explicit activation.
- Persist activated packs in settings (`validatedSpeciesPacks`).

Success criteria
- New drop-in packs appear in validator.
- Invalid packs cannot be activated.
- Species selector only shows activated packs (plus current species fallback).

### Step 2: Smoke Coverage
- Add focused smoke tests for:
  - Pack activation flow in customization.
  - Photo booth user-triggered capture flow.
  - Quest completion progression helper in Rust.
- Add a smoke runner script and package script entry.

Success criteria
- `npm run test:smoke` passes locally.
- Smoke suite covers species-switch gating + quest-complete logic + photo-booth path.

### Step 3: Visual Polish
- Improve sprite presentation with cleaner depth/shadow treatment and refined overlay rendering.
- Keep low distraction defaults (no flashing, no aggressive motion).

Success criteria
- Pet visuals appear cleaner/cuter while preserving calm behavior.
- Animation budgets still throttle as expected.

### Step 4: Risk Mitigations
- Context-aware chill risk:
  - Keep heuristics local/configurable and make thresholds visible in settings.
- Pack assumption risk:
  - Support additional sprite formats (`svg`, `png`, `webp`) and validate expectations.
- Tray badge variance risk:
  - Return tray capability result from backend and provide local fallback count display.

Success criteria
- Platform limitations degrade gracefully without breaking UX.
- Settings and validation surfaces make constraints explicit.

## Verification
- `npm test`
- `npm run test:smoke`
- `npm run build`
- `cargo test --manifest-path src-tauri/Cargo.toml`
- `./scripts/verify.sh` (final full gate)
