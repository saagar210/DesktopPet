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
