# Phase 1 Implementation Plan

**Date:** 2026-02-15
**Scope:** Achievement System + Pack Authoring UX Improvements
**Estimated Duration:** 8-10 hours

## Overview

Phase 1 combines two high-value features:
1. **Achievement System** - Milestone tracking with badges and progression rewards
2. **Pack Authoring UX** - Improved validator feedback and copy-report functionality

Both features enhance user engagement and developer experience without compromising the calm, local-first philosophy.

---

## Part A: Achievement System

### Goals
- Recognize user milestones (streaks, sessions, progression)
- Add collectible badges that reward engagement
- Integrate achievements with existing progression system
- Maintain calm UX (no intrusive popups)

### Achievement Categories

**Focus Achievements** (Pomodoro-based)
- `first_session` - Complete your first focus session
- `focused_5` - Complete 5 focus sessions
- `focused_25` - Complete 25 focus sessions
- `focused_100` - Complete 100 focus sessions
- `marathon` - Complete 10 sessions in one day

**Streak Achievements**
- `streak_3` - Maintain 3-day streak
- `streak_7` - Maintain 7-day streak
- `streak_30` - Maintain 30-day streak
- `dedication` - Maintain 100-day streak

**Pet Care Achievements**
- `first_interaction` - Interact with your pet
- `caretaker` - Reach 100% on all care stats
- `pet_evolved` - Evolve your pet to stage 2
- `pet_mastery` - Evolve your pet to stage 3

**Progression Achievements**
- `level_5` - Reach level 5
- `level_10` - Reach level 10
- `wealthy` - Earn 1000 coins
- `collector` - Own 10 accessories

**Special Achievements**
- `early_bird` - Complete session before 8am
- `night_owl` - Complete session after 10pm
- `quest_master` - Complete 10 quests
- `perfectionist` - Complete all daily goals

### Data Schema

