import { describe, it, expect, beforeEach, vi } from "vitest";
import { renderHook, act, waitFor } from "@testing-library/react";
import { usePet } from "../usePet";

// Mock Tauri
vi.mock("../lib/tauri", () => ({
  invokeMaybe: vi.fn(async (command: string) => {
    if (command === "pet_interact") {
      return {
        currentStage: 0,
        mood: "happy",
        energy: 70,
        hunger: 30,
        cleanliness: 70,
        affection: 60,
        speciesId: "penguin",
        totalPomodoros: 0,
        evolutionThresholds: [0, 5, 15],
      };
    }
    if (command === "set_pet_customization") {
      return {
        currentStage: 0,
        skin: "neon",
        scene: "beach",
        speciesId: "penguin",
        totalPomodoros: 0,
        evolutionThresholds: [0, 5, 15],
      };
    }
    if (command === "set_pet_species") {
      return {
        currentStage: 0,
        speciesId: "cat",
        totalPomodoros: 0,
        evolutionThresholds: [0, 5, 15],
      };
    }
    return null;
  }),
  invokeOr: vi.fn(async (command: string, args: any, defaultValue: any) => {
    if (command === "get_pet_state") {
      return {
        currentStage: 0,
        animationState: "idle",
        accessories: [],
        totalPomodoros: 0,
        speciesId: "penguin",
        evolutionThresholds: [0, 5, 15],
        mood: "content",
        energy: 80,
        hunger: 20,
        cleanliness: 80,
        affection: 50,
        personality: "balanced",
        evolutionPath: "undetermined",
        skin: "classic",
        scene: "meadow",
        lastInteraction: null,
        lastCareUpdateAt: new Date().toISOString(),
      };
    }
    return defaultValue;
  }),
  listenSafe: vi.fn(() => Promise.resolve(() => {})),
}));

describe("usePet", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should initialize with default pet state", async () => {
    const { result } = renderHook(() => usePet());

    await waitFor(() => {
      expect(result.current.pet).toBeDefined();
    });

    expect(result.current.pet.speciesId).toBe("penguin");
    expect(result.current.pet.currentStage).toBe(0);
  });

  it("should calculate stage progress correctly", async () => {
    const { result } = renderHook(() => usePet());

    await waitFor(() => {
      expect(result.current.pet).toBeDefined();
    });

    expect(result.current.stageProgress).toBe(0);
    expect(result.current.progressToNext).toBeLessThanOrEqual(1);
  });

  it("should get correct stage name from species pack", async () => {
    const { result } = renderHook(() => usePet());

    await waitFor(() => {
      expect(result.current.stageName).toBeDefined();
    });

    expect(typeof result.current.stageName).toBe("string");
  });

  it("should interact with pet and update state", async () => {
    const { result } = renderHook(() => usePet());

    await waitFor(() => {
      expect(result.current.pet).toBeDefined();
    });

    await act(async () => {
      await result.current.interact("pet");
    });

    await waitFor(() => {
      expect(result.current.pet.mood).toBe("happy");
    });
  });

  it("should set customization (skin and scene)", async () => {
    const { result } = renderHook(() => usePet());

    await waitFor(() => {
      expect(result.current.pet).toBeDefined();
    });

    await act(async () => {
      await result.current.setCustomization("neon", "beach");
    });

    await waitFor(() => {
      expect(result.current.pet.skin).toBe("neon");
      expect(result.current.pet.scene).toBe("beach");
    });
  });

  it("should change pet species", async () => {
    const { result } = renderHook(() => usePet());

    await waitFor(() => {
      expect(result.current.pet).toBeDefined();
    });

    const initialSpecies = result.current.pet.speciesId;

    await act(async () => {
      await result.current.setSpecies("cat", [0, 5, 15]);
    });

    await waitFor(() => {
      expect(result.current.pet.speciesId).not.toBe(initialSpecies);
    });
  });
});
