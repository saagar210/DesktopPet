# Performance Budget

This budget keeps DesktopPet calm, battery-safe, and non-distracting while new packs and cosmetics are added.

## Animation Cadence Budget

Animation budget settings must produce measurable cadence differences:

| Setting | Idle Loop Cadence | Blink Window | Accent Animations |
| --- | --- | --- | --- |
| `low` | one major cycle every `>= 6s` | `5000-9000ms` | max 1 accent every `>= 12s` |
| `medium` | one major cycle every `4-6s` | `3500-7000ms` | max 1 accent every `>= 8s` |
| `high` | one major cycle every `2.5-4s` | `2500-5500ms` | max 1 accent every `>= 5s` |

Rules:

- `low` must always appear calmer than `medium`.
- `high` behavior is opt-in and must still respect Quiet/Focus/Context-aware chill.
- Accessory personality modifiers may not bypass animation-budget clamping.

## Bundle Size Budget

Use `npm run build` output as evidence in PRs touching pets/assets.

Release deltas (compared with latest `master` release artifact):

- `dist/assets/species-*.js`: max `+25 kB` gzip per pack-focused PR.
- `dist/assets/pet-*.css`: max `+8 kB` gzip per visual polish PR.
- `dist/assets/panel-*.js`: max `+12 kB` gzip per feature PR.

If a PR exceeds a budget:

- include explicit justification
- add mitigation (asset compression, sprite simplification, or code-splitting)
- obtain reviewer sign-off before merge

## Runtime Guardrail Checks

Manual runtime checks (Tauri dev or release build):

1. Toggle `low/medium/high` and confirm visible cadence differences.
2. Enable Quiet + Focus modes and verify accent animations remain subdued.
3. Simulate context-aware chill trigger (fullscreen/meeting/heavy typing) and verify dim/idle transition.
4. Verify no animation loop causes sustained CPU spikes during 10+ minute idle.

## Evidence Required in PR

- build output snippet with changed bundle sizes
- checklist confirmation from `docs/pack-author-regression-checklist.md`
- manual note that calmness behavior remained intact under `low` budget
