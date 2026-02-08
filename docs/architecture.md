# Architecture

Desktop Pet is a local-first macOS desktop application built with:

- Tauri 2 (Rust backend + native shell)
- React + TypeScript (panel + pet overlay UIs)
- Vite + TailwindCSS (frontend build + styling)
- Tauri Store plugin (`store.json`) for persisted local state

## High-Level Components

- `pet` window (`/pet.html`)
  - Transparent always-on-top companion window.
  - Handles visual pet interactions, species rendering, animation budget, and chill dimming behavior.
- `panel` window (`/panel.html`)
  - Main control surface for timer, tasks, goals, settings, shop, stats, customization, and Photo Booth.
- Rust command layer (`/src-tauri/src/commands/*`)
  - Owns all persistent state mutation.
  - Enforces validation and safe defaults.
- Event bus (`/src-tauri/src/events.rs`)
  - Broadcasts state updates from Rust to frontend listeners.
- Species + cosmetics packs (`/src/pets`)
  - Data-driven species packs (`packs/*.json`) + sprite assets (`sprites/*.svg`).
  - Runtime pack loading enforces `schemaVersion` compatibility before activation.
  - Behavior composer merges species profile + accessory effects + calm controls.
  - Optional seasonal pack metadata (`seasonal/*.json`) activated only by user opt-in.
  - Photo Booth theme resolver maps species + loadout to deterministic card presentation.
  - Pack compatibility policy is documented in `docs/pet-species-pack-format.md`.

## Data Flow

1. Frontend invokes command (`invoke`) through `src/lib/tauri.ts` wrappers.
2. Rust command validates input and applies mutation under `StoreLock` when needed.
3. Updated state is persisted to `store.json`.
4. Rust emits event(s) to synchronize UI state.
5. Frontend hooks receive events via `listenSafe` and update local component state.

## Persistence Model

Primary persisted keys:

- `pet`
- `coins`
- `tasks`
- `goals`
- `sessions`
- `settings`
- `timer_runtime`
- `user_progress`
- `daily_summaries`
- `customization_loadouts`
- `pet_events`
- `pet_active_quest`
- `focus_guardrail_events`
- `schema_version`
- quest progression preference signals (`pet_recent_focus_progress`, `pet_recent_care_progress`)
- calm-control settings (`quiet mode`, `focus mode`, `animation budget`, `context-aware chill`)
- optional seasonal pack activation list

Startup hardening in `storage::ensure_schema_version` ensures baseline keys exist for core safety-critical state.

## Concurrency Model

- Shared read-modify-write operations use `StoreLock` (`Mutex<()>`) to prevent lost updates.
- Commands that mutate multiple keys in a single logical transaction acquire the lock before loading store values.
- Non-mutating reads avoid lock contention where possible.

## Recovery + Operations

Maintenance commands:

- `export_app_snapshot`
- `import_app_snapshot`
- `reset_app_state`
- `get_app_diagnostics`

These support local backup/restore, reset, and support diagnostics without external services.

## Calmness + Cuteness Enforcement

- Quiet-by-default behavior is enforced by default settings (`quietModeEnabled=true`, `toastNotificationsEnabled=false`).
- Seasonal content is opt-in only and may not trigger urgency prompts or automatic toast pressure.
- Cuteness standards are codified in:
  - `docs/cuteness-style-guide.md`
  - `docs/evolution-validation-checklist.md`
- Validation coverage spans:
  - species pack rule checks
  - quest/event anti-nag cooldown behavior
  - animation budget and chill-path behavior composition

## Performance Guardrails

- Animation cadence and bundle-size budgets are defined in `docs/performance-budget.md`.
- Pack changes must pass `docs/pack-author-regression-checklist.md` before merge.
- CI enforces budget thresholds with `npm run check:performance-budget`.
- Budget exceptions require explicit PR justification and mitigation notes.

## Build + Test Entry Points

Canonical commands:

- `npm test`
- `npm run test:smoke`
- `npm run test:pack-qa`
- `npm run build`
- `npm run check:performance-budget`
- `cargo test --manifest-path src-tauri/Cargo.toml`
- `npm run tauri build`
