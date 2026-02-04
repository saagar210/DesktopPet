import { describe, expect, it, vi, beforeEach, afterEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  invokeMaybe,
  invokeOr,
  invokeQuiet,
  listenSafe,
  startDraggingSafe,
} from "../tauri";

describe("tauri safety wrappers", () => {
  let warnSpy: ReturnType<typeof vi.spyOn>;

  beforeEach(() => {
    vi.clearAllMocks();
    warnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});
  });

  afterEach(() => {
    warnSpy.mockRestore();
  });

  it("invokeOr returns result when invoke succeeds", async () => {
    vi.mocked(invoke).mockResolvedValueOnce("ok");

    const result = await invokeOr("cmd", { a: 1 }, "fallback");

    expect(result).toBe("ok");
    expect(invoke).toHaveBeenCalledWith("cmd", { a: 1 });
  });

  it("invokeOr returns fallback when invoke fails", async () => {
    vi.mocked(invoke).mockRejectedValueOnce(new Error("boom"));

    const result = await invokeOr("cmd", undefined, "fallback");

    expect(result).toBe("fallback");
    expect(warnSpy).toHaveBeenCalled();
  });

  it("invokeMaybe returns null when invoke fails", async () => {
    vi.mocked(invoke).mockRejectedValueOnce(new Error("boom"));

    const result = await invokeMaybe("cmd", { x: 1 });

    expect(result).toBeNull();
    expect(warnSpy).toHaveBeenCalled();
  });

  it("invokeQuiet swallows invoke failures", async () => {
    vi.mocked(invoke).mockRejectedValueOnce(new Error("boom"));

    expect(() => invokeQuiet("cmd", { x: 1 })).not.toThrow();
    await Promise.resolve();
    expect(warnSpy).toHaveBeenCalled();
  });

  it("listenSafe returns unlisten when listen succeeds", async () => {
    const unlisten = vi.fn();
    vi.mocked(listen).mockResolvedValueOnce(unlisten);

    const result = await listenSafe("event", vi.fn());

    expect(result).toBe(unlisten);
  });

  it("listenSafe returns noop when listen fails", async () => {
    vi.mocked(listen).mockRejectedValueOnce(new Error("boom"));

    const result = await listenSafe("event", vi.fn());

    expect(typeof result).toBe("function");
    expect(() => result()).not.toThrow();
    expect(warnSpy).toHaveBeenCalled();
  });

  it("startDraggingSafe handles rejected promise", async () => {
    vi.mocked(getCurrentWindow).mockReturnValueOnce({
      startDragging: vi.fn().mockRejectedValueOnce(new Error("drag-failed")),
    } as never);

    expect(() => startDraggingSafe()).not.toThrow();
    await Promise.resolve();
    expect(warnSpy).toHaveBeenCalled();
  });

  it("startDraggingSafe handles non-promise startDragging", () => {
    vi.mocked(getCurrentWindow).mockReturnValueOnce({
      startDragging: vi.fn(),
    } as never);

    expect(() => startDraggingSafe()).not.toThrow();
    expect(warnSpy).toHaveBeenCalled();
  });
});
