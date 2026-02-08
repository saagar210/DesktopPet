import { describe, expect, it } from "vitest";
import { getEnabledSeasonalPacks, getSeasonalPacks } from "../seasonalPacks";

describe("seasonal packs", () => {
  it("loads optional seasonal cosmetic packs", () => {
    const packs = getSeasonalPacks();
    const ids = packs.map((pack) => pack.id);
    expect(ids).toContain("spring-blossom");
    expect(ids).toContain("winter-cozy");
  });

  it("filters enabled packs", () => {
    const enabled = getEnabledSeasonalPacks(["winter-cozy"]);
    expect(enabled).toHaveLength(1);
    expect(enabled[0].id).toBe("winter-cozy");
  });
});
