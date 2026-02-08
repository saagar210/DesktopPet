# DesktopPet Expansion Plan

## Baseline Audit (February 8, 2026)

### Implemented today
- Tauri desktop app with two windows (`pet` overlay + `panel` control surface).
- Local-first store (`store.json`) with command-layer validation and lock-guarded mutations.
- Core systems: Pomodoro runtime, coins, goals, tasks, shop, customization loadouts, focus guardrails, analytics summaries, backup/import/reset.
- Single hard-coded pet flow (3 evolution stages), fixed accessories, fixed quest flavor, fixed stage thresholds.
- Notification controls exist but were coarse-grained (global desktop notifications/sounds).

### Aspirational / missing before this expansion
- No multi-species data model or drop-in pack format.
- Evolutions not governed by explicit cute-style constraints.
- No measurable non-distracting controls (quiet/focus modes, animation budget, whitelist policy).
- No context-aware chill loop for typing/meeting/fullscreen signals.
- No Photo Booth export card.
- No optional seasonal cosmetic pack activation model.

## Measurable Product Constraints
- Quiet by default: `quietModeEnabled = true`, `toastNotificationsEnabled = false`, tray badge enabled.
- Toast rate limit: max 3 per rolling hour.
- Animation budget:
  - `low`: slows active animation loops and strips glow complexity.
  - `medium`: default cadence.
  - `high`: faster loops only when user opts in.
- Context-aware chill applies dim/idle behavior from local signals (fullscreen heuristic, meeting host matches, heavy typing threshold).
- All features stay local-first and permission-safe (no new network/remote dependencies).

## Phase Plan

### Foundation
Deliverables
- Expand settings model for calm controls + notification policy + context-aware chill config.
- Add species-aware pet state fields (`speciesId`, `evolutionThresholds`) and backend command to switch species safely.
- Update evolution stage resolution to use state-driven thresholds (not hard-coded constants).
- Wire panel-to-overlay chill signal transport channel.

File touchpoints
- `src/store/types.ts`
- `src/hooks/useSettings.ts`
- `src/hooks/usePomodoro.ts`
- `src/hooks/useContextAwareChill.ts`
- `src/lib/chill.ts`
- `src/components/pet/PetOverlay.tsx`
- `src/components/pet/animations.css`
- `src-tauri/src/models.rs`
- `src-tauri/src/commands/settings.rs`
- `src-tauri/src/commands/pomodoro.rs`
- `src-tauri/src/commands/pet.rs`
- `src-tauri/src/lib.rs`

Stop/Go checkpoint
- Stop if TypeScript or Rust compilation fails.
- Go only when `npm test`, `npm run build`, and `cargo test --manifest-path src-tauri/Cargo.toml` are all green.

### Iter1
Deliverables
- Data-driven species framework with drop-in JSON pack + sprite assets.
- At least 3 new species defined and selectable (`cat`, `corgi`, `axolotl`) plus `penguin` pack.
- Species-specific idle behavior mapping and interaction verb labels.
- Seasonal pack scaffolding (optional activation, no timed pressure).

File touchpoints
- `src/pets/species.ts`
- `src/pets/packs/*.json`
- `src/pets/sprites/*.svg`
- `src/pets/seasonalPacks.ts`
- `src/pets/seasonal/*.json`
- `src/components/pet/PetCharacter.tsx`
- `src/hooks/usePet.ts`
- `src/components/panel/CustomizationPanel.tsx`
- `src/components/panel/ControlPanel.tsx`
- `src/components/pet/__tests__/PetCharacter.test.tsx`
- `src/pets/__tests__/*.test.ts`

Stop/Go checkpoint
- Stop if species pack load or fallback behavior regresses.
- Go only when tests/build pass and species switching works in UI without command errors.

### Iter2
Deliverables
- Quest redesign for calm progression (focus + gentle care quest types, no nag loops).
- Accessory micro-behavior modifiers (subtle visual delight).
- Photo Booth “pet card” export (user-triggered only).

File touchpoints
- `src-tauri/src/models.rs`
- `src-tauri/src/commands/pet.rs`
- `src/components/panel/PetPanel.tsx`
- `src/lib/photoBooth.ts`
- `src/components/pet/PetOverlay.tsx`
- `src/components/pet/animations.css`

Stop/Go checkpoint
- Stop if quest completion/reward accounting regresses.
- Go only when coin/progress/pet state remain consistent and tests/build are green.

### Polish
Deliverables
- Cuteness style guide + evolution validation checklist docs.
- Architecture/docs refresh for species pack format, calm controls, and seasonal packs.
- Final quality gate and release-safe verification.

File touchpoints
- `docs/cuteness-style-guide.md`
- `docs/evolution-validation-checklist.md`
- `docs/pet-species-pack-format.md`
- `docs/architecture.md`
- `README.md`

Stop/Go checkpoint
- Stop if documentation mismatches shipped behavior.
- Go only with clean final verification (`./scripts/verify.sh`).

## Verification Strategy
- Unit tests:
  - TypeScript component/hook behavior (species rendering, settings interactions, seasonal pack filtering).
  - Rust command and model invariants (settings sanitation, quest/evolution progression).
- Integration checks:
  - Panel ↔ overlay signal synchronization (settings + chill behavior).
  - Species switch + persistence + evolution threshold behavior.
- End-to-end (manual, Tauri):
  - Timer lifecycle + quest progress + coin rewards.
  - Quiet mode default behavior (no toast spam).
  - Photo Booth export success path.
  - Seasonal pack activation/deactivation path.
- Release gate:
  - `npm test`
  - `npm run build`
  - `cargo test --manifest-path src-tauri/Cargo.toml`
  - `npm run tauri build` (via `./scripts/verify.sh` at final checkpoint)
