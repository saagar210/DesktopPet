# Phase 8: Behavior Composer + Accessory Personality Layer

**Date:** 2026-02-15
**Goal:** Add subtle, data-driven personality to species and accessories while preserving calm UX

## Overview

This phase enhances pet personality through:
1. **Species Behavior Profiles** - Each species has unique idle patterns, interaction cadence
2. **Accessory Modifiers** - Accessories can subtly modify behavior (e.g., glasses → more scholarly poses)
3. **Behavior Composer** - Runtime system that blends species + accessories + animation budget + context

## Design Principles

✅ **Calm-First**: All behavior respects animation budget and chill modes
✅ **Subtle**: Micro-behaviors, not distracting animations
✅ **Data-Driven**: Configuration in JSON, easy to author/validate
✅ **Precedence**: Budget clamp > focus/quiet chill > accessory > species baseline

## Data Schema

### Species Behavior Profile

```typescript
interface BehaviorProfile {
  // Idle animation variance
  idleMinIntervalMs: number;  // Min time between idle animations
  idleMaxIntervalMs: number;  // Max time between idle animations

  // Interaction cadence
  interactionMinCooldownMs: number;  // Min time between interactions
  interactionMaxCooldownMs: number;  // Max time between interactions

  // Posture preferences (affects animation selection)
  posture: "settle" | "hover" | "curl";

  // Blink pattern
  blinkMinIntervalMs: number;
  blinkMaxIntervalMs: number;
}
```

### Accessory Behavior Modifier

```typescript
interface AccessoryBehaviorModifier {
  // Optional modifier - if present, affects behavior
  behaviorEffect?: {
    idleIntervalMultiplier?: number;     // 0.8 = 20% faster, 1.2 = 20% slower
    interactionCadenceMultiplier?: number;
    postureHint?: "settle" | "hover" | "curl";  // Suggests posture
  };
}
```

### Animation Budget Multipliers

```typescript
const ANIMATION_BUDGET_MULTIPLIERS = {
  low: 2.0,      // 2x slower (calmer)
  medium: 1.0,   // baseline
  high: 0.7,     // 30% faster (more lively)
};
```

## Implementation Steps

### Step 1: Extend Species Pack Schema ✅

Add `behaviorProfile` to species pack format:

```json
{
  "schemaVersion": 1,
  "id": "penguin",
  "name": "Penguin",
  "stageNames": ["Chick", "Juvenile", "Adult"],
  "behaviorProfile": {
    "idleMinIntervalMs": 3000,
    "idleMaxIntervalMs": 8000,
    "interactionMinCooldownMs": 1000,
    "interactionMaxCooldownMs": 3000,
    "posture": "settle",
    "blinkMinIntervalMs": 2000,
    "blinkMaxIntervalMs": 5000
  }
}
```

### Step 2: Extend Accessory Schema ✅

Add optional `behaviorEffect` to shop items:

```json
{
  "id": "scholar_glasses",
  "name": "Scholar Glasses",
  "cost": 150,
  "behaviorEffect": {
    "idleIntervalMultiplier": 1.3,
    "postureHint": "settle"
  }
}
```

### Step 3: Create Behavior Composer ✅

**File:** `src/pets/behaviorComposer.ts`

```typescript
export function composeBehavior(
  speciesProfile: BehaviorProfile,
  equippedAccessories: AccessoryBehaviorModifier[],
  animationBudget: "low" | "medium" | "high",
  isChillMode: boolean
): ComposedBehavior {
  // Apply accessory modifiers
  // Apply animation budget multiplier
  // Apply chill mode overrides
  // Clamp to safe ranges
}
```

### Step 4: Integrate with usePet Hook ✅

Update `usePet.ts` to use composed behavior for timing decisions.

### Step 5: Add Validation ✅

Extend pack validation to check behavior profile constraints.

### Step 6: Add Tests ✅

- Unit tests for behavior composer
- Integration tests for precedence ordering
- Snapshot tests for animation cadence

## File Changes

**New Files:**
- `src/pets/behaviorComposer.ts` - Core composition logic
- `src/pets/__tests__/behaviorComposer.test.ts` - Unit tests

**Modified Files:**
- `src/pets/species.ts` - Type definitions
- `src/pets/packs/penguin.json` - Add behavior profile
- `src/pets/packValidation.ts` - Add behavior profile validation
- `src/hooks/usePet.ts` - Use composed behavior
- `src/store/types.ts` - Add behavior types

## Success Criteria

✅ Low animation budget is noticeably calmer
✅ Accessories subtly affect behavior
✅ Chill modes override all other settings
✅ No animation loops or excessive activity
✅ All validation tests pass
✅ Manual QA confirms calm UX preserved

## Timeline

**Estimated:** 3-4 hours

1. Schema extensions (30min)
2. Behavior composer (1 hour)
3. Integration (1 hour)
4. Testing (1-1.5 hours)
5. Validation + QA (30min)
