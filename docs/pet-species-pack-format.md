# Pet Species Pack Format

Species packs are drop-in JSON files loaded from `src/pets/packs/*.json`.
Sprites are loaded from `src/pets/sprites/*.svg`, `*.png`, or `*.webp`.

## Goals
- Add new species without code changes.
- Keep behavior and progression data config-driven.

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

## Seasonal Packs
Optional seasonal cosmetics are loaded from `src/pets/seasonal/*.json` and enabled explicitly by user settings.
No seasonal pack may trigger automatic urgency or notification pressure.
