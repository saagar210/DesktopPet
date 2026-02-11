# Next Steps Implementation Plan (Phases 13-17)

Date: February 10, 2026

## Goal
Shift from "feature complete" to "reliable daily driver" by improving runtime reliability, guardrail quality, analytics usefulness, onboarding, and pack ecosystem growth.

## Phase 13 — Runtime Confidence & Packaging Reliability

### Scope
- Add Linux native dependency preflight script for Tauri Rust/build workflows.
- Add environment diagnostics to smoke script output.
- Wire preflight into verification flow.
- Document Linux preflight and dependency expectations.

### Deliverables
- `scripts/tauri-preflight.sh`
- `scripts/tauri-smoke.sh` environment + preflight checks
- `scripts/verify.sh` runs preflight before Rust/Tauri build
- `package.json` script: `test:tauri-preflight`
- Docs updates in `README.md` and `docs/debugging.md`

### Acceptance Criteria
- `npm run test:tauri-preflight` passes on correctly provisioned Linux hosts.
- `npm run test:smoke` prints environment diagnostics.
- `./scripts/verify.sh` fails early when Linux dependencies are missing.

### Effort Estimate
- Engineering: 1–2 days
- QA/docs: 0.5 day
- Total: 1.5–2.5 days

### Price Estimate (contractor planning)
- Low: $1,200
- Mid: $2,000
- High: $2,800

## Phase 14 — Guardrails + Context-Aware Chill Accuracy

### Scope
- Improve guardrail decision visibility in settings (why intervention fired).
- Add deterministic simulation fixtures for host/phase matching.
- Replace static host sampling with rotating sample strategy in timer interventions.

### Deliverables
- Expanded focus guardrails unit tests (frontend + Rust helper tests)
- Settings diagnostics section for latest intervention context
- Timer hook sampling strategy update

### Acceptance Criteria
- False-positive bug reports decrease in manual QA runs.
- Intervention context is visible and copyable in UI diagnostics.
- Guardrail simulation suite is green in CI.

### Effort Estimate
- Engineering: 3–5 days
- QA/docs: 1 day
- Total: 4–6 days

### Price Estimate
- Low: $3,200
- Mid: $5,000
- High: $7,200

## Phase 15 — Actionable Stats & Weekly Recap

### Scope
- Upgrade stats panel from raw counters to trend and action cards.
- Add weekly recap export.
- Add recommendation layer based on consistency and guardrail pressure.

### Deliverables
- Trend cards in `StatsPanel`
- Weekly recap generator (JSON + markdown export)
- Recommendation logic tests

### Acceptance Criteria
- Stats panel surfaces trend direction (up/down/flat) for at least 3 core metrics.
- Weekly recap export works from panel.
- Recommendation snapshots pass deterministic tests.

### Effort Estimate
- Engineering: 4–6 days
- QA/docs: 1 day
- Total: 5–7 days

### Price Estimate
- Low: $4,000
- Mid: $6,400
- High: $8,600

## Phase 16 — Onboarding & First-Week Retention

### Scope
- Build first-run setup wizard (preset, calm defaults, species activation, quick task).
- Add progressive disclosure for advanced settings.
- Add first-week check-in prompts in panel (quiet, inline only).

### Deliverables
- Onboarding state machine and UI flow
- First-week checklist card
- Regression tests for non-nag behavior

### Acceptance Criteria
- New user can start first focus session in <2 minutes.
- Advanced settings are hidden by default, discoverable on demand.
- No autonomous popups introduced.

### Effort Estimate
- Engineering: 5–8 days
- QA/docs: 1–2 days
- Total: 6–10 days

### Price Estimate
- Low: $5,500
- Mid: $8,800
- High: $12,500

## Phase 17 — Pack Ecosystem UX

### Scope
- Add local pack inbox/import flow with validator result summaries.
- Add pack health badges and compatibility messaging.
- Add optional curated local bundle install UX.

### Deliverables
- Pack import UI + validation report viewer
- Pack health metadata in customization panel
- Extended pack QA docs and troubleshooting

### Acceptance Criteria
- Users can import a pack locally and understand pass/fail in one screen.
- Invalid packs are blocked with remediation guidance.
- Compatibility state is surfaced before activation.

### Effort Estimate
- Engineering: 6–9 days
- QA/docs: 1–2 days
- Total: 7–11 days

### Price Estimate
- Low: $6,800
- Mid: $10,500
- High: $14,500

## Consolidated Delivery Window
- Sequential execution: ~24 to 36 working days.
- Parallelized (2 engineers): ~14 to 22 working days.

## Recommended Execution Order
1. Phase 13 (dependency + verification reliability)
2. Phase 14 (guardrail/chill quality)
3. Phase 16 (onboarding and retention)
4. Phase 15 (actionable analytics)
5. Phase 17 (pack ecosystem UX)

## Verification Matrix for Every Phase
- `npm test`
- `npm run test:smoke`
- `npm run test:pack-qa`
- `npm run build`
- `npm run check:performance-budget`
- `npm run test:tauri-preflight`
- `cargo test --manifest-path src-tauri/Cargo.toml`
- `npm run tauri build`
