# DesktopPet Next Phases Detailed Plan

Date: February 8, 2026

This plan is intentionally detailed so execution can proceed in larger batches with clear stop/go checkpoints.

## Operating Constraints (Must Hold Every Phase)

### Calmness (measurable)
- Default notifications remain tray-first; desktop toasts are opt-in.
- No autonomous pop-up prompts for seasonal/events content.
- Animation budget is enforced (`low`, `medium`, `high`) with measurable cadence differences.
- Context-aware chill remains configurable and reversible from settings.

### Cuteness (measurable)
- Every new evolution stage passes `docs/evolution-validation-checklist.md`.
- Species packs must pass validator checks before activation.
- New assets must preserve silhouette continuity and avoid aggressive visual contrast.

### Reliability + Permissions
- Local-first only: no remote telemetry by default.
- Respect existing Tauri capabilities in `src-tauri/capabilities/default.json`.
- No build/test regressions at phase boundaries.

## Phase 7: Pack Authoring UX + Validation Hardening

### Goals
- Make species-pack failures actionable with one-click share/copy diagnostics.
- Keep validator rules drift-free between UI, code, and docs.
- Strengthen CI smoke coverage for both frontend and Tauri contexts.

### Deliverables
- Validator “Copy Report” action for failed packs.
- Canonical validation rulebook metadata (id + label + remediation) consumed by validator runtime.
- Docs explicitly reference canonical rulebook source.
- Smoke tests executed in frontend and backend CI jobs.
- `verify.sh` includes smoke suite.

### File Touchpoints
- `src/pets/packValidation.ts`
- `src/components/panel/CustomizationPanel.tsx`
- `src/lib/clipboard.ts`
- `src/components/panel/SettingsPanel.tsx`
- `src/lib/__tests__/clipboard.test.ts`
- `src/pets/__tests__/packValidation.test.ts`
- `.github/workflows/ci.yml`
- `scripts/verify.sh`
- `README.md`
- `CONTRIBUTING.md`
- `docs/architecture.md`
- `docs/pet-species-pack-format.md`

### Stop/Go Checkpoint
- Stop if validation UI exposes failures without remediation guidance.
- Stop if smoke tests are green locally but not represented in CI.
- Go only when:
  - `npm test` passes
  - `npm run test:smoke` passes
  - `npm run build` passes
  - `cargo test --manifest-path src-tauri/Cargo.toml` passes
  - `./scripts/verify.sh` passes

## Phase 8: Behavior Composer + Accessory Personality Layer

### Goals
- Expand subtle species-specific behaviors without adding distraction.
- Make accessory choices feel alive through micro-behavior modifiers.

### Deliverables
- Data-driven behavior profile per species (idle variance, interaction cadence, calm/focus posture).
- Accessory metadata extension for optional micro-behavior effects.
- Runtime behavior composer that combines species baseline + accessory modifiers + animation budget.
- Deterministic clamping to prevent overactive loops.

### File Touchpoints
- `src/pets/packs/*.json`
- `src/pets/species.ts`
- `src/components/pet/PetCharacter.tsx`
- `src/components/pet/animations.css`
- `src/hooks/usePet.ts`
- `src/store/types.ts`
- `src/components/panel/PetPanel.tsx`
- `src/components/panel/ShopPanel.tsx`
- `src/components/pet/__tests__/PetCharacter.test.tsx`
- `src/pets/__tests__/species.test.ts`

### Risks + Mitigations
- Risk: animation drift raises distraction.
- Mitigation: enforce per-budget max animation frequency and add tests that snapshot cadence parameters.
- Risk: accessory effects conflict with species posture.
- Mitigation: define precedence order (budget clamp > focus/quiet chill > accessory modifier > species baseline).

### Stop/Go Checkpoint
- Stop if `low` budget no longer produces visibly calmer output.
- Go only when unit tests cover precedence ordering and manual quiet/focus checks pass in Tauri dev runtime.

## Phase 9: Progression 2.0 (Calm Quests + Optional Moments)

### Goals
- Deepen progression while preserving non-nag behavior.
- Keep rewarding loops user-triggered and low-noise.

### Iteration Plan (Detailed)

### Iteration A (completed): Calm Scheduler + UI Feedback
- Deliverables
- Quest template scheduler with anti-repeat + cooldown behavior.
- Quiet cooldown response surfaced in `PetPanel` with no toast/popup escalation.
- New-quest action button throttled while request is in flight to avoid spam clicks.
- File touchpoints
- `src-tauri/src/commands/pet.rs`
- `src/components/panel/PetPanel.tsx`
- `src/components/panel/ControlPanel.tsx`
- `src/hooks/usePetEvents.ts`
- `src/components/panel/__tests__/PetPanel.smoke.test.tsx`
- Stop/Go criteria
- Stop if New Quest can emit multiple rapid requests.
- Go only when `PetPanel.smoke` covers cooldown feedback + button throttle.

