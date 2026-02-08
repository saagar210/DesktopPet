# Contributing

## Development Setup

```bash
npm install
npm run tauri dev
```

## Required Verification

Run these before opening a PR:

```bash
npm test
npm run test:smoke
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

Or run the wrapper:

```bash
./scripts/verify.sh
```

## Code Standards

- Keep diffs focused and minimal.
- Validate all command inputs defensively.
- Use `StoreLock` for read-modify-write store operations.
- Emit state-change events when backend state mutation should update UI.
- Add tests for new logic branches.

## Security Constraints

- Do not commit secrets.
- Do not add remote data dependencies without explicit design review.
- Preserve least-privilege capability posture in `src-tauri/capabilities/default.json`.

## PR Expectations

- Explain what changed and why.
- Include verification command output summary.
- Call out any user-facing behavior changes.
- If docs-relevant behavior changed, update corresponding docs in the same PR.
- For release-facing changes, include:
  - latest CI check links
  - `./scripts/verify.sh` summary
  - manual QA run file under `docs/manual-qa-runs/`

## Pack Author Requirements

If your PR adds or changes species/seasonal packs:

- complete `docs/pack-author-regression-checklist.md`
- confirm schema compatibility in `docs/pet-species-pack-format.md`
- report animation and bundle impact from `docs/performance-budget.md`
