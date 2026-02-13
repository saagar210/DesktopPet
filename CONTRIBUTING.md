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
npm run test:pack-qa
npm run build
npm run check:performance-budget
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

Or run the wrapper:

```bash
./scripts/verify.sh
```

## Testing Patterns

### Running Tests

```bash
# Frontend unit tests
npm test                 # Run all frontend tests once
npm run test:watch      # Watch mode for development
npm run test:coverage   # Coverage report

# Rust backend tests
npm run test:rust       # Run all Rust tests
npm run test:rust:release  # Release mode tests

# Combined test suite
npm run test:all        # Run frontend + Rust tests
```

### Frontend Testing Patterns

**Hook Testing** (`src/hooks/__tests__/*.test.ts`)
- Mock Tauri IPC with `vi.mock("../lib/tauri")`
- Use `renderHook()` from @testing-library/react
- Test state initialization, updates, and event listeners
- Example:
```typescript
import { renderHook, act, waitFor } from "@testing-library/react";
import { usePomodoro } from "../usePomodoro";

it("should start timer", async () => {
  const { result } = renderHook(() => usePomodoro());
  await act(async () => {
    await result.current.start();
  });
  await waitFor(() => {
    expect(result.current.state.phase).toBe("work");
  });
});
```

**Component Testing** (`src/components/**/__tests__/*.test.tsx`)
- Use `render()` from @testing-library/react
- Test prop handling, user interactions, and rendering
- Use `screen.getByRole()`, `screen.getByText()` for queries

**Error Boundary Testing**
- ErrorBoundary catches React errors
- Location: `src/components/shared/ErrorBoundary.tsx`
- Tests verify fallback UI displays on component errors

### Rust Testing Patterns

**Integration Tests** (`src-tauri/src/tests/mod.rs`)
- Use `fixtures` module for creating default test states
- Helper functions: `default_app_state()`, `create_test_session()`, `create_test_task()`
- Group related tests in modules: `timer_coins_flow`, `pet_interaction_flow`, `edge_cases`
- Example:
```rust
#[test]
fn test_pomodoro_session_creation() {
    let session = create_test_session(25);
    assert_eq!(session.duration_minutes, 25);
}
```

**Command Testing**
- Each command module has unit tests
- Mock store with test state using `StoreLock`
- Test both success and error paths
- Verify event emissions

### Test Coverage Goals

| Layer | Target | Current |
|-------|--------|---------|
| Frontend Components | ≥ 80% | ~85% |
| Frontend Hooks | ≥ 70% | ~60% (scaffolding) |
| Rust Commands | ≥ 70% | ~90% |
| Pet System | ≥ 80% | ~95% |

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
