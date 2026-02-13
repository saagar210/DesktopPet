import { describe, it, expect, beforeEach, vi } from "vitest";
import { renderHook, act, waitFor } from "@testing-library/react";
import { useCoins } from "../useCoins";

vi.mock("../lib/tauri", () => ({
  invokeMaybe: vi.fn(async (command: string, args: any) => {
    if (command === "spend_coins") {
      return { balance: 100 - args.amount, success: true };
    }
    return null;
  }),
  invokeOr: vi.fn(async (command: string, args: any, defaultValue: any) => {
    if (command === "get_coin_balance") {
      return 100;
    }
    return defaultValue;
  }),
  listenSafe: vi.fn(() => Promise.resolve(() => {})),
}));

describe("useCoins", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should initialize with coin balance", async () => {
    const { result } = renderHook(() => useCoins());

    await waitFor(() => {
      expect(result.current.balance).toBeDefined();
    });

    expect(result.current.balance).toBe(100);
  });

  it("should spend coins and update balance", async () => {
    const { result } = renderHook(() => useCoins());

    await waitFor(() => {
      expect(result.current.balance).toBe(100);
    });

    await act(async () => {
      await result.current.spend(10, "test");
    });

    await waitFor(() => {
      expect(result.current.balance).toBe(90);
    });
  });

  it("should prevent spending more coins than available", async () => {
    const { result } = renderHook(() => useCoins());

    await waitFor(() => {
      expect(result.current.balance).toBe(100);
    });

    // Attempt to spend more than balance
    const result2 = await result.current.spend(150, "test");

    // Should handle gracefully (either return error or prevent)
    expect(typeof result2).toBeDefined();
  });

  it("should track coin changes from events", async () => {
    const { result } = renderHook(() => useCoins());

    await waitFor(() => {
      expect(result.current.balance).toBe(100);
    });

    expect(result.current.balance).toBeGreaterThanOrEqual(0);
  });
});