**Rust Models** (`src-tauri/src/models.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    pub id: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub unlocked_at: Option<String>,
    pub progress: u32,
    pub target: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementState {
    pub achievements: Vec<Achievement>,
    pub total_unlocked: u32,
    pub last_unlocked_id: Option<String>,
}
```

**TypeScript Types** (`src/store/types.ts`)
```typescript
export interface Achievement {
  id: string;
  category: "focus" | "streak" | "pet" | "progression" | "special";
  title: string;
  description: string;
  icon: string;
  unlockedAt: string | null;
  progress: number;
  target: number;
}

export interface AchievementState {
  achievements: Achievement[];
  totalUnlocked: number;
  lastUnlockedId: string | null;
}
```

### Backend Implementation

**Files to Create:**
- `src-tauri/src/achievements.rs` - Achievement definitions and unlock logic
- `src-tauri/src/commands/achievements.rs` - Tauri commands

**Commands:**
- `get_achievements()` - Fetch all achievements with unlock status
- `check_achievement_progress()` - Evaluate achievement triggers
- `get_achievement_stats()` - Summary stats (X/Y unlocked)

**Achievement Triggers:**
- Session completion → check focus achievements
- Daily summary update → check streak achievements
- Pet evolution → check pet achievements
- XP/level up → check progression achievements

### Frontend Implementation

**New Components:**
- `src/components/panel/AchievementsPanel.tsx` - Achievement gallery
- `src/components/shared/AchievementBadge.tsx` - Single badge display
- `src/components/shared/AchievementToast.tsx` - Subtle unlock notification

**Hook:**
- `src/hooks/useAchievements.ts` - Achievement state management

**UI Design:**
- Grid layout with locked/unlocked states
- Progress bars for in-progress achievements
- Filter by category
- Search functionality
- Subtle animation on unlock (respects animation budget)

### Integration Points

1. **Session Completion** (`commands/pomodoro.rs:complete_pomodoro`)
   - After updating session count, check focus achievements
   - Emit `achievement_unlocked` event if triggered

2. **Daily Summary** (`progression.rs`)
   - Check streak achievements on streak updates
   - Check daily goal completion achievements

3. **Pet Evolution** (`commands/pet.rs`)
   - Trigger pet achievement checks on stage changes

4. **Store Updates** (`storage.rs`)
   - Persist achievement state in app store
   - Load achievements on app start

---

## Part B: Pack Authoring UX Improvements

### Goals
- Make validation failures actionable
- Add "Copy Report" button for failed pack validations
- Improve remediation guidance visibility
- Strengthen smoke test coverage

### Deliverables

**1. Copy Report Functionality**
- Add clipboard utility for validation reports
- Format validation results as shareable text
- Include all failed checks with remediation steps

**2. Enhanced Validation UI**
- Display remediation inline with failures
- Add "Copy Report" button in CustomizationPanel
- Highlight failed checks clearly

**3. Clipboard Utilities**
```typescript
// src/lib/clipboard.ts
export async function copyValidationReport(result: PackValidationResult): Promise<void>
export function formatValidationReport(result: PackValidationResult): string
```

**4. Test Coverage**
- Unit tests for clipboard utilities
- Integration tests for validation flow
- Smoke tests for pack loading

### Implementation Steps

**Step 1: Clipboard Utility**
- Create `src/lib/clipboard.ts`
- Implement `copyValidationReport()` with Tauri clipboard API
- Format validation results as markdown

**Step 2: Enhanced Validation UI**
- Update `src/components/panel/CustomizationPanel.tsx`
- Add "Copy Report" button for failed validations
- Show remediation inline with each failed check

**Step 3: Tests**
- `src/lib/__tests__/clipboard.test.ts` - Clipboard utilities
- `src/pets/__tests__/packValidation.test.ts` - Validation logic
- Update smoke tests to include pack validation

**Step 4: Documentation**
- Update `docs/pet-species-pack-format.md` with examples
- Reference canonical rulebook in docs
- Add troubleshooting section

---

## Implementation Order

### Stage 1: Backend Foundation (2-3 hours)
1. ✅ Create achievement data structures (Rust)
2. ✅ Implement achievement unlock logic
3. ✅ Add Tauri commands for achievements
4. ✅ Integrate with existing progression system

### Stage 2: Frontend Achievement UI (2-3 hours)
5. ✅ Create AchievementsPanel component
6. ✅ Build AchievementBadge component
7. ✅ Implement useAchievements hook
8. ✅ Add navigation to achievements panel

### Stage 3: Pack Validator UX (2 hours)
9. ✅ Create clipboard utilities
10. ✅ Add Copy Report button to validation UI
11. ✅ Enhance remediation display

### Stage 4: Testing & Polish (2 hours)
12. ✅ Write unit tests for achievements
13. ✅ Write tests for clipboard utilities
14. ✅ Update smoke tests
15. ✅ Run full verification suite
16. ✅ Update documentation

---

## Testing Strategy

### Achievement Tests
- Unit tests for unlock logic
- Integration tests for trigger points
- UI tests for badge rendering

### Validation Tests
- Clipboard formatting tests
- Pack validation regression tests
- Smoke tests for pack loading

### Manual Verification
- Complete session → verify achievement unlock
- Break streak → verify reset
- Load invalid pack → verify copy report works
- Check animation budget respected

---

## Success Criteria

**Must Have:**
- ✅ All 20 achievements implemented and testable
- ✅ Achievement panel accessible from navigation
- ✅ Copy Report button works for validation failures
- ✅ All tests pass (`npm test`, `npm run test:smoke`)
- ✅ No performance regressions
- ✅ Documentation updated

**Nice to Have:**
- Achievement icons (can use emoji placeholders)
- Achievement toast notifications (subtle, respects settings)
- Achievement export in diagnostics

---

## File Touchpoints Summary

**New Files:**
- `src-tauri/src/achievements.rs`
- `src-tauri/src/commands/achievements.rs`
- `src/components/panel/AchievementsPanel.tsx`
- `src/components/shared/AchievementBadge.tsx`
- `src/components/shared/AchievementToast.tsx`
- `src/hooks/useAchievements.ts`
- `src/lib/clipboard.ts`
- `src/lib/__tests__/clipboard.test.ts`
- `src/hooks/__tests__/useAchievements.test.ts`
- `docs/phase1-implementation-plan.md` (this file)

**Modified Files:**
- `src-tauri/src/models.rs` - Add Achievement models
- `src-tauri/src/lib.rs` - Register achievement commands
- `src-tauri/src/commands/pomodoro.rs` - Trigger achievement checks
- `src-tauri/src/progression.rs` - Integrate achievement triggers
- `src/store/types.ts` - Add Achievement types
- `src/components/panel/CustomizationPanel.tsx` - Add Copy Report
- `src/pets/packValidation.ts` - Export formatting utilities
- `src/App.tsx` - Add achievements route
- `CONTRIBUTING.md` - Document achievement system
- `README.md` - Mention achievements feature

---

## Risk Mitigation

**Risk:** Achievement spam disrupts calm UX
**Mitigation:** No popups, tray-only notifications, respects quiet mode

**Risk:** Performance impact from achievement checks
**Mitigation:** Lazy evaluation, check only on relevant events

**Risk:** Achievements don't sync across app restarts
**Mitigation:** Store in app state, persist to disk like other data

**Risk:** Validation report copy fails on some platforms
**Mitigation:** Fallback to manual copy with textarea selection

---

## Next Steps After Phase 1

With Achievement System + Pack Validator UX complete:
- Phase 2: Behavior Composer (accessory personality)
- Phase 3: Advanced Quests (multi-step chains)
- Phase 4: Social features (local achievement sharing)

---

**Ready to implement!** 🚀
