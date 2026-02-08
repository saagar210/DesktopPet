# Pet Species Pack Format

Species packs are drop-in JSON files loaded from `src/pets/packs/*.json`.
Sprites are loaded from `src/pets/sprites/*.svg`, `*.png`, or `*.webp`.

## Goals
- Add new species without code changes.
- Keep behavior and progression data config-driven.
- Keep pack additions compatible with calmness and performance guardrails.

## Required Fields
```json
{
  "id": "cat",
  "name": "Cat",
  "description": "A cozy desk companion.",
  "stageNames": ["Kit", "Purrling", "Moon Cat"],
  "evolutionThresholds": [0, 4, 12],
  "idleBehavior": {
    "baseAnimation": "sway",
    "blinkIntervalMs": [4200, 7600]
  },
  "interactionVerbs": [
    { "id": "pet", "label": "Scritch" },
    { "id": "feed", "label": "Treat" },
    { "id": "play", "label": "Feather" },
    { "id": "nap", "label": "Curl up" },
    { "id": "clean", "label": "Brush" },
    { "id": "train", "label": "Trick" }
  ],
  "behaviorProfile": {
    "motionBias": "calm",
    "interactionCadenceMs": [620, 1180],
    "chillPosture": "curl"
  },
  "stageSpriteFiles": ["cat-stage-0.svg", "cat-stage-1.svg", "cat-stage-2.svg"],
  "accessoryAnchors": {
    "head": { "x": 100, "y": 34 },
    "neck": { "x": 100, "y": 114 },
    "left": { "x": 52, "y": 120 },
    "right": { "x": 148, "y": 120 }
  }
}
```

## Validation Rules
Canonical validation metadata (rule ids, labels, remediation text) is defined in
`src/pets/packValidation.ts` via `PACK_VALIDATION_RULEBOOK`.

- `id-slug`: lowercase slug (`a-z`, `0-9`, `_`, `-`).
- `stage-names`: exactly 3 entries.
- `thresholds`: exactly 3 ascending numbers, starts at 0.
- `sprites`: exactly 3 sprite files present in `src/pets/sprites/`.
- `blink-window`: blink interval min must be `>= 1200` and max must be greater than min.
- `anchors`: 0-200 coordinate space matching sprite viewbox expectations.
- `verbs`: include `pet`, `feed`, `play`, `nap`, `clean`, and `train`.
- `behavior-profile`: `interactionCadenceMs` must be `[fast, slow]` where `250<=fast<=slow<=5000` and `chillPosture` is `settle`, `hover`, or `curl`.

## Compatibility Matrix

DesktopPet currently supports pack schema **v1** (the shape documented on this page).
Until runtime schema negotiation is introduced, any new pack format must be migrated to v1 before drop-in use.

| Schema | Runtime Support | Required Action | Notes |
| --- | --- | --- | --- |
| `v1` (current) | Supported | None | Uses `stageSpriteFiles` in source pack JSON and normalizes to runtime `stageSprites`. |
| `v2` (future proposal) | Not supported yet | Migrate to v1 before shipping | Keep migration notes in PR and include before/after JSON sample. |
| `v3+` (future) | Not supported yet | Do not ship directly | Requires explicit runtime implementation + validator expansion. |

## Migration Notes

- `v2 -> v1`: flatten any nested sprite metadata into `stageSpriteFiles: [baby, teen, adult]`.
- `v2 -> v1`: ensure behavior cadence is constrained to `[250, 5000]` bounds.
- `v2 -> v1`: map any extra interaction verbs back to the core set (`pet`, `feed`, `play`, `nap`, `clean`, `train`).
- `v3+ -> v1`: unsupported by default; provide a one-time conversion script and reviewer evidence before merge.

## Behavior Profile Semantics
- `motionBias`: controls baseline motion preference (`calm`, `balanced`, `playful`).
- `interactionCadenceMs`: controls minimum tap/pat interaction cooldown in normal and chill states.
- `chillPosture`: controls subtle resting posture during focus/quiet/context chill.

## Seasonal Packs
Optional seasonal cosmetics are loaded from `src/pets/seasonal/*.json` and enabled explicitly by user settings.
No seasonal pack may trigger automatic urgency or notification pressure.

## Author Gate

Before opening a PR for a new pack:

- complete `docs/pack-author-regression-checklist.md`
- confirm animation and bundle impact against `docs/performance-budget.md`
- attach validation evidence (automated + manual) in the PR description
