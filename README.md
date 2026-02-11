# Desktop Pet

Desktop Pet is a playful desktop penguin + focus sidekick.
It floats above your windows, reacts to interactions, and turns your Pomodoro sessions into progression, coins, quests, and customization unlocks.

## Why It Is Fun

- Your penguin lives on your desktop and reacts to pats, care actions, and focus progress.
- Focus sessions reward coins and XP, which feed directly into pet growth and unlocks.
- Quests, events, and accessories keep the loop fresh without becoming noisy.
- You can theme both the app UI and your pet setup with loadouts.

## Current App Features

- **Floating Pet Overlay**: transparent, always-on-top companion window
- **Calm Controls**: Quiet Mode, Focus Mode, animation budget, and context-aware chill
- **Pomodoro Modes**: 15/5, 25/5, and 50/10 presets with runtime persistence
- **Multi-Species Pets**: data-driven species packs with selectable pet species
- **Pet Progression**: evolution stages, care stats, personality state, and calm questing
- **Quests + Events**: rolling events, active quests, and completion rewards
- **Shop + Coins**: accessory catalog with ownership and purchase tracking
- **Tasks + Daily Goals**: lightweight productivity tracking tied to session outcomes
- **Focus Guardrails**: allowlist/blocklist host matching, interventions, and event history
- **Customization**: skins, scenes, themes, and saved loadouts
- **Photo Booth**: user-triggered shareable pet card screenshots
- **Seasonal Cosmetic Packs**: optional, local-only cosmetic bundles
- **System Tray Controls**: quick timer start/pause/resume/reset + preset switching

## Reliability and Recovery

- Local-first data storage with schema normalization on startup
- Snapshot export/import for backup and restore
- One-click full local reset to defaults
- Diagnostics export for debugging and support workflows
- CI quality gates for frontend checks and Tauri backend/bundle checks
- Release evidence policy (manual QA run + CI gate summary in release PRs)

## Tech Stack

Built with [Tauri 2](https://tauri.app/), React, TypeScript, and TailwindCSS.
The pet is SVG-based, and app data is stored locally on-device.

## Prerequisites

- Node.js 20+
- Rust toolchain (stable)
- Xcode Command Line Tools on macOS (`xcode-select --install`)

## Local Development

```bash
npm install
npm test
npm run test:smoke
npm run test:pack-qa
npm run tauri dev
```

## Verification

```bash
./scripts/verify.sh
```

This runs:
- `npm test`
- `npm run test:smoke`
- `npm run test:pack-qa`
- `npm run build`
- `npm run check:performance-budget`
- `npm run test:tauri-preflight`
- `cargo test --manifest-path src-tauri/Cargo.toml`
- `npm run tauri build`

## Production Build

```bash
npm run tauri build
```

Build artifacts are written to `src-tauri/target/release/bundle/`.

## Project Docs

- [Architecture](./docs/architecture.md)
- [DesktopPet Expansion Plan](./docs/desktop-pet-expansion-plan.md)
- [Security Model](./docs/security-model.md)
- [Permission Matrix](./docs/permission-matrix.md)
- [Pet Species Pack Format](./docs/pet-species-pack-format.md)
- [Pack Author Regression Checklist](./docs/pack-author-regression-checklist.md)
- [Performance Budget](./docs/performance-budget.md)
- [Cuteness Style Guide](./docs/cuteness-style-guide.md)
- [Evolution Validation Checklist](./docs/evolution-validation-checklist.md)
- [Debugging](./docs/debugging.md)
- [Operations Runbook](./docs/operations.md)
- [Release Process](./docs/release.md)
- [Manual QA Checklist](./docs/manual-qa-checklist.md)
- [Manual QA Runs](./docs/manual-qa-runs/)
- [Implementation Status](./docs/implementation-status.md)
- [Next Phases Detailed Plan](./docs/next-phases-detailed-plan.md)
- [Contributing](./CONTRIBUTING.md)
