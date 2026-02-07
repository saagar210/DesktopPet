# Security Model

This project is a local desktop app. Security posture emphasizes:

- strict input validation at command boundaries
- minimized Tauri capability scope
- deterministic state recovery and safe defaults
- no external secrets or remote execution paths by default

## Threat Model Summary

Primary risks in this architecture:

- renderer-side malformed payloads to backend commands
- race conditions causing local data corruption
- over-broad capability grants enabling unintended APIs
- malformed persisted state causing runtime instability

Current mitigations:

- command input validation and normalization (settings, tasks, timer runtime, customization, guardrails)
- `StoreLock` for critical read-modify-write paths
- capability restrictions in `src-tauri/capabilities/default.json`
- startup baseline key initialization and schema handling
- defensive fallback wrappers in frontend `invoke`/`listen` utilities

## Tauri Capability Intent

See `docs/permission-matrix.md` for the explicit permission rationale.

## OWASP / SSDF / SOC 2 (App-Level Mapping)

- Input validation and sanitization: enforced at command layer
- Least privilege: capability file restricted to required runtime permissions
- Failure-safe defaults: defaults applied on missing/invalid store values
- Data integrity: lock-based mutation for multi-step updates
- Observability: diagnostics command + event-driven state updates

## What Is Out Of Scope

- cloud-side identity, RBAC, SSO, and secret rotation systems
- remote telemetry pipelines
- centralized audit logging

## Security Review Cadence

For each release candidate:

1. Re-run full verification (`scripts/verify.sh`)
2. Review capability changes
3. Review command additions for validation + locking
4. Confirm no new external dependency risks were introduced
