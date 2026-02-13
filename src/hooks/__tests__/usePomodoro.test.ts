import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { renderHook, act, waitFor } from "@testing-library/react";
import { usePomodoro } from "../usePomodoro";
import * as TauriApi from "@tauri-apps/api/core";
import * as TauriEvent from "@tauri-apps/api/event";

// Mock Tauri modules
vi.mock("@tauri-apps/api/core");
vi.mock("@tauri-apps/api/event");

// Mock tauri.ts wrapper functions
vi.mock("../lib/tauri", () => ({
  invokeMaybe: vi.fn(async (command: string, args: any) => {
    if (command === "get_timer_runtime") {
      return {
        phase: "idle",
        secondsLeft: 1500,
        totalSeconds: 1500,
        paused: false,
        sessionId: null,
        sessionsCompleted: 0,
        preset: "standard",
        lastUpdatedAt: new Date().toISOString(),
      };
    }
    if (command === "start_pomodoro") {
      return { id: "session-123" };
    }
    if (command === "get_settings") {
      return {
        notificationsEnabled: true,
        quietModeEnabled: false,
        soundEnabled: true,
        volume: 50,
      };
    }
    return null;
  }),
  invokeOr: vi.fn(async (command: string, args: any, defaultValue: any) => {
    if (command === "get_timer_runtime") {
      return {
        phase: "idle",
        secondsLeft: 1500,
        totalSeconds: 1500,
        paused: false,
        sessionId: null,
        sessionsCompleted: 0,
        preset: "standard",
        lastUpdatedAt: new Date().toISOString(),
      };
    }
    return defaultValue;
  }),
  invokeQuiet: vi.fn(),
  listenSafe: vi.fn(() => undefined),
}));

