# Architecture

Desktop Pet is a local-first macOS desktop application built with:

- Tauri 2 (Rust backend + native shell)
- React + TypeScript (panel + pet overlay UIs)
- Vite + TailwindCSS (frontend build + styling)
- Tauri Store plugin (`store.json`) for persisted local state

## High-Level Components

- `pet` window (`/pet.html`)
  - Transparent always-on-top companion window.
  - Handles visual pet interactions and drag behavior.
- `panel` window (`/panel.html`)
  - Main control surface for timer, tasks, goals, settings, shop, and stats.
- Rust command layer (`/src-tauri/src/commands/*`)
  - Owns all persistent state mutation.
  - Enforces validation and safe defaults.
- Event bus (`/src-tauri/src/events.rs`)
  - Broadcasts state updates from Rust to frontend listeners.

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

## Build + Test Entry Points

Canonical commands:

- `npm test`
- `npm run build`
- `cargo test --manifest-path src-tauri/Cargo.toml`
- `npm run tauri build`