### Iteration B (completed): Accessory-linked Progression Moments
- Deliverables
- Extend quest catalog weights by stage and recent behavior (focus/care mix).
- Accessory metadata hooks for tiny quest-related micro-behavior accents.
- Photo Booth v2 theme variants keyed by species + loadout.
- File touchpoints
- `src-tauri/src/models.rs`
- `src-tauri/src/commands/pet.rs`
- `src/components/panel/ShopPanel.tsx`
- `src/components/pet/PetOverlay.tsx`
- `src/lib/photoBooth.ts`
- `src/lib/__tests__/photoBooth.test.ts`
- `src-tauri/src/commands/pet.rs` tests
- Stop/Go criteria
- Stop if reward totals drift beyond stage pacing envelope.
- Go only when stage reward regression tests and photo booth theme tests pass.

### Iteration C (completed): Optional Seasonal Packs + Calm QA
- Deliverables
- Optional seasonal cosmetic packs remain disabled by default.
- Seasonal pack activation/deactivation flow in customization with clear calm copy.
- Manual calmness QA pass for quiet/focus/chill while seasonal packs enabled.
- File touchpoints
- `src/pets/seasonal/*.json`
- `src/pets/seasonalPacks.ts`
- `src/components/panel/CustomizationPanel.tsx`
- `src/components/panel/__tests__/CustomizationPanel.smoke.test.tsx`
- `docs/manual-qa-runs/*`
- Stop/Go criteria
- Stop if any seasonal pack path emits automatic toasts or nag prompts.
- Go only when seasonal behavior remains opt-in and manual QA checklist passes.

### Phase Risks + Mitigations
- Risk: reward inflation breaks pacing.
- Mitigation: cap daily quest coin issuance and add regression tests for reward totals by stage.
- Risk: optional events feel like pressure.
- Mitigation: default all event packs disabled; no automatic toast path for event content.
- Risk: quiet cooldown messaging becomes noisy.
- Mitigation: keep feedback inline-only and overwrite previous roll feedback instead of stacking notices.

### Phase Stop/Go Checkpoint
- Stop if quest notifications exceed whitelist policy.
- Go only when reward accounting and quest completion tests pass across stage boundaries.
- Status: complete on February 8, 2026 (`docs/manual-qa-runs/2026-02-08-phase9-pr3.md`).

## Phase 10: Polish + Release Hardening

### Goals
- Ship a stable expansion release with clear operational docs and release checklists.
- Reduce support load via diagnostics and contributor clarity.

### Deliverables
- Final docs pass for architecture, pack format, cuteness style, and troubleshooting.
- Manual QA checklist for calmness/cuteness acceptance.
- Release checklist alignment for CI gates and artifact validation.
- Optional accessibility pass (contrast, motion sensitivity guidance).

### File Touchpoints
- `docs/architecture.md`
- `docs/cuteness-style-guide.md`
- `docs/evolution-validation-checklist.md`
- `docs/debugging.md`
- `docs/release.md`
- `README.md`
- `CONTRIBUTING.md`

### Risks + Mitigations
- Risk: docs drift from shipped behavior.
- Mitigation: require docs PR checklist entry and smoke-run output in each release PR.
- Risk: performance regressions after visual polish.
- Mitigation: run side-by-side build size + runtime smoke checks before tagging.

### Stop/Go Checkpoint
- Stop on any mismatch between docs and observable app behavior.
- Go only with full release gate green and manual QA sign-off.
- Status: complete on February 8, 2026 (`docs/manual-qa-runs/2026-02-08-phase10-release-hardening.md`).

## Phase 11: Post-Release Expansion Guardrails

### Goals
- Keep future content additions scalable without eroding calm defaults.
- Preserve local-first reliability while adding optional pack complexity.

### Deliverables
- Species pack compatibility matrix (supported schema versions and migration notes).
- Lightweight performance budget doc for animation cadence and bundle-size deltas.
- Regression checklist for pack authors (validator + calmness + accessibility pass).

### File Touchpoints
- `docs/pet-species-pack-format.md`
- `docs/architecture.md`
- `docs/release.md`
- `CONTRIBUTING.md`

### Stop/Go Checkpoint
- Stop if a new pack requires runtime code changes to load.
- Go only when pack onboarding remains drop-in and verification matrix stays green.
- Status: complete on February 8, 2026 (`docs/manual-qa-runs/2026-02-08-phase11-post-release-guardrails.md`).

## Cross-Phase Verification Matrix

### Automated
- Frontend unit/integration: `npm test`
- Smoke pack: `npm run test:smoke`
- Frontend build/type checks: `npm run build`
- Rust command/model tests: `cargo test --manifest-path src-tauri/Cargo.toml`
- Full verification and bundle: `./scripts/verify.sh`

### Manual (Tauri runtime)
- Quiet mode default behavior and absence of unsolicited toasts.
- Focus mode + context-aware chill transitions during:
  - fullscreen app
  - meeting host match
  - heavy typing burst
- Species switching persistence and evolution threshold correctness.
- Photo Booth export quality and loadout metadata correctness.

## Execution Cadence
- Complete one phase per PR stack when feasible, with each phase checkpoint fully green before continuing.
- Keep commits scoped by subsystem (validator, CI, docs, behavior engine, quests) for fast reviews.
- Do not start the next phase until the current phase passes both automated and manual checkpoint criteria.

## Current State
- Planned phases in this document (7 through 11) are complete and have associated verification evidence.
