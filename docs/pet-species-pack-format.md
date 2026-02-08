# Pet Species Pack Format

Species packs are drop-in JSON files loaded from `src/pets/packs/*.json`.
Sprites are loaded from `src/pets/sprites/*.svg`.

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
- `id`: lowercase slug (`a-z`, `0-9`, `_`, `-`).
- `stageNames`: exactly 3 entries.
- `evolutionThresholds`: exactly 3 ascending numbers, starts at 0.
- `stageSpriteFiles`: exactly 3 files present in `src/pets/sprites/`.
- `interactionVerbs`: should include supported action ids used by the pet panel.
- `accessoryAnchors`: 0-200 coordinate space matching sprite viewbox expectations.

## Seasonal Packs
Optional seasonal cosmetics are loaded from `src/pets/seasonal/*.json` and enabled explicitly by user settings.
No seasonal pack may trigger automatic urgency or notification pressure.
