# Evolution Validation Checklist

Use this checklist whenever adding or updating species/evolution assets.

## Visual Validation
- [ ] Stage 0/1/2 share a consistent recognizable silhouette.
- [ ] Eye ratio remains in the 10%-22% face-area range.
- [ ] No sharp/aggressive face geometry (angles under 60 degrees avoided around face).
- [ ] Stage 2 remains cute and welcoming (not intimidating).
- [ ] Palette stays soft and cohesive (2-4 base colors + 1 accent).
- [ ] Idle pose reads clearly at small size (overlay scale).

## Behavior Validation
- [ ] Idle animation remains subtle in `low` animation budget.
- [ ] Species-specific idle behavior does not increase distraction (no jitter, no flashing).
- [ ] Accessory micro-behaviors are tiny and optional.

## Product Validation
- [ ] Quiet Mode default still suppresses toast noise.
- [ ] Notification whitelist behavior remains respected.
- [ ] Context-aware chill can dim/idle pet when active.
- [ ] Species switch persists after restart.
- [ ] Evolution thresholds for the species are ascending and valid.

## Testing Validation
- [ ] `npm test` passes.
- [ ] `npm run build` passes.
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [ ] Manual Tauri check confirms species switch + evolution + calm behavior.