describe("usePomodoro", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Clear localStorage
    localStorage.clear();
  });

  afterEach(() => {
    vi.clearAllTimers();
  });

  it("should initialize with idle phase and default preset", async () => {
    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current).toBeDefined();
    });

    // After hydration, should have loaded timer runtime
    expect(result.current.state.phase).toBe("idle");
    expect(result.current.state.preset).toBe("standard");
  });

  it("should start a pomodoro session and transition to work phase", async () => {
    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
    });

    // Start timer
    await act(async () => {
      await result.current.start();
    });

    await waitFor(() => {
      expect(result.current.state.phase).toBe("work");
      expect(result.current.state.sessionId).toBe("session-123");
    });
  });

  it("should countdown seconds during work phase", async () => {
    vi.useFakeTimers();

    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
    });

    const initialSeconds = result.current.state.secondsLeft;

    // Start timer
    await act(async () => {
      await result.current.start();
    });

    await waitFor(() => {
      expect(result.current.state.phase).toBe("work");
    });

    // Advance time by 1 second
    await act(async () => {
      vi.advanceTimersByTime(1000);
    });

    await waitFor(() => {
      expect(result.current.state.secondsLeft).toBe(initialSeconds - 1);
    });

    vi.useRealTimers();
  });

  it("should pause and resume timer", async () => {
    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
    });

    // Start timer
    await act(async () => {
      await result.current.start();
    });

    await waitFor(() => {
      expect(result.current.state.phase).toBe("work");
    });

    // Pause
    act(() => {
      result.current.pause();
    });

    expect(result.current.paused).toBe(true);

    // Resume
    act(() => {
      result.current.resume();
    });

    expect(result.current.paused).toBe(false);
  });

  it("should reset timer to idle state", async () => {
    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
    });

    // Start timer
    await act(async () => {
      await result.current.start();
    });

    await waitFor(() => {
      expect(result.current.state.phase).toBe("work");
    });

    // Reset
    act(() => {
      result.current.reset();
    });

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
      expect(result.current.state.sessionId).toBeNull();
    });
  });

  it("should handle preset changes when idle", async () => {
    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
    });

    const initialPreset = result.current.state.preset;

    // Change preset
    act(() => {
      result.current.setPreset("long");
    });

    await waitFor(() => {
      expect(result.current.state.preset).toBe("long");
      expect(result.current.state.preset).not.toBe(initialPreset);
    });
  });

  it("should not allow preset change during active timer", async () => {
    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
    });

    // Start timer
    await act(async () => {
      await result.current.start();
    });

    await waitFor(() => {
      expect(result.current.state.phase).toBe("work");
    });

    const currentPreset = result.current.state.preset;

    // Try to change preset (should be ignored)
    act(() => {
      result.current.setPreset("long");
    });

    expect(result.current.state.preset).toBe(currentPreset);
  });

  it("should persist timer state to localStorage on change", async () => {
    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
    });

    // Start timer
    await act(async () => {
      await result.current.start();
    });

    await waitFor(() => {
      expect(result.current.state.phase).toBe("work");
    });

    // Verify save_timer_runtime was called
    const { invokeQuiet } = await import("../lib/tauri");
    expect(invokeQuiet).toHaveBeenCalledWith(
      "save_timer_runtime",
      expect.objectContaining({
        runtime: expect.objectContaining({
          phase: "work",
          sessionId: "session-123",
        }),
      })
    );
  });

  it("should increment sessions completed after work phase", async () => {
    vi.useFakeTimers();

    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
    });

    const initialCount = result.current.state.sessionsCompleted;

    // Start timer
    await act(async () => {
      await result.current.start();
    });

    await waitFor(() => {
      expect(result.current.state.phase).toBe("work");
    });

    // Simulate timer completion (advance by full duration)
    const duration = result.current.state.totalSeconds * 1000;
    await act(async () => {
      vi.advanceTimersByTime(duration);
    });

    // After work phase completes, should transition to celebrating
    await waitFor(() => {
      expect(result.current.state.phase).toBe("celebrating");
      expect(result.current.state.sessionsCompleted).toBe(initialCount + 1);
    });

    vi.useRealTimers();
  });

  it("should transition from celebrating to break phase after 3 seconds", async () => {
    vi.useFakeTimers();

    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
    });

    // Manually set to celebrating phase for this test
    await act(async () => {
      await result.current.start();
    });

    await waitFor(() => {
      expect(result.current.state.phase).toBe("work");
    });

    // Fast forward to completion
    const duration = result.current.state.totalSeconds * 1000;
    await act(async () => {
      vi.advanceTimersByTime(duration);
    });

    // Should be in celebrating
    await waitFor(() => {
      expect(result.current.state.phase).toBe("celebrating");
    });

    // Advance 3 seconds more
    await act(async () => {
      vi.advanceTimersByTime(3000);
    });

    // Should transition to break
    await waitFor(() => {
      expect(result.current.state.phase).toBe("break");
    });

    vi.useRealTimers();
  });

  it("should track total sessions completed", async () => {
    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
    });

    expect(result.current.state.sessionsCompleted).toBe(0);
  });

  it("should call complete_pomodoro when work phase ends", async () => {
    vi.useFakeTimers();

    const { result } = renderHook(() => usePomodoro());

    await waitFor(() => {
      expect(result.current.state.phase).toBe("idle");
    });

    // Start timer
    await act(async () => {
      await result.current.start();
    });

    await waitFor(() => {
      expect(result.current.state.phase).toBe("work");
    });

    const sessionId = result.current.state.sessionId;

    // Simulate timer completion
    const duration = result.current.state.totalSeconds * 1000;
    await act(async () => {
      vi.advanceTimersByTime(duration);
    });

    // Verify complete_pomodoro was called
    const { invokeQuiet } = await import("../lib/tauri");
    await waitFor(() => {
      expect(invokeQuiet).toHaveBeenCalledWith("complete_pomodoro", {
        sessionId,
      });
    });

    vi.useRealTimers();
  });
});
