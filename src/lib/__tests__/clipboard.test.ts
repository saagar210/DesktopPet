import { beforeEach, describe, expect, it, vi } from "vitest";
import { copyTextWithFallback } from "../clipboard";

describe("copyTextWithFallback", () => {
  beforeEach(() => {
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: undefined,
    });
  });

  it("copies payload when clipboard API is available", async () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText },
    });

    const result = await copyTextWithFallback("payload", "Report");

    expect(writeText).toHaveBeenCalledWith("payload");
    expect(result).toBe("Report copied to clipboard");
  });

  it("returns payload when clipboard API is unavailable", async () => {
    const result = await copyTextWithFallback("payload", "Report");
    expect(result).toBe("Clipboard unavailable. Report: payload");
  });

  it("returns payload when clipboard write is blocked", async () => {
    const writeText = vi.fn().mockRejectedValue(new Error("blocked"));
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText },
    });

    const result = await copyTextWithFallback("payload", "Report");
    expect(result).toBe("Clipboard write blocked. Report: payload");
  });

  it("supports compact fallback messages without payload", async () => {
    const result = await copyTextWithFallback("payload", "Report", {
      includePayloadOnFallback: false,
    });
    expect(result).toBe("Clipboard unavailable. Report");
  });

  it("truncates long fallback payloads", async () => {
    const result = await copyTextWithFallback("0123456789", "Report", {
      maxFallbackPayloadLength: 5,
    });
    expect(result).toContain("Clipboard unavailable. Report: 01234…");
    expect(result).toContain("truncated 5 chars");
  });
});
