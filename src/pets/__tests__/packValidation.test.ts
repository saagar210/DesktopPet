import { describe, expect, it } from "vitest";
import { validateSpeciesPack } from "../packValidation";
import { getSpeciesPackById } from "../species";

describe("pack validation", () => {
  it("passes built-in packs", () => {
    const penguin = getSpeciesPackById("penguin");
    const result = validateSpeciesPack(penguin);
    expect(result.pass).toBe(true);
    expect(result.checks.every((check) => check.pass)).toBe(true);
    expect(result.checks.every((check) => check.remediation.length > 0)).toBe(true);
  });

  it("fails malformed thresholds", () => {
    const pack = {
      ...getSpeciesPackById("cat"),
      evolutionThresholds: [0, 8, 4] as [number, number, number],
    };
    const result = validateSpeciesPack(pack);
    expect(result.pass).toBe(false);
    const thresholds = result.checks.find((check) => check.id === "thresholds");
    expect(thresholds?.pass).toBe(false);
    expect(thresholds?.remediation).toContain("Set thresholds");
  });
});
