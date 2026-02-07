# Operations Runbook

## Local Data Safety

Before risky changes or test sessions:

1. Export backup from Settings.
2. Save the JSON backup file.
3. Proceed with tests/changes.

## Recovery Paths

### Import Backup

Use Settings -> `Import Backup` and select a previously exported JSON file.
The app reloads after successful import.

### Full Reset

Use Settings -> `Reset App Data`.
This restores defaults and clears local progress data.

## Diagnostics For Support

Use Settings -> `Copy Diagnostics` and attach the JSON diagnostics payload to bug reports.

Include:

- app version
- OS/arch
- schema version
- object counts (tasks/sessions/summaries/events)
