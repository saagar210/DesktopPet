# Manual QA Checklist

Use this checklist for release candidates and major behavior-system changes.

Date:
Tester:
Build/Branch:

## Calmness + Non-Distraction

- [ ] App starts with quiet defaults (`quietModeEnabled=true`, toast notifications off).
- [ ] Timer completion does not show desktop toast unless user opted in.
- [ ] `Animation Budget: Low` visibly reduces animation intensity/frequency.
- [ ] `Animation Budget: High` remains smooth but not jittery or noisy.
- [ ] Focus Mode visibly calms pet behavior (dim + lower motion).
- [ ] Context-aware chill responds to:
  - [ ] fullscreen app
  - [ ] configured meeting host
  - [ ] heavy typing burst

## Species + Cuteness

- [ ] Species switching works for penguin, cat, corgi, and axolotl.
- [ ] Species idle behavior feels distinct but subtle (no aggressive loops).
- [ ] Evolution stage transitions preserve silhouette and cuteness rules.
- [ ] Validation failures show remediation details and copyable report.

## Accessory Micro-Behaviors

- [ ] Accessory hints appear in shop cards.
- [ ] Passive accessories (`scarf`, `sunglasses`) remain subtle during chill.
- [ ] Active accessories (`party_hat`, `bow_tie`) are clamped in low budget/focus.
- [ ] Snack accessories (`apple`, `cookie`) animate only when motion budget allows.

## Seasonal + Event Calmness

- [ ] Seasonal packs are disabled by default.
- [ ] Enabling seasonal pack shows optional bundle presets only (no urgency language).
- [ ] Applying seasonal bundle does not change notification toggles.
- [ ] Seasonal content remains user-triggered and toast-silent unless user opted in.

## Accessibility + Motion

- [ ] Low animation budget remains readable and non-jittery at overlay scale.
- [ ] Focus/quiet/chill dim states preserve species readability.
- [ ] No decorative motion appears as high-frequency flicker.

## Reliability + Recovery

- [ ] Backup export/import/reset still work.
- [ ] Diagnostics copy works (clipboard + fallback message).
- [ ] Species and settings persist across restart.

## Notes / Regressions

- Observed issues:
- Follow-ups:
