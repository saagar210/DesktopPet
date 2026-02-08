# Pack Author Regression Checklist

Use this checklist for every new or updated species/seasonal pack PR.

## Validator + Schema

- [ ] Pack passes validator (`src/pets/packValidation.ts`) with no failed rules.
- [ ] Pack format matches `docs/pet-species-pack-format.md` schema v1 requirements.
- [ ] Compatibility matrix was reviewed and no unsupported schema was introduced.
- [ ] Sprite assets resolve correctly for all three evolution stages.

## Calmness + Non-Distraction

- [ ] No automatic toast or popup behavior is introduced by the pack.
- [ ] Seasonal/event content remains opt-in only.
- [ ] `low` animation budget remains visibly calmer than `medium`.
- [ ] Accessory or species micro-behaviors do not bypass quiet/focus/chill constraints.

## Cuteness + Accessibility

- [ ] Evolutions satisfy `docs/evolution-validation-checklist.md`.
- [ ] Cuteness visuals align with `docs/cuteness-style-guide.md`.
- [ ] Motion-sensitive behavior is acceptable under reduced animation settings.
- [ ] Color/contrast changes keep controls and text readable in panel UI.

## Performance + Build

- [ ] Bundle-size deltas are within `docs/performance-budget.md` limits.
- [ ] No heavy render loops or unthrottled timers were added.
- [ ] `npm run build` output was captured in PR evidence.

## Verification + Evidence

- [ ] `npm test` passes.
- [ ] `npm run test:smoke` passes.
- [ ] `npm run build` passes.
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [ ] `./scripts/verify.sh` passes.
- [ ] Manual QA run note added under `docs/manual-qa-runs/` when behavior changed.
