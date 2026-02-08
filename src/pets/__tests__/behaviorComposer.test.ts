import { describe, expect, it } from "vitest";
import { composePetBehavior } from "../behaviorComposer";
import { getSpeciesPackById } from "../species";

describe("behavior composer", () => {
  it("clamps lively motion to subtle on low animation budget", () => {
    const result = composePetBehavior({
      species: getSpeciesPackById("corgi"),
      accessories: ["party_hat", "bow_tie"],
      animationState: "idle",
      animationBudget: "low",
      quietModeEnabled: false,
      focusModeEnabled: false,
      contextChilled: false,
    });

    expect(result.motionLevel).toBe(1);
    expect(result.speciesIntensityClass).toBe("species-intensity-subtle");
    expect(result.accessoryClasses).not.toContain("behavior-partyhat");
  });

  it("uses chill clamp precedence over accessory modifiers", () => {
    const result = composePetBehavior({
      species: getSpeciesPackById("corgi"),
      accessories: ["party_hat", "bow_tie", "sunglasses"],
      animationState: "working",
      animationBudget: "high",
      quietModeEnabled: false,
      focusModeEnabled: true,
      contextChilled: false,
    });

    expect(result.shouldChill).toBe(true);
    expect(result.motionLevel).toBe(1);
    expect(result.animationState).toBe("idle");
    expect(result.accessoryClasses).toContain("behavior-sunglasses");
    expect(result.accessoryClasses).not.toContain("behavior-partyhat");
  });

  it("preserves evolving animation while chilled", () => {
    const result = composePetBehavior({
      species: getSpeciesPackById("cat"),
      accessories: [],
      animationState: "evolving",
      animationBudget: "medium",
      quietModeEnabled: true,
      focusModeEnabled: false,
      contextChilled: true,
    });
    expect(result.animationState).toBe("evolving");
    expect(result.postureClass).toBe("species-posture-curl");
  });

  it("quiet mode calms motion without forcing idle state", () => {
    const result = composePetBehavior({
      species: getSpeciesPackById("penguin"),
      accessories: ["party_hat"],
      animationState: "working",
      animationBudget: "medium",
      quietModeEnabled: true,
      focusModeEnabled: false,
      contextChilled: false,
    });
    expect(result.shouldChill).toBe(true);
    expect(result.animationState).toBe("working");
    expect(result.motionLevel).toBeLessThanOrEqual(1);
  });

  it("calculates interaction cooldown from species profile and budget", () => {
    const result = composePetBehavior({
      species: getSpeciesPackById("axolotl"),
      accessories: [],
      animationState: "idle",
      animationBudget: "high",
      quietModeEnabled: false,
      focusModeEnabled: false,
      contextChilled: false,
    });
    expect(result.interactionCooldownMs).toBeLessThan(700);
  });
});
