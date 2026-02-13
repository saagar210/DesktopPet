import { describe, it, expect, beforeEach, vi } from "vitest";
import { renderHook, act, waitFor } from "@testing-library/react";
import { useSettings } from "../useSettings";

vi.mock("../lib/tauri", () => ({
  invokeMaybe: vi.fn(async (command: string, args: any) => {
    if (command === "update_settings") {
      return {
        appTheme: args.patch?.appTheme || "auto",
        notificationsEnabled: args.patch?.notificationsEnabled !== undefined
          ? args.patch.notificationsEnabled
          : true,
        soundEnabled: args.patch?.soundEnabled !== undefined
          ? args.patch.soundEnabled
          : true,
        volume: args.patch?.volume || 50,
      };
    }
    return null;
  }),
  invokeOr: vi.fn(async (command: string, args: any, defaultValue: any) => {
    if (command === "get_settings") {
      return {
        appTheme: "auto",
        notificationsEnabled: true,
        soundEnabled: true,
        volume: 50,
        quietModeEnabled: false,
        timerPreset: "standard",
        animationBudget: "medium",
      };
    }
    return defaultValue;
  }),
  listenSafe: vi.fn(() => Promise.resolve(() => {})),
}));

describe("useSettings", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should initialize with default settings", async () => {
    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.settings).toBeDefined();
    });

    expect(result.current.settings.appTheme).toBe("auto");
    expect(result.current.settings.notificationsEnabled).toBe(true);
  });

  it("should update a single setting", async () => {
    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.settings).toBeDefined();
    });

    const initialTheme = result.current.settings.appTheme;

    await act(async () => {
      await result.current.updateSettings({ appTheme: "dark" });
    });

    await waitFor(() => {
      expect(result.current.settings.appTheme).not.toBe(initialTheme);
    });
  });

  it("should toggle notifications", async () => {
    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.settings).toBeDefined();
    });

    const initialState = result.current.settings.notificationsEnabled;

    await act(async () => {
      await result.current.updateSettings({
        notificationsEnabled: !initialState
      });
    });

    await waitFor(() => {
      expect(result.current.settings.notificationsEnabled).toBe(!initialState);
    });
  });

  it("should update volume within valid range", async () => {
    const { result } = renderHook(() => useSettings());

    await waitFor(() => {
      expect(result.current.settings).toBeDefined();
    });

    await act(async () => {
      await result.current.updateSettings({ volume: 75 });
    });

    await waitFor(() => {
      expect(result.current.settings.volume).toBe(75);
    });
  });
});
