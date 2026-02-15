# Contributing

## Development Setup

```bash
npm install
npm run tauri dev
```

## Required Verification

Run this before opening a PR:

```bash
./scripts/verify.sh
```

Equivalent checks:

```bash
npm test
npm run test:smoke
npm run test:pack-qa
npm run build
npm run check:performance-budget
npm run test:tauri-preflight
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

## Engineering Rules

- Keep changes minimal and scoped.
- Validate command inputs defensively.
- Use `StoreLock` for read-modify-write store operations.
- Emit events when backend mutations should update UI.
- Add or update tests for new logic paths.

## Security Rules

- Do not commit secrets.
- Do not weaken capability boundaries in `src-tauri/capabilities/default.json`.
- Do not add remote data dependencies without explicit review.

## PR Expectations

- Explain what changed and why.
- Include verification command output summary.
- Call out any user-facing behavior changes.
